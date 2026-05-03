fn main() {

    let mut x = 5;
    let y = x;

    let hello: String = String::from("I am hello");
    println!("hello is {hello}");
    let world = hello.clone();
    println!("World is {hello}");
    println!("World is {world}");
    // println!("X is {x} and y is {y}");
}

fn add(x : i32) -> i32{
    x + 10
}
