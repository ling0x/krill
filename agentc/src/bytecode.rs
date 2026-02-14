// Bytecode IR and compilation
use crate::ast::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeProgram {
    pub agents: Vec<BytecodeAgent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeAgent {
    pub name: String,
    pub state_init: Vec<(String, Value)>,
    pub handlers: Vec<BytecodeHandler>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeHandler {
    pub variant: String,
    pub params: Vec<String>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    LoadVar(String),
    LoadConst(Value),
    Store(String),
    BinOp(BinOp),
    Send {
        target_var: String,
        variant: String,
    },
    Effect {
        name: String,
        arg_count: usize,
    },
    FieldAccess(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
}

pub fn compile(program: &Program) -> Result<BytecodeProgram> {
    let mut agents = Vec::new();

    for agent in &program.agents {
        agents.push(compile_agent(agent)?);
    }

    Ok(BytecodeProgram { agents })
}

fn compile_agent(agent: &AgentDef) -> Result<BytecodeAgent> {
    let mut state_init = Vec::new();

    for state_var in &agent.state {
        let value = eval_const_expr(&state_var.init)?;
        state_init.push((state_var.name.clone(), value));
    }

    let mut handlers = Vec::new();
    for handler in &agent.handlers {
        handlers.push(compile_handler(handler)?);
    }

    Ok(BytecodeAgent {
        name: agent.name.clone(),
        state_init,
        handlers,
    })
}

fn compile_handler(handler: &Handler) -> Result<BytecodeHandler> {
    let mut instructions = Vec::new();

    for stmt in &handler.body {
        compile_stmt(stmt, &mut instructions)?;
    }

    Ok(BytecodeHandler {
        variant: handler.variant.clone(),
        params: handler.params.clone(),
        instructions,
    })
}

fn compile_stmt(stmt: &Stmt, instructions: &mut Vec<Instruction>) -> Result<()> {
    match stmt {
        Stmt::Assign { target, value } => {
            compile_expr(value, instructions)?;
            instructions.push(Instruction::Store(target.clone()));
        }
        Stmt::Send {
            target,
            msg_variant,
            args,
        } => {
            // Load target ref
            if let Expr::Var(target_var) = target {
                // Load args (simplified: assume all args are expressions)
                for arg in args {
                    compile_expr(arg, instructions)?;
                }
                instructions.push(Instruction::Send {
                    target_var: target_var.clone(),
                    variant: msg_variant.clone(),
                });
            }
        }
        Stmt::Effect { name, args } => {
            for arg in args {
                compile_expr(arg, instructions)?;
            }
            instructions.push(Instruction::Effect {
                name: name.clone(),
                arg_count: args.len(),
            });
        }
    }
    Ok(())
}

fn compile_expr(expr: &Expr, instructions: &mut Vec<Instruction>) -> Result<()> {
    match expr {
        Expr::Var(name) => {
            instructions.push(Instruction::LoadVar(name.clone()));
        }
        Expr::Int(n) => {
            instructions.push(Instruction::LoadConst(Value::Int(*n)));
        }
        Expr::Str(s) => {
            instructions.push(Instruction::LoadConst(Value::Str(s.clone())));
        }
        Expr::Bool(b) => {
            instructions.push(Instruction::LoadConst(Value::Bool(*b)));
        }
        Expr::BinOp { op, left, right } => {
            compile_expr(left, instructions)?;
            compile_expr(right, instructions)?;
            instructions.push(Instruction::BinOp(op.clone()));
        }
        Expr::FieldAccess { obj, field } => {
            compile_expr(obj, instructions)?;
            instructions.push(Instruction::FieldAccess(field.clone()));
        }
    }
    Ok(())
}

fn eval_const_expr(expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Str(s) => Ok(Value::Str(s.clone())),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        _ => anyhow::bail!("Non-constant expression in state initialization"),
    }
}
