// Abstract Syntax Tree definitions
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub types: Vec<TypeDef>,
    pub agents: Vec<AgentDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Ref(String),   // Ref[MessageType]
    Named(String), // User-defined type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDef {
    pub name: String,
    pub state: Vec<StateVar>,
    pub handlers: Vec<Handler>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateVar {
    pub name: String,
    pub ty: Type,
    pub init: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handler {
    pub variant: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Assign { target: String, value: Expr },
    Send {
        target: Expr,
        msg_variant: String,
        args: Vec<Expr>,
    },
    Effect { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Var(String),
    Int(i64),
    Str(String),
    Bool(bool),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FieldAccess { obj: Box<Expr>, field: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
}
