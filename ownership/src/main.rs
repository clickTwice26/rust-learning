fn main() {

    let num = 10;
    let result = add(num);
    let name = String::from("Shagato");
    takes_ownership(name);
    // print!("Vale of name is {name}");
    println!("Num is {num}\nResult is {result}");
    let s = gives_ownership();
    println!("{s}");
    let s2 = takes_and_gives_ownership(s);
    println!(" s2 = {s2}");
}

fn add(x : i32) -> i32{
    x + 10
}
fn takes_ownership(s : String){
    println!("Inside ownership : {s}");
}
fn gives_ownership() -> String{
    let s = String::from("This is a string from gives_ownership function");
    s
}
fn takes_and_gives_ownership(s :String) -> String{
    println!(" S in takes_and_gives_ownership = {s}");
    s
}