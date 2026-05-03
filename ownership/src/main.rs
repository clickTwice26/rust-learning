fn main() {

    let mut s  = String::from("Hello");
    s = s.to_uppercase();
    s.push_str(" World");
    println!("{s}");
}
