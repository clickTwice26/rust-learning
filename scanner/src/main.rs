use std::env;
use std::fs;
use std::collections::HashMap;  

const DEBUG_MODE_ON : bool = true;


fn dprint(message : &str)->bool{
    if !DEBUG_MODE_ON{
        return false;
    }
    let debug_msg_prefix : &str = "[DEBUG]::";
    println!("{}->{}", debug_msg_prefix, message);
    true
}

fn main() -> std::io::Result<()>{
    let file_path = env::current_dir()?;
    let file_path_str : String = format!("{}/sample.c",file_path.display());

    // println!("In file {}", file_path.display());
    let contents: String = fs::read_to_string(file_path_str)
        .expect("Unable to read the file");
    println!("File Content : \n{contents}");


    let mut counter : usize = 0;
    // we are using usize because it can't be negative 
    loop {
        if counter > 10{
            break;
        }
        //print!("{:?}|",contents.chars().nth(counter));
        match contents.chars().nth(counter) {
            Some(c) => println!("{}", c),
            None => println!("Char not found at that index{}", counter)
        }
        counter+=1;
    }


    // println!("Hello, world!");
    references();
    Ok(())
}



fn references(){
    let mut refereces_lib = HashMap::new();
    refereces_lib.insert("file_io/rw_doc", "https://doc.rust-lang.org/book/ch12-02-reading-a-file.html");

    println!("{:#?}", refereces_lib);
}