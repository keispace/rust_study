// https://keispace.github.io/comprehensive-rust-kr/exercises/day-1/iterators-and-ownership.html?highlight=iter#iterator

pub fn example() {
    println!("----- for -----");
    let mut v = vec![10, 20, 30];
    for x in v.iter_mut() {
        *x += 1;
        println!("x: {x}");
    }

    println!("----- for with range-----");
    for x in 0..11 {
        if x % 2 == 0 {
            continue;
        }
        println!("x: {x}");
    }
}
