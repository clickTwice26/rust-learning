use std::collections::HashMap;
use std::env;
use std::fs;

const DEBUG_MODE_ON: bool = false;
const KEYWORDS: [&str; 4] = ["auto", "int", "float", "return"];
const OPERATORS: [&str; 4] = ["=", "+", "-", "*"];
const DELIMITER: char = ';';
const DIGITS : [char; 10]= ['1','2','3','4','5','6','7','8','9','0'];

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
        // if i == '\n' {
        //     dprint("Newline found".to_string());
        // }

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
    let mut is_found_keyword: bool = false;
    let mut is_found_operator : bool = false;
    for i in words{
        if KEYWORDS.contains(&i){
            println!("{i} is a keyword");
            // println!("{}")
            is_found_keyword = true;
            continue;
        }
        if OPERATORS.contains(&i){
            println!("{i} is an operator");
            is_found_operator = true;
            continue;
        }
        if is_found_keyword {
            println!("{i} is a identifer");
            is_found_keyword = false;
            continue;
        }
        if is_found_operator {
            // println!("{i} is")
            dprint("Found word after an operator".to_string());
            // then sanitize is it a value or identifier
            // if i.starts_with('"'){
            //     println!("{}")
            // }
            if DIGITS.iter().any(|&c| i.starts_with(c)){
                println!("{i} is a value");

            }else{
                println!("{i} is a identifier")
            } 
            is_found_keyword = false;
            continue;

        }
        
        
        

    }
        
    
    vec![]
}
fn main() -> std::io::Result<()> {
    let mut identifier : Vec<String> = vec![];

    let file_path = env::current_dir()?;
    let file_path_str: String = format!("{}/sample.c", file_path.display());
    // println!("In file {}", file_path.display());
    let contents: String = fs::read_to_string(file_path_str).expect("Unable to read the file");
    let mut contents_2 = contents.clone();
    dprint(format!("\n{contents}"));
    // println!("{:?}", contents.chars().nth(1));
    dprint(format!(
        "Content Length: {:?}",
        contents.chars().size_hint()
    ));
    println!("{:?}",find_header_files(contents));
    find_defined_syntax(contents_2);
    let mut counter: usize = 0;
    // we are using usize because it can't be negative


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
