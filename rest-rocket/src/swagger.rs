use crate::route;
use route::{body, forward, hello, ignored, r#async};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello::get_hello,
        hello::get_hello_name,
        hello::get_world,
        body::json_user
    ),
    components(schemas(body::User))
)]
pub struct ApiDoc;
