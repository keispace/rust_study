#![allow(dead_code, unreachable_code, unused_labels, unreachable_patterns)]

mod for_control;
mod if_control;
mod loop_control;
mod match_control;
mod while_control;

fn main() {
    if_control::example();
    if_control::let_example();
    loop_control::example();
    while_control::example();
    while_control::let_example();
    for_control::example();
    match_control::example();
    match_control::destructure();
    match_control::guards();
    match_control::binding();
}
