pub fn example() {
    println!("----- if/else -----");
    let n = 40;
    if n < 0 {
        println!("{}은 음수입니다.", n);
    } else if n > 0 {
        println!("{}은 양수입니다.", n);
    } else {
        println!("{}은 0 입니다.", n);
    }
    // {} 은 표현식(expression)이라 반환값으로 사용 가능(타입이 같아야 함!)
    let cal_n = if n < 10 && n > -10 {
        println!("-> 한자리 수이므로 10을 곱해봅니다.");
        10 * n
    } else {
        println!("-> 큰 수이므로 반으로 나눠 봅니다.");
        n / 2
    };
    println!("=> {0} -> {1}", n, cal_n);
}

#[test]
pub fn let_example() {
    // 패턴 매칭 중 일부만 필요할때.(전체는 match)
    println!("----- if let -----");

    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
    println!("----- from binding -----");
    fn some_number() -> Option<u32> {
        Some(42)
    }
    if let Some(n @ 42) = some_number() {
        println!("The Answer is {}!", n); //값이 42인당걸 n에 할당
    }
    // match some_number() {
    //     Some(n) => println!("Not interesting... {}", n),  // 그 외 값
    //     None => println!("return None(Null)"),            // options.None
    //     _ => (),                                          //default 여기서는 None과 같음
    // }
}
