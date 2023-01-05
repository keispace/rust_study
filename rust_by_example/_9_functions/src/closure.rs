use std::mem;

pub fn closures() {
    // closures
    fn is_function(i: i32) -> i32 {
        i + 1
    }
    let is_closure = |i: i32| -> i32 { i + 1 };

    //값 캡쳐링
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("There're {} elements in vec", haystack.len()); // <-- move면 에러

    /*
    함수 제너릭 타입
    fn: &T
    fnMut: mut &T
    fnOnce: T
    */
    fn apply<F: FnOnce()>(f: F)
    // where
    //     F: FnOnce(),
    {
        f();
    }
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let x = 8;
    let diary = || {
        println!("I said {}.", greeting); // Fn
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell); // FnMut
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell); // FnOnce
        println!("x: {}", x); // Fn, FnMut, FnOnce 필요.(캡처 -> 제너릭에서 익명타입으로 캡쳐)
    };
    apply(diary);
    //함수/클로저를 인자로 받는경우 fn을 만족하는 함수여야함.
    // r-value 클로저는 제너릭 미지원
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
