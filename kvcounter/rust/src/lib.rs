use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{IncrementRequest, KeyValue, KeyValueSender};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct {{to_pascal_case project-name}}Actor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for {{to_pascal_case project-name}}Actor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // Format key with a friendly separator
        let key = format!("counter:{}", req.path.replace('/', ":"));

        let (body, status_code) = match increment_counter(ctx, key).await {
            Ok(v) => (json!({ "counter": v }).to_string(), 200),
            // Ensure we properly return database errors as server errors
            Err(e) => (json!({ "error": e.to_string() }).to_string(), 500),
        };

        HttpResponse::json(body, status_code)
    }
}

/// Increment the key in the keyvalue store by 1, returning the new value
async fn increment_counter(ctx: &Context, key: String) -> RpcResult<i32> {
    Ok(KeyValueSender::new()
        .increment(ctx, &IncrementRequest { key, value: 1 })
        .await?)
}
