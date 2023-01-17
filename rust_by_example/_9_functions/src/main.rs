#![allow(
    dead_code,
    unreachable_code,
    unused_labels,
    unreachable_patterns,
    unused_variables
)]

mod closure;
mod function;
mod method;

fn main() {
    // function::fizzbuzz_to_func(13);
    // method::example();
    // closure::closures();
    closure::capturing();
    //diverging function
    // fn afn() -> ! {
    //     panic!("panic!")
    // }
    // afn();
}
