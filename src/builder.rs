pub mod builder{
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
        let imd_lang_code: imd_lang_types::Statements = vec![];
        let iter_code = veccode.iter();
        while let Some(row) = iter_code.next() {
            if ref_cell(row, 1) == "=".to_string() {
                imd_lang_code.push(imd_lang_types::Statement::Substitution{
                    left_hand_side: imd_lang_types::VarOrAttr::Var {
                        varname: row[0],
                        value: imd_lang_types::HasLiteralAndEmpty::Empty
                    },
                    right_hand_side: parse_expression((&row[2..]).to_vec())
                });
            } else if ref_cell(row, 0) == "attr".to_string() {
                if row.contains(&("=".to_string())) {
                    if let Some(index) = row.iter().position(|s| *s == "=".to_string()) {
                        imd_lang_code.push(imd_lang_types::Statement::Substitution {
                            left_hand_side: imd_lang_types::VarOrAttr::Attr((&row[..index]).to_vec()),
                            right_hand_side: parse_expression((&row[(index + 2)..]).to_vec())
                        })
                    }
                } else {
                    imd_lang_code.push(imd_lang_types::Statement::AddTmp(parse_expression(*row)))
                }
            } else {
                panic!("Invaild syntax.");
            }
        }
        imd_lang_code
    }

    fn ref_cell(row: &Vec<String>, col: usize) -> String{
        if row.len() <= col {
            return "".to_string();
        }
        row.as_slice()[col].clone()
    }

    fn parse_expression(vec_expression: Vec<String>) -> imd_lang_types::Expression {
        unimplemented!();
        imd_lang_types::Expression {
            symbol_and_values: vec![imd_lang_types::SymbolAndValues::NotImplement]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::builder;

    fn testing_str_code_to_vec(s: &str, v: Vec<Vec<&str>>) {
        let mut vcode: Vec<Vec<String>> = vec![];
        for r in v {
            let mut row: Vec<String> = vec![];
            for c in r {
                row.push(c.to_string());
            }
            vcode.push(row);
        }
        assert_eq!(builder::str_code_to_vec(s.to_string()), vcode)
    }

    #[test]
    fn test1() {
        testing_str_code_to_vec(
            "call,print,\"Hello, World!\"",
            vec![vec!["call", "print", "\"Hello, World!\""]]
        )
    }

    #[test]
    fn comment_exclusion() {
        testing_str_code_to_vec(
            "call,print,\"Hello, World!\", #calling print(printを呼び出し)",
            vec![vec!["call", "print", "\"Hello, World!\""]]
        )
    }
}