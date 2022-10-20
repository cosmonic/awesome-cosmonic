use crate::chatlog::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_logging::info;

const MESSAGE_LIST_KEY: &str = "chatlog:messages";

pub(crate) async fn write_message(ctx: &Context, msg: &CanonicalChatMessage) -> RpcResult<()> {
    let raw = serde_json::to_string(msg).map_err(|e| RpcError::Ser(format!("{}", e)))?;

    let kv = KeyValueSender::new();
    info!("Storing chat log message '{}'", raw);
    let _ = kv
        .list_add(
            ctx,
            &ListAddRequest {
                list_name: MESSAGE_LIST_KEY.to_string(),
                value: raw,
            },
        )
        .await?;

    Ok(())
}

pub(crate) async fn get_messages(ctx: &Context) -> RpcResult<Vec<CanonicalChatMessage>> {
    let kv = KeyValueSender::new();

    kv.list_range(
        ctx,
        &ListRangeRequest {
            list_name: MESSAGE_LIST_KEY.to_string(),
            start: 0,
            stop: 9999,
        },
    )
    .await
    .map(|res| {
        res.iter()
            .filter_map(|s| match serde_json::from_str(s.as_str()) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<Vec<CanonicalChatMessage>>()
    })
}
