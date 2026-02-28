use std::collections::HashMap;
use std::env;
use std::fs;
use std::task::Context;

const DEBUG_MODE_ON: bool = true;

const keywords: [&str; 2] = ["auto", "int"];
const operators: [char; 4] = ['=', '+', '-', '*'];
const delimiter: char = ';';

fn dprint(message: String) -> bool {
    if !DEBUG_MODE_ON {
        return false;
    }
    let debug_msg_prefix: &str = "[DEBUG]::";
    println!("{}->{}", debug_msg_prefix, message);
    true
}
fn find_header_files(code_str: String) -> Vec<String> {
    dprint(format!("from {{find_header_files}}"));
    let header_files: Vec<&str> = vec![];
    let mut preprocessor_found: bool = false;
    let code_str_length: usize = code_str.len();
    // println!("Code String Length : {}", code_str_length);
    dprint(format!("Code String Length : {}", code_str_length));
    return vec!["hello".to_string()];
    let mut index: u8 = 0;
    for i in code_str.chars() {
        if i == '\n' {
            println!("NewLine found");
        }
        if i == '#' {
            dprint(format!("Preprocessor found at {index}"));
            preprocessor_found = true;
        }

        if preprocessor_found == true && i == '<' {
            let mut n: usize = index as usize;

            while n < code_str_length {
                let temp_char: char;
                match code_str.chars().nth(n) {
                    Some(c) => temp_char = c,
                    None => println!("Index out of range"),
                };
                if temp_char == '>' {
                    println!("Delimiter found for the preprocessor at {}", n);
                }

                n += 1;
                continue;
            }
            preprocessor_found = false;
        }
        index += 1;
    }

    vec!["test".to_string()]
}
fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?;
    let file_path_str: String = format!("{}/sample.c", file_path.display());
    // println!("In file {}", file_path.display());
    let contents: String = fs::read_to_string(file_path_str).expect("Unable to read the file");
    println!("File Content : \n{contents}");
    // println!("{:?}", contents.chars().nth(1));
    dprint(format!(
        "Content Length: {:?}",
        contents.chars().size_hint()
    ));
    find_header_files(contents);
    let mut counter: usize = 0;
    // we are using usize because it can't be negative
    loop {
        break;
        if counter > 10 {
            break;
        }
        //print!("{:?}|",contents.chars().nth(counter));

        match contents.chars().nth(counter) {
            Some(c) => println!("{}", c),
            None => println!("Char not found at that index {}", counter),
        }
        counter += 1;
    }

    // println!("Hello, world!");
    references();
    Ok(())
}

fn references() {
    let mut refereces_lib = HashMap::new();
    refereces_lib.insert(
        "file_io/rw_doc",
        "https://doc.rust-lang.org/book/ch12-02-reading-a-file.html",
    );

    println!("{:#?}", refereces_lib);
}
