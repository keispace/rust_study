fn main() {
    // _3_1_variable_and_mutable();
    // _3_2_data_types();
    // _3_3_fn(1);
    // 3.4 comment. // is comment.
    _3_5_control();
}

fn _3_1_variable_and_mutable() {
    let a = 5;
    println!("a is {}", a);
    // a = 6; // compile failed!!
    let a = "shadowing like variable overwriting";
    println!("this is shadowing a : {}", a);
    // 변경후에도 여전히 immutable.

    let mut b = 7;
    println!("b is {}", b);
    b = 8;
    println!("b is {}", b);
    // mut 은 타입변경 안됨.

    const MAX_POINTS: u32 = 100_000;
    // const must upper case
    // const cannot set mut
    println!("MAX_POINTS is {}", MAX_POINTS);
}

fn _3_2_data_types() {
    let int8: i8 = i8::MAX;
    let uint8: u8 = u8::MAX;
    let int16: i16 = i16::MAX;
    let uint32: u32 = u32::MAX;
    let int64: i64 = i64::MAX;
    let int_arch: isize = isize::MAX;
    // 정수형 기본은 i32
    println!(
        "{:?},{:?},{:?},{:?},{:?},{:?}",
        int8, uint8, int16, uint32, int64, int_arch
    );

    let decimal = 100_000;
    println!("정수 리터럴엔 , 처럼 _ 허용됨. {}", decimal);
    println!(
        "hex, octal binary, byte(u8) 가능\n{}, {}, {}, {}",
        0xff, 0o77, 0b1011, b'A'
    );
    let _float64 = 3.0; // f32, f64(default)

    let sum = 1 + 2;
    let difference = 95.1 - 2.0;
    // let panic!! = 1 / 0; // panic!
    let product = 9 * 20;
    let division = 1 / 2;
    let remainder = 5 / 2;
    println!(
        "{} {} {} {} {}",
        sum, difference, product, division, remainder
    );

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z = '가';
    let heart_eyed_cat = '😻';
    println!("{} {} {} ", _c, _z, heart_eyed_cat);

    let this_is_tuple = (1, '가', 5.8);
    let (_x, _y, _z) = this_is_tuple;

    println!("{}, {}", this_is_tuple.2, _y);

    let array_is_stack = [1, 2, 3, 4, 5];
    println!("{}", array_is_stack[0]);
}

fn _3_3_fn(x: i32) {
    println!("The value of x is: {}", x);
    let _statements = 5; // 구문은 반환없음. ;로 보면 됨.
                         // let x = (let y = 6); //x에 할당할게 없어서 panic
    let _expressions = {
        //{}블록은 표현식.
        let x = 1;
        x + 1 //; 생략하면 return
    };
    println!("{}", _expressions);
    println!("{}", _3_3_sub(3))
}
fn _3_3_sub(x: i32) -> i32 {
    x + 1
}

fn _3_5_control() {
    let number = 6;
    // if condition is only bool
    if number % 2 == 0 {
        println!("2배수")
    } else if number % 3 == 0 {
        println!("3배수")
    } else {
        println!("false")
    }

    let condition = true;
    let mut number = if condition { 5 } else { 6 }; // 같은 타입이어야 함. 정적타입언어다(...)
    println!("The value of number is: {}", number);

    loop {
        println!("again!");
        number = number - 1;
        if number == 0 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
