use std::collections::HashMap;
use std::env;
use std::fs;

const DEBUG_MODE_ON: bool = true;
const KEYWORDS: [&str; 3] = ["auto", "int", "float"];
const OPERATORS: [char; 4] = ['=', '+', '-', '*'];
const DELIMITER: char = ';';

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
    let mut header_files: Vec<String> = vec![];
    let mut preprocessor_found: bool = false;
    let code_str_length: usize = code_str.len();
    // println!("Code String Length : {}", code_str_length);
    dprint(format!("Code String Length : {}", code_str_length));
    // return vec!["hello".to_string()];
    let mut index: u8 = 0;
    for i in code_str.chars() {
        // println!("{i}");
        if i == '\n' {
            dprint("Newline found".to_string());
        }
        if i == '#' {
            dprint(format!("Preprocessor found at {index}"));
            preprocessor_found = true;
        }

        if preprocessor_found == true && i == '<' {
            let mut n: usize = index as usize;
            let mut header_file_name : String = String::new();
            while n < code_str_length {
                let Some(temp_char) = code_str.chars().nth(n) else{
                    return vec![];
                };
                header_file_name = format!("{header_file_name}{temp_char}");

                // match code_str.chars().nth(n) {
                //     Some(c) => temp_char = c,
                //     None => {
                //         dprint(format!("None returned with the index {n}"));
                //         return vec![]},
                // };
                
                if temp_char == '>' {
                    dprint(format!("Delimiter found for the preprocessor at {}", n));
                    
                    header_files.push(header_file_name.to_string());
                    break;
                }

                n += 1;
                continue;
            }
            preprocessor_found = false;
        }
        index += 1;
    }

    header_files
}

fn find_defined_syntax(code_str: String) -> Vec<String>{
    let words: Vec<&str>  = code_str.split_whitespace().collect();

    for i in words{
        if KEYWORDS.contains(&i){
            println!("{i} is a keyword");
        }
    }
        
    
    vec![]
}
fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?;
    let file_path_str: String = format!("{}/sample.c", file_path.display());
    // println!("In file {}", file_path.display());
    let contents: String = fs::read_to_string(file_path_str).expect("Unable to read the file");
    let mut contents_2 = contents.clone();
    println!("File Content : \n{contents}");
    // println!("{:?}", contents.chars().nth(1));
    dprint(format!(
        "Content Length: {:?}",
        contents.chars().size_hint()
    ));
    println!("{:?}",find_header_files(contents));
    find_defined_syntax(contents_2);
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
