use std::env;
use std::fmt::format;
use std::fs::{self, read};  
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



    // println!("Hello, world!");
    references();
    Ok(())
}



fn references(){
    let mut refereces_lib = HashMap::new();
    refereces_lib.insert("file_io/rw_doc", "https://doc.rust-lang.org/book/ch12-02-reading-a-file.html");

    println!("{:#?}", refereces_lib);
}