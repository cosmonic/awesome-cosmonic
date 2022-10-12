use chatlog::*;
use outbound::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::*;
use wasmcloud_interface_logging::debug;
use wasmcloud_interface_numbergen::*;

#[allow(dead_code)]
mod chatlog;

#[allow(dead_code)]
mod outbound;

const CHANNEL_NAME: &str = "http";
const API_ACTOR: &str = "mcchat/api";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer, Outbound)]
struct HttpChannelActor {}

#[async_trait]
impl Outbound for HttpChannelActor {
    async fn publish_message(&self, ctx: &Context, arg: &OutboundMessage) -> RpcResult<bool> {
        // This is absorbed silently because the HTTP channel does not currently expose
        // any kind of realtime subscription. Perhaps in the future a websocket subscription
        // could be used?
        Ok(true)
    }
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for HttpChannelActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        match (req.method.as_ref(), segments.as_slice()) {
            ("POST", ["messages"]) => write_message(ctx, deser(&req.body)?).await,
            ("GET", ["messages"]) => get_messages(ctx).await,
            (m, p) => {
                debug!("unexpected method and path: {} - {:?}", m, p);
                Ok(HttpResponse::not_found())
            },
        }
    }
}

async fn write_message(ctx: &Context, im: IncomingMessage) -> RpcResult<HttpResponse> {
    // Go to the gateway, not directly to the chat logger
    let logger = ChatlogSender::to_actor(API_ACTOR);
    let numgen = NumberGenSender::new();
    let guid = numgen.generate_guid(ctx).await.unwrap_or("n/a".to_string());

    logger
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
        .map(|r| r.into())
}

async fn get_messages(ctx: &Context) -> RpcResult<HttpResponse> {
    let logger = ChatlogSender::to_actor(API_ACTOR);
    match logger.get_messages(ctx).await {
        Ok(r) => HttpResponse::json(r, 200),
        Err(e) => Ok(HttpResponse::internal_server_error(format!("{}", e))),
    }
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
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

impl From<chatlog::WriteMessageResponse> for HttpResponse {
    fn from(source: chatlog::WriteMessageResponse) -> Self {
        if source.accepted {
            HttpResponse::default()
        } else {
            HttpResponse::internal_server_error(source.reason.unwrap_or_else(|| "".to_string()))
        }
    }
}
