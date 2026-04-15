fn change(s: &String) {
    s.push_str(" world");  // attempt to modify borrowed value
}

fn main() {
    let s1 = String::from("hello");
    change(&s1);
}
