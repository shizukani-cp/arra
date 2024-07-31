pub mod builder{
    use std::fs::File;
    use std::io::prelude::*;

    use crate::imd_lang_types::imd_lang_types;
    use super::syntax_error;
    
    pub fn generate_code(source_filename:&String, imd_filename:&String) -> imd_lang_types::Statements {
        let vec_code = str_code_to_vec(read_file(source_filename));
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
        let mut in_quotes = false;
        let mut current_string = String::new();
        let mut row = vec![];
        for row_s in str_code.split("\n") {
            for c in row_s.chars() {
                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                        current_string.push(c);
                        if !in_quotes && !(current_string.is_empty()) {
                            row.push(current_string.clone());
                            current_string.clear();
                        }
                    },
                    ',' => {
                        if in_quotes {
                            current_string.push(c);
                        } else {
                            if !current_string.is_empty() {
                                let cell = current_string.clone().trim().to_owned();
                                row.push(cell);
                                current_string.clear();
                            }
                        }
                    }
                    '\r' => {},
                    '#' => {
                        break;
                    }
                    _ => {
                        current_string.push(c)
                    }
                }
            }
            if !(row.is_empty()){
                code.push(row.clone());
            }
            row.clear();
        }
        code
    }
    pub fn vec_code_to_imd_lang(veccode:Vec<Vec<String>>) -> imd_lang_types::Statements {
        unimplemented!();
        let imd_lang_code: imd_lang_types::Statements = vec![];
        let iter_code = veccode.iter();
        while let Some(row) = iter_code.next() {
            if ref_cell(row, 1) == "=".to_string() {  // 変数に代入する場合
                imd_lang_code.push(imd_lang_types::Statement::Substitution{
                    left_hand_side: imd_lang_types::VarOrAttr::Variable(imd_lang_types::Var {
                        varname: row[0],
                        value: imd_lang_types::AttrKeyAndValue::new()
                    }),
                    right_hand_side: parse_expression((&row[2..]).to_vec())
                });
            } else if ref_cell(row, 0) == "attr".to_string() {  // 属性に代入する場合
                if let Some(index) = row.iter().position(|s| *s == "=".to_string()) {
                    imd_lang_code.push(imd_lang_types::Statement::Substitution {
                        left_hand_side: imd_lang_types::VarOrAttr::Attr((&row[..index]).to_vec()),
                        right_hand_side: parse_expression((&row[(index + 2)..]).to_vec())
                    })
                } else {
                    syntax_error::invaild_syntax_error();
                }
            } else {
                syntax_error::invaild_syntax_error();
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
        imd_lang_types::Expression::NotImplement
    }
}

mod syntax_error {
    
    pub fn invaild_syntax_error() {
        print_error("Invaild syntax");
    }

    fn print_error(message: &str) {
        panic!("{}", message);
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