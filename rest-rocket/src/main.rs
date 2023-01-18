#[macro_use]
extern crate rocket;

mod route;
mod swagger;

use rocket::fs::FileServer;
use route::body;
use route::forward;
use route::hello;
use route::ignored;
use route::r#async;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// #[derive(OpenApi)]
// #[openapi(paths(hello::get_hello))]
// pub struct ApiDoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from("static/")) //static serve
        .mount("/async", routes![r#async::delay, r#async::blocking_task])
        .mount("/ignored", routes![ignored::everything, ignored::foo_bar])
        .mount(
            "/hello",
            routes![hello::get_hello, hello::get_world, hello::get_hello_name],
        )
        .mount(
            "/forward",
            routes![forward::user, forward::user_int, forward::user_str],
        )
        .mount("/body", routes![body::json_user])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-doc/openapi.json", swagger::ApiDoc::openapi()),
        )
}
