pub mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
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
