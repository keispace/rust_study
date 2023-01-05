pub mod structs;

pub mod public_mod {
    fn private_fn() {
        println!("외부 호출 불가")
    }
    pub fn public_fn() {
        println!("외부 호출 가능")
    }
    pub fn indirect_fn() {
        println!("private 간접호출");
        private_fn();
    }
    mod nested_private_mod {
        fn nested_private_fn() {
            println!("외부 호출 불가")
        }
    }
}

mod private_mod {
    fn private_fn() {
        println!("외부 호출 불가")
    }
}
