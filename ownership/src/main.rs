fn main() {
    let mut name : String = String::from("Shagato Chowdhury");
    let l : usize = calculate_length(&mut name);
    println!("The length of string `name`: {name} is {l}");

}


fn calculate_length( s : &mut String) ->usize{
    s.push_str("Hello world");
    let result: usize = s.len();
    println!("Inside the function");
    println!("{}", *s);
    return result
}