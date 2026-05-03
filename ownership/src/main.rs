fn main() {
    let name : String = String::from("Shagato Chowdhury");
    let (name, l) = calculate_length(name);
    println!("The length of string `name`: {name} is {l}");

}


fn calculate_length(s : String) -> (String, usize){
    let result: usize = s.len();
    return (s, result);
}