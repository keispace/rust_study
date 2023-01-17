#![allow(
    dead_code,
    unreachable_code,
    unused_labels,
    unreachable_patterns,
    unused_variables
)]

// extern crate _11_crates_lib; // ~2015전버전

use _11_crates_lib::add;

/// 이건 문서화 주석
fn main() {
    let a = add(1, 2);
    println!("Hello, world!: {}", a);
}

// 파일 rlib 사용하기
// 터미널에서 컴파일할때 링크
// 그냥 폴더째로 링크 거는 거 추천.(Cargo.toml)
// rustc main.rs --extern _11_crates_lib=lib_11_crates_lib.rlib
