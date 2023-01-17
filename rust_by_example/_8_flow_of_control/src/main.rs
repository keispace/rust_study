#![allow(dead_code, unreachable_code, unused_labels, unreachable_patterns)]

mod controls;

fn main() {
    // controls::if_control::example();
    // controls::for_control::example();
    // controls::while_control::example();
    // controls::loop_control::example();
    // controls::if_control::let_example();
    // controls::while_control::let_example();
    // controls::match_control::example();
    // controls::match_control::destructure();
    controls::match_control::guards();
    // controls::match_control::binding();
}
