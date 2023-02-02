// https://keispace.github.io/comprehensive-rust-kr/control-flow/match-expressions.html?highlight=match#match-expressions
pub fn example() {
    println!("----- match -----");
    let number = 19;
    println!("Tell me about {}", number);
    // 위에서 아래로 진행됨.
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),  // 13 ~ 19 까지 슬라이스
        _ => println!("Ain't special"), // default
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
        // _ => 2
    };

    println!("{} -> {}", boolean, binary);
}

#[allow(dead_code, unreachable_code, unused_labels, unreachable_patterns)]
pub fn destructure() {
    println!("----- destructure -----");

    println!("----- Array and tuple (same) -----");

    // array와 tuple 둘다 슬라이스 가능
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    println!("----- enums -----");
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        // Color::RGB(.. , b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        _ => (),
    }

    println!("----- pointer/ref -----");
    let immutable_value = 5;
    let mut mutable_value = 6;
    let reference = &3;
    let ref reference2 = 4;

    match reference {
        &val => println!("참조값은 역구조화해서 접근: {:?}", val),
    }
    match *reference2 {
        val => println!("미리 역참조 선언해서 접근: {:?}", val),
    }
    match immutable_value {
        ref r => println!("불변 변수 값에 대한 참조 접근: {:?}", r),
    }
    match mutable_value {
        ref mut m => {
            *m += 10;
            println!("가변 변수도 불변변수랑 비슷함.: {:?}", m);
        }
    }

    println!("----- structs -----");
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}

pub fn guards() {
    println!("----- guards -----");
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }
    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
        _ => println!("아무것도 없음"),
    }
}

pub fn binding() {
    println!("----- binding -----");
    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer is {}!", n), //값이 42인당걸 n에 할당
        Some(n) => println!("Not interesting... {}", n),  // 그 외 값
        None => println!("return None(Null)"),            // options.None
        _ => (),                                          //default 여기서는 None과 같음
    }
}
