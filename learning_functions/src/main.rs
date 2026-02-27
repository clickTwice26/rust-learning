// Learning functions


fn main() {
    let greeting : &str = "hello";
    // rust compiler will run only the main function
    println!("{greeting}, world!");
    another_function(5, true); // 5 is the argument
}
fn another_function(x: i32, y : bool) {  
    // x is parameter
    // must have to annotate the types of the parameter

    println!("Hello from another_function(), {x} {y}");
    if y == true {
        println!("Yes it is true");
    }
    add_function(1, 3);
}
fn add_function(x : i32, y : i32){
    // Statement vs Expression
    let add : i32 = x + y;

    // let add : i32 = x + y; this is a statement; statement do not return values, expressions are the parts of statement
    // calling a function is an expression and macro also expression
    // but function
    // x + y this is an expression
    println!("Total : {x} + {y} = {add}");
    println!("{:?}", add);


}