use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1", api = api::MyApi)
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = axum::Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1/", api::router());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
