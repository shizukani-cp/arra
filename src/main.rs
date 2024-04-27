use std::env;
use std::fs::File;
use std::io::prelude::*;

pub mod ImdLangTypes {

    use std::fmt;

    #[derive(Debug)]
    pub enum Symbols {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        Pow,
        Equal
    }

    #[derive(Debug)]
    pub enum SymbolAndValues {
        Lit(Literal),
        Variable(Var),
        Simbol(Symbols)
    }

    pub struct Expression {
        symbol_and_values:Vec<SymbolAndValues>
    }

    impl Expression {
        fn to_literal(&self) -> Literal {
            panic!("this method is not implemented.");
            Literal {
                type_:"Error".to_string(),
                value:"Error".to_string()
            }
        }
    }

    impl fmt::Debug for Expression {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Expression")
                .field("symbol_and_values", &self.symbol_and_values)
                .finish()
        }
    }

    pub struct Literal {
        type_:String,
        value:String
    }

    impl fmt::Debug for Literal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Literal")
                .field("type_", &self.type_)
                .field("value", &self.value)
                .finish()
        }
    }

    pub struct Var {
        varname:String,
        value:Literal
    }

    impl fmt::Debug for Var {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Var")
                .field("varname", &self.varname)
                .field("value", &self.value)
                .finish()
        }
    }

    pub struct Calling {
        func:Expression,
        args:Vec<Expression>
    }

    impl fmt::Debug for Calling {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Calling")
                .field("func", &self.func)
                .field("args", &self.args)
                .finish()
        }
    }

    pub struct Case {
        condition:Expression,
        block:Box<Vec<ImdLangType>>
    }

    impl fmt::Debug for Case {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Case")
                .field("condtion", &self.condition)
                .field("block", &self.block)
                .finish()
        }
    }

    pub struct SwitchStatement {
        cases:Vec<Case>
    }

    impl fmt::Debug for SwitchStatement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwitchStatement")
                .field("cases", &self.cases)
                .finish()
        }
    }

    pub struct LoopStatement {
        block:Box<Vec<ImdLangType>>
    }

    impl fmt::Debug for LoopStatement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LoopStatement")
                .field("block", &self.block)
                .finish()
        }
    }

    pub struct SubstitutionStatement {
        var:Var,
        right_hand_side:Expression
    }

    impl fmt::Debug for SubstitutionStatement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubstitutionStatement")
                .field("var", &self.var)
                .field("right_hand_side", &self.right_hand_side)
                .finish()
        }
    }

    pub struct InstanceExpression {
        type_:String,
        args:Vec<Expression>
    }

    impl fmt::Debug for InstanceExpression {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("InstanceExpression")
                .field("type_", &self.type_)
                .field("args", &self.args)
                .finish()
        }
    }

    #[derive(Debug)]
    pub enum ImdLangType {
        Call(Calling),
        Switch(SwitchStatement),
        Loop(LoopStatement),
        Substitution(SubstitutionStatement),
        Instance(InstanceExpression),
        Tmp
    }
}

fn generate_code(filename:&String) -> Vec<ImdLangTypes::ImdLangType> {
    let vec_code = str_code_to_vec(read_file(filename));
    println!("{:?}", vec_code);
    vec_code_to_imd_lang(vec_code)
}

fn read_file(filename:&String) -> String{
    let mut f = File::open(filename)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn str_code_to_vec(str_code:String) -> Vec<Vec<String>> {
    let mut code = vec![];
    for str_row in str_code.as_str().split("\n"){
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
                if cell_code.chars().next() == None {
                    break;
                }
                //println!("{}", cell_code.chars().next().unwrap());
                //println!("{:?}", cell_code.chars());
                if cell_code.chars().next().unwrap() == '#' {
                    cell_code = "".to_string();
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

fn vec_code_to_imd_lang(veccode:Vec<Vec<String>>) -> Vec<ImdLangTypes::ImdLangType> {
    //panic!("this method is not implemented.");
    vec![ImdLangTypes::ImdLangType::Tmp]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", generate_code(&filename));
}
