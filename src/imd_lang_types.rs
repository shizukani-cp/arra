pub mod ImdLangTypes {

    pub type Statements = Vec<Statement>;

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

    #[derive(Debug)]
    pub struct Expression {
        symbol_and_values:Vec<SymbolAndValues>
    }

    impl Expression {
        fn to_literal(&self) -> Literal {
            unimplemented!();
            Literal {
                type_:"Error".to_string(),
                value:"Error".to_string()
            }
        }
    }

    #[derive(Debug)]
    pub struct Literal {
        type_:String,
        value:String
    }

    #[derive(Debug)]
    pub enum HasLiteralAndEmpty {
        Empty,
        Lit(Literal)
    }

    #[derive(Debug)]
    pub struct Var {
        varname:String,
        value:HasLiteralAndEmpty
    }

    #[derive(Debug)]
    pub struct Case {
        condition:Expression,
        block:Box<Statements>
    }

    #[derive(Debug)]
    pub enum Statement {
        Call{
            func:Expression,
            args:Vec<Expression>
        },
        Switch {
            cases:Vec<Case>
        },
        Loop {
            block:Box<Statements>
        },
        Substitution {
            var:Var,
            right_hand_side:Expression
        }
        NotImplement
    }
}