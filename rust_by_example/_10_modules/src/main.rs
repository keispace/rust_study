#![allow(
    dead_code,
    unreachable_code,
    unused_labels,
    unreachable_patterns,
    unused_variables
)]

mod visibility; // visibility/mod.rs or ./visibility.rs
use visibility::structs::my;
fn main() {
    visibility::public_mod::public_fn();
    visibility::public_mod::indirect_fn();
    let open_box = my::OpenBox { contents: "a" };
    // let closed_box = visibility::structs::my::ClosedBox { contents: "b" };
    let closed_new_box = my::ClosedBox::new("C");
    // println!("{}", closed_new_box.contents);
    let cnb_content = closed_new_box.get_contents().to_owned().to_owned();
    println!("{}", cnb_content);
}
