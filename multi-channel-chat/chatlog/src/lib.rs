use chatlog::*;
use wasmbus_rpc::actor::prelude::*;

#[allow(dead_code)]
mod chatlog;

mod store;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Chatlog)]
struct ChatlogActor {}

/// Implementation of Chat Log actor trait methods
#[async_trait]
impl Chatlog for ChatlogActor {
    async fn write_message(
        &self,
        ctx: &Context,
        arg: &CanonicalChatMessage,
    ) -> RpcResult<WriteMessageResponse> {
        Ok(match store::write_message(ctx, arg).await {
            Ok(_) => WriteMessageResponse {
                accepted: true,
                reason: None,
            },
            Err(e) => WriteMessageResponse {
                accepted: false,
                reason: Some(format!("{}", e)),
            },
        })
    }

    async fn get_messages(&self, ctx: &Context) -> RpcResult<MessagesList> {
        Ok(match store::get_messages(ctx).await {
            Ok(v) => v,
            Err(_) => vec![],
        })
    }
}
