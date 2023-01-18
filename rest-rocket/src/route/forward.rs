#[get("/<id>")]
pub fn user(id: usize) -> String {
    format!("user {}", id)
}

#[get("/<id>", rank = 2)]
pub fn user_int(id: isize) -> String {
    format!("user_int {}", id)
}

#[get("/<id>", rank = 3)]
pub fn user_str(id: &str) -> String {
    format!("user_str {}", id)
}
