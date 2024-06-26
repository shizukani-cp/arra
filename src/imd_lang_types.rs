pub mod imd_lang_types {

    pub type Statements = Vec<Statement>;

    #[derive(Debug)]
    pub enum Symbols {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        Pow,
        Equal,
        NotEqual,
        Big,
        Small,
        EqualOrSmall,
        EqualOrBig
    }

    impl Symbols {
        fn to_op_func_name(&self) -> String {
            match self {
                Symbols::Add => "ad",
                Symbols::Sub => "sb",
                Symbols::Mul => "ml",
                Symbols::Div => "dv",
                Symbols::Mod => "md",
                Symbols::Pow => "pw",
                Symbols::Equal => "eq",
                Symbols::NotEqual => "ne",
                Symbols::Big => "gt",
                Symbols::Small => "lt",
                Symbols::EqualOrSmall => "le",
                Symbols::EqualOrBig => "ge"
            }
        }
    }

    #[derive(Debug)]
    pub enum SymbolAndValues {
        Lit(Literal),
        Variable(Var),
        Simbol(Symbols),
        Attr(Vec<String>),
        Ref {
            object:Expression,
            index:Expression
        },
        Call {
            func:Expression,
            args:Vec<Expression>
        }
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
        },
        NameSpace {
            block:Box<Statements>
        },
        Return(Expression),
        Break,
        Continue,
        NotImplement
    }
}