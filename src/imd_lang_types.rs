pub mod imd_lang_types {

    use std::collections::HashMap;

    pub type Statements = Vec<Statement>;
    pub type AttrKeyAndValue = HashMap<String, String>;

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
            }.to_string()
        }
    }

    #[derive(Debug)]
    pub enum FormulaElements {
        Synbol(Symbols),
        Literal(Literal),
        Variable(Var)
    }

    #[derive(Debug)]
    pub enum Expression {
        Lit(Literal),
        VarOrAttr(VarOrAttr),
        Formula(Vec<FormulaElements>),
        Module(String),
        Function {
            args:Vec<Var>,
            block:Box<Statements>
        },
        Ref {
            object:Box<Expression>,
            index:Box<Expression>
        },
        Call {
            func:Box<Expression>,
            args:Vec<Expression>
        },
        NotImplement
    }

    #[derive(Debug)]
    pub struct Literal {
        pub value:String
    }

    #[derive(Debug)]
    pub enum VarOrAttr {
        Variable(Var),
        Attr(Vec<String>)
    }

    #[derive(Debug)]
    pub struct Var {
        pub varname:String,
        pub value:AttrKeyAndValue
    } 

    #[derive(Debug)]
    pub struct Case {
        pub condition:Expression,
        pub block:Box<Statements>
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
            left_hand_side:VarOrAttr,
            right_hand_side:Expression
        },
        NameSpace {
            block:Box<Statements>,
            super_space:Option<String>
        },
        AddTmp(Expression),
        Return(Expression),
        Export(Var),
        Break,
        Continue,
        NotImplement
    }
}