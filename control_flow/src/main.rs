fn main(){
    is_even(3); 
    loop{ // forever loop
        break;
        println!("Hello from main!");
    }
    let mut num: i32 = 0;
    loop {
        println!("Value  of number is {num}");
        if num == 10{
            println!("Loop printed for {} times", num+1);
            break;
        }
        num+=1;
    }
}
// fn main() {
//     // println!("Hello, world!");
//     let y = is_even(5);
//     if y {
//         println!("Function Returned {y}");
//     }
//     let z: i32 = if y == false {5} else {10};
//     println!("The value of z is {z}");
// }
fn is_even(x: i32) -> bool {
    if x % 2 == 0 {
        return true;
    }
    false
}
