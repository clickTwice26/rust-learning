fn main() {
    let name : String = String::from("Shagato Chowdhury");
    let l : usize = calculate_length(&name);
    println!("The length of string `name`: {name} is {l}");

}


fn calculate_length(s : &String) -> usize{
    let result: usize = s.len();
    return result;
}