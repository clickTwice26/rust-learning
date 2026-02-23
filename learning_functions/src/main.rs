// Learning functions


fn main() {
    let greeting : &str = "hello";
    // rust compiler will run only the main function
    println!("{greeting}, world!");
    another_function(5, true); // 5 is the argument
}
fn another_function(x: i32, y : bool) {  //x is parameter

    println!("Hello from another_function(), {x} {y}");
}