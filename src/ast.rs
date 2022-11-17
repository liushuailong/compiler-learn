use std::fmt::{Formatter, write};

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

impl std::fmt::Display for CompUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.func_def)
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

impl std::fmt::Display for FuncDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"fun @{}(): {} {}"#, self.ident, self.func_type, self.block)
    }
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}

impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let func_type_temp = match self {
            FuncType::Int => "i32".to_string(),
        };
        write!(f, "{}", func_type_temp)
    }
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{ {}
}}"#, self.stmt)
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub num: i32,
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
%entry:
    ret {}"#, self.num)
    }
}

#[derive(Debug)]
pub struct Number {
    pub num: i32,
}

