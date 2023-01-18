use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
// #[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct User<'r> {
    name: &'r str,
    #[serde(alias = "isAdmin")]
    is_admin: bool,
}

#[utoipa::path( 
    context_path="/body", 
    request_body = User,
    responses( 
        (status = 200, description = "describe user")
    )
)]
#[post("/json", data = "<user>")]
pub fn json_user(user: Json<User<'_>>) -> String {
    format!("{:?}", user)
}

#[derive(FromForm, Debug)]
struct Task<'r> {
    complete: bool,
    r#type: &'r str,
}

#[post("/form", data = "<task>")]
fn form(task: Form<Task<'_>>) -> String {
    format!("{:?}", task)
}

#[derive(FromForm)]
struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/upload", data = "<upload>")]
fn upload_form(upload: Form<Upload<'_>>) { /* .. */
}
