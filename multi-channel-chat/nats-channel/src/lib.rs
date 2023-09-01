use chatlog::*;
use outbound::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_messaging::*;
use wasmcloud_interface_numbergen::*;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHANNEL_NAME: &str = "nats";
const API_ACTOR: &str = "mcchat/api";
const RTCHAT_TOPIC: &str = "rtchat.messages";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber, Outbound)]
struct NatsChannelActor {}

#[async_trait]
impl Outbound for NatsChannelActor {
    async fn publish_message(&self, ctx: &Context, arg: &OutboundMessage) -> RpcResult<bool> {
        let pubber = MessagingSender::new();
        let res = pubber
            .publish(
                ctx,
                &PubMessage {
                    subject: RTCHAT_TOPIC.to_string(),
                    reply_to: None,
                    body: serde_json::to_vec(arg).unwrap(),
                },
            )
            .await;
        Ok(res.is_ok())
    }
}

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl MessageSubscriber for NatsChannelActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let reply_topic = msg.reply_to.as_deref().unwrap_or("dedletter");

        match serde_json::from_slice::<IncomingMessage>(&msg.body) {
            Ok(im) => {
                let logger = ChatlogSender::to_actor(API_ACTOR);
                let numgen = NumberGenSender::new();
                let guid = numgen.generate_guid(ctx).await.unwrap_or("n/a".to_string());
                match logger
                    .write_message(
                        ctx,
                        &CanonicalChatMessage {
                            body: im.body,
                            channel_name: CHANNEL_NAME.to_string(),
                            id: guid,
                            source_user: im.user_name,
                        },
                    )
                    .await
                {
                    Ok(resp) => {
                        let ack = Ack {
                            accepted: resp.accepted,
                            reason: resp.reason,
                        };
                        let _ = pub_ack(ctx, reply_topic, ack).await;
                    }
                    Err(e) => {
                        let _ = pub_fail(ctx, reply_topic, e.to_string()).await;
                    }
                }
            }
            Err(_) => {
                let _ = pub_fail(ctx, reply_topic, "Bad Request".to_string()).await;
            }
        }
        Ok(())
    }
}

async fn pub_ack(ctx: &Context, topic: &str, ack: Ack) {
    let publisher = MessagingSender::new();
    let _ = publisher
        .publish(
            ctx,
            &PubMessage {
                subject: topic.to_string(),
                reply_to: None,
                body: serde_json::to_vec(&ack).unwrap(),
            },
        )
        .await;
}

async fn pub_fail(ctx: &Context, topic: &str, msg: String) {
    pub_ack(
        ctx,
        topic,
        Ack {
            accepted: false,
            reason: Some(msg),
        },
    )
    .await
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IncomingMessage {
    user_name: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ack {
    accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}
