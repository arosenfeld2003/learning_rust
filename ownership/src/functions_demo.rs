fn takes_ownership(s: String) {
    println!("{}", s);
}   // s dropped here, heap memory freed

fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);        // s1 moves into function
    println!("{}", s1);         // error: s1 no longer valid
}
