fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;  // second mutable reference to same value

    println!("{}, {}", r1, r2);
}
