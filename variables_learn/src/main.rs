fn main() {
    let mut age: u32 = 24;
    // const a
    println!("My age is {age}");
    const PI : f64 = 3.14;
    age = 3;
    println!("Update age is {age}, Value of PI : {PI}");
    // const THREE_HOURS_IN_SECONDS : u32 = 3600* age; //wrong cant use non-constant value in a constant variable
    const THREE_HOURS_IN_SECONDS : u32 = 3600 * 3;
    println!("value of THREE_HOURS_IN_SECONDS : {THREE_HOURS_IN_SECONDS}");

    const ONE_HOUR : u32 = 60 * 60;
    let mut time = ONE_HOUR * 30;
    println!("Time is {time}"); 
}
fn magical() -> u32{
    return 10;

}
