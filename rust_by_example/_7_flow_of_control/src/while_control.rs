pub fn example() {
    println!("----- while -----");
    let mut n = 1;
    while n < 11 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
pub fn let_example() {
    // 패턴 매칭으로 사용할 경우.
    println!("----- while let -----");
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
