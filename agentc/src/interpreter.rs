// Bytecode interpreter
use crate::bytecode::*;
use agentr::{Effect, EffectContext};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn execute(program: BytecodeProgram) -> Result<()> {
    let effect_ctx = Arc::new(EffectContext::new());

    // Grant log capability for all agents
    let log_cap = effect_ctx.grant(Effect::Log).await;

    // Execute first agent for demo (full version would spawn all agents)
    if let Some(agent) = program.agents.first() {
        execute_agent(agent, effect_ctx.clone(), log_cap).await?;
    }

    Ok(())
}

async fn execute_agent(
    agent: &BytecodeAgent,
    effect_ctx: Arc<EffectContext>,
    log_cap: agentr::Capability,
) -> Result<()> {
    println!("Executing agent: {}", agent.name);

    // Initialize state
    let state = Arc::new(RwLock::new(HashMap::new()));
    for (name, value) in &agent.state_init {
        state.write().await.insert(name.clone(), value.clone());
    }

    // For demo: execute first handler with synthetic message
    if let Some(handler) = agent.handlers.first() {
        println!("  Handler: {}", handler.variant);
        execute_handler(
            handler,
            state.clone(),
            effect_ctx.clone(),
            log_cap.clone(),
            &HashMap::new(),
        )
        .await?;
    }

    Ok(())
}

async fn execute_handler(
    handler: &BytecodeHandler,
    state: Arc<RwLock<HashMap<String, Value>>>,
    effect_ctx: Arc<EffectContext>,
    log_cap: agentr::Capability,
    _params: &HashMap<String, Value>,
) -> Result<()> {
    let mut stack: Vec<Value> = Vec::new();

    for instr in &handler.instructions {
        match instr {
            Instruction::LoadVar(name) => {
                let state_read = state.read().await;
                if let Some(val) = state_read.get(name) {
                    stack.push(val.clone());
                }
            }
            Instruction::LoadConst(val) => {
                stack.push(val.clone());
            }
            Instruction::Store(name) => {
                if let Some(val) = stack.pop() {
                    state.write().await.insert(name.clone(), val);
                }
            }
            Instruction::BinOp(op) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = eval_binop(op, &left, &right)?;
                stack.push(result);
            }
            Instruction::Send {
                target_var: _,
                variant: _,
            } => {
                // Simplified: just pop args from stack
                // In full version, would actually send to target actor
                println!("    [Send message - not implemented in demo]");
            }
            Instruction::Effect { name, arg_count } => {
                let mut args = Vec::new();
                for _ in 0..*arg_count {
                    if let Some(val) = stack.pop() {
                        args.push(value_to_string(&val));
                    }
                }
                args.reverse();

                if name == "log" {
                    effect_ctx.execute(&log_cap, &args).await?;
                }
            }
            Instruction::FieldAccess(_field) => {
                // Simplified: not implemented in demo
            }
        }
    }

    Ok(())
}

fn eval_binop(op: &crate::ast::BinOp, left: &Value, right: &Value) -> Result<Value> {
    use crate::ast::BinOp::*;
    match (op, left, right) {
        (Add, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
        (Sub, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
        (Mul, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
        (Div, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
        _ => anyhow::bail!("Type error in binary operation"),
    }
}

fn value_to_string(val: &Value) -> String {
    match val {
        Value::Int(n) => n.to_string(),
        Value::Str(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
    }
}
