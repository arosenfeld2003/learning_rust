fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // immutable
    let r2 = &s;      // immutable
    let r3 = &mut s;  // mutable — breaks the immutable promise

    println!("{}, {}, {}", r1, r2, r3);
}
