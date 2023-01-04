pub fn example() {
    println!("----- loop -----");
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("3!");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("end!");
            break;
        }
    }
    println!("----- nested loop -----");
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop");
            break 'outer;
            println!("여기는 실행되지 않습니다.");
        }
        println!("여기는 실행되지 않습니다.");
    }
    println!("중첩 Loop 끝");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("블록({{}})은 표현식입니다 == 값 반환 가능 : {}", result);
}
