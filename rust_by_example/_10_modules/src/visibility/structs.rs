pub mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T, // private. cannot create `let c_box = ClosedBox {contents:""} `
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
        pub fn get_contents(&self) -> &T {
            &self.contents
        }
    }
}
