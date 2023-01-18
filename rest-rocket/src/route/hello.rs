#[utoipa::path( 
    context_path="/hello", 
    responses( 
        (status = 200, description = "say hello")
    )
)]
#[get("/")]
pub fn get_hello() -> &'static str {
    "Hello, world!"
}

#[utoipa::path( 
    context_path="/hello", 
    params(
        ("name" = str, Path, description = "name"),
    ),
    responses( 
        (status = 200, description = "say hello <name>")
    )
)]
#[get("/<name>")]
pub fn get_hello_name(name: &str) -> String {
    format!("Hello, {}", name)
}

#[utoipa::path( 
    context_path="/hello", 
    responses( 
        (status = 200, description = "say world")
    )
    )]
#[get("/world")]
pub fn get_world() -> &'static str {
    "this is world"
}
