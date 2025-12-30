fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("changed value: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
