use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct {{to_pascal_case project-name}}Actor {}

/// Implementation of the HTTP server capability
#[async_trait]
impl HttpServer for {{to_pascal_case project-name}}Actor {
    async fn handle_request(&self, _ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        HttpResponse::ok("Hello, World!")
    }
}

