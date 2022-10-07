use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{
    HttpClient, HttpClientSender, HttpRequest as HttpClientRequest,
};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::random_in_range;

#[derive(serde::Deserialize)]
// XKCD comic metadata fields
struct XkcdComic {
    title: String,
    img: String,
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XkcdgeneratorActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for XkcdgeneratorActor {
    async fn handle_request(&self, ctx: &Context, _req: &HttpRequest) -> RpcResult<HttpResponse> {
        // Generate a comic number, between the first and most recent comic
        let random_num = random_in_range(1, 2680).await?;
        // Create request URL where XKCD stores JSON metadata about comics
        let xkcd_url = format!("https://xkcd.com/{}/info.0.json", random_num);

        let response = HttpClientSender::new()
            .request(ctx, &HttpClientRequest::get(&xkcd_url))
            .await?;

        // Deserialize JSON to retrieve comic title and img URL
        let comic: XkcdComic = serde_json::from_slice(&response.body).map_err(|e| {
            RpcError::ActorHandler(format!("Failed to deserialize comic request: {}", e))
        })?;

        // Format HTTP response body as an HTML string
        let body = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Your XKCD random comic</title>
        </head>
        <body>
            <h1>{}</h1>
            <img src="{}"/>
        </body>
        </html>
            "#,
            comic.title, comic.img
        );

        Ok(HttpResponse {
            body: body.as_bytes().to_vec(),
            ..Default::default()
        })
    }
}
