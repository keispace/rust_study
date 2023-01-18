#[get("/foo/<_>/bar")]
pub fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/<_..>")] //multi segments pattern
pub fn everything() -> &'static str {
    "Hey, you're here."
}
