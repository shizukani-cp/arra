use std::env;
use std::fs::File;
use std::io::prelude::*;

fn generate_code(filename:&String) -> Vec<Vec<String>> {
    let mut f = File::open(filename)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    let mut code = vec![];
    for str_row in contents.as_str().split("\n"){
        let mut code_row = vec![];
        let sp: Vec<_> = str_row.split(",").collect();
        let mut cells = sp.iter().peekable();
        loop{
            let mut cell_code = "".to_string();
            let mut in_str = false;
            loop{
                let code = match cells.next(){
                    Some(v) => v,
                    _ => ""
                }.replace("\r", "\n");
                if !in_str{
                    cell_code += code.trim();
                } else {
                    cell_code += &(",".to_string().to_owned() + &code.lines().collect::<String>());
                }
                //println!("{}", cell_code);
                //println!("\"\\\"\" num is {}", vec!(cell_code.chars().filter(|&s| s.to_string() == "\"".to_string())).len() + 1);
                //println!("{:?}", code);
                let mut back_slash_num = 0;
                for s in cell_code.chars() {
                    if s.to_string() == "\"".to_string(){
                        back_slash_num += 1;
                    }
                }
                if back_slash_num % 2 == 0 {
                    break;
                } else {
                    in_str = true;
                }
            }
            if cell_code != ""{
                code_row.push(cell_code);
            }
            if cells.peek() == None {
                break;
            }
        }
        if !(code_row.is_empty()){
            code.push(code_row);
        }
    }
    code
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", generate_code(&filename));
}
