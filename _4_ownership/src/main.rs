fn main() {
    // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
    _4_1_ownership_by_string();
    let s = "hello"; // s는 이 지점부터 유효합니다.
                     // s를 가지고 뭔가 합니다.
} // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.

fn _4_1_ownership_by_string() {
    let strMu = "hello"; // immutable 메모리 하드코딩
    let mut s = String::from("hello"); // heap.
    s.push_str(", world");
    println!("{}", s);

    // rust 는 ownership을 기준으로 메모리 관리.
    // 스코프 밖으로 나가면 메모리 반납됨.

    let x = 5;
    let y = x; //copy.
    println!("{}", x); // ok,

    let s1 = String::from("hello");
    let s2 = s1; // 소유권이 이동(move)됨.  shallow copy와 유사하지만 s1는 무효화됨.
    let s3 = s2.clone(); // deep copy. 메모리가 복사됨

    // println!("{}", s1); // panic! value borrowed here after move
    println!("{}", s2);
    println!("{}", s3);

    let s = String::from("hello"); // s가 스코프 안으로 들어왔습니다.
    takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...
                        // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5; // x가 스코프 안으로 들어왔습니다.

    makes_copy(x); // x가 함수 안으로 이동했습니다만,
                   // i32는 Copy가 되므로, x를 이후에 계속
                   // 사용해도 됩니다.
} //여기에 오면 내부 변수들이 전부 drop됨.

fn takes_ownership(some_string: String) {
    // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) {
    // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

fn test2() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    let r3 = &mut s;
    // println!("{} {} {}", r1, r2, r3);
    println!("{}", r3);
}
