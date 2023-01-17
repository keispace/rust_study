fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn basic() {
    // statement(구문) -> 그 자체.
    let x = 5; // 변수 바인딩

    // expression + ';'
    x;
    x + 1;

    // expression(표현식) -> 반환값 있음
    {}
    let z = {
        5 // expression
    };
    let z = {
        5;
    };
}
