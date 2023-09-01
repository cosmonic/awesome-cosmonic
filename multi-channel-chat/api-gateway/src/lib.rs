use chatlog::*;
use outbound::{Outbound, OutboundMessage, OutboundSender};
use wasmbus_rpc::actor::prelude::*;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHATLOG_ACTOR: &str = "mcchat/chatlog";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct ApiGatewayActor {}

const KNOWN_CHANNEL_NAMES: &[&str] = &["http", "nats"];

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for ApiGatewayActor {
    async fn write_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<WriteMessageResponse> {
        let chatlog = ChatlogSender::to_actor(CHATLOG_ACTOR);

        let res = chatlog.write_message(ctx, arg).await;

        // Select every channel that isn't the one that just called
        let mut targets = KNOWN_CHANNEL_NAMES.to_vec();
        targets.retain(|c| *c != arg.channel_name);

        for channel in targets {
            let outbound = OutboundSender::to_actor(&format!("mcchat/{}", channel));
            let _ = outbound
                .publish_message(
                    ctx,
                    &OutboundMessage {
                        body: arg.body.to_string(),
                        source_channel: arg.channel_name.to_string(),
                        source_user: arg.source_user.to_string(),
                    },
                )
                .await;
        }
        res
    }

    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        let chatlog = ChatlogSender::to_actor(CHATLOG_ACTOR);

        chatlog.get_messages(ctx).await
    }
}
