pub mod compiler{
    use std::fs::File;
    use std::io::prelude::*;

    use crate::imd_lang_types::imd_lang_types;
    
    pub fn generate_code(filename:&String) -> imd_lang_types::Statements {
        let vec_code = str_code_to_vec(read_file(filename));
        println!("{:?}", vec_code);
        vec_code_to_imd_lang(vec_code)
    }

    pub fn read_file(filename:&String) -> String{
        let mut f = File::open(filename)
            .expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
    }

    pub fn str_code_to_vec(str_code:String) -> Vec<Vec<String>> {
        let mut code = vec![];
        for str_row in str_code.as_str().split("\n"){
            let mut code_row = vec![];
            let sp: Vec<_> = str_row.split(",").collect();
            let mut cells = sp.iter().peekable();
            'row: loop{
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
                    if cell_code.chars().next() == None {
                        break;
                    }
                    //println!("{}", cell_code.chars().next().unwrap());
                    //println!("{:?}", cell_code.chars());
                    if cell_code.chars().next().unwrap() == '#' {
                        continue 'row;
                    }
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

    pub fn vec_code_to_imd_lang(veccode:Vec<Vec<String>>) -> imd_lang_types::Statements {
        unimplemented!();
        vec![imd_lang_types::Statement::NotImplement]
    }
}