use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(list_sections, create_section), components(schemas(Section)))]
pub struct MyApi;

pub fn router() -> Router {
    Router::new().route("/sections", get(list_sections).post(create_section))
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct Section {
    #[schema(example = "section1")]
    name: String,
}

/// List all sections
///
/// List all sections available
#[utoipa::path(
    get,
    path = "/sections",
    responses(
        (status = 200, description = "List all sections successfully", body = [()])
    )
)]
async fn list_sections() -> Json<Vec<Section>> {
    Json::<Vec<Section>>(vec![])
}

/// Create new Section
///
/// Tries to create a new Section item to in-memory storage or fails with 409 conflict if already exists.
#[utoipa::path(
    post,
    path = "/sections",
    request_body = Section,
    responses(
        (status = 201, description = "Section item created successfully", body = Section),
        // (status = 409, description = "Section already exists", body = TodoError)
    )
)]
async fn create_section() -> Json<Section> {
    todo!()
}
