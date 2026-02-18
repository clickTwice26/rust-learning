fn main() {
    let mut age: i32 = 24;
    println!("My age is {age}");
    const PI : f64 = 3.14;
    age = 3;
    println!("Update age is {age}, Value of PI : {PI}");
    // const THREE_HOURS_IN_SECONDS : u32 = 3600* age; //wrong cant use non-constant value in a constant variable

}
