// https://doc.rust-lang.org/std/fmt/

use std::fmt;

fn main() {
    // 주석
    /*
    블록 주석
     */
    /// cargo test 시에는 컴파일 되는 문서 주석
    println!("Hello, world!");

    // rust formatted print
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    let a = 1;
    println!("{a}");

    // :타입 지정
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    /* 커스텀 출력 방식 구현 */
    println!("----- {:^5} -----", "커스텀 출력 impl");

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    /*  Activity */
    #[derive(Debug)]
    struct Complex {
        real: f32,
        imag: f32,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", &self.real, &self.imag)
        }
    }
    let com = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", com);
    println!("Debug: {:?}", com);

    println!("----- {:^5} -----", "List");

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let List(ref vec) = *self;
            // try! (deprecated 됨) -> 연산자
            // try!(write!(f, "["));
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }
    println!("{}", List(vec![1, 2, 3]));

    println!("----- {:^5} -----", "formatting");
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // println!("{:?}", *color);
        println!(
            "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
            color.red, color.green, color.blue,
        )
    }
}
