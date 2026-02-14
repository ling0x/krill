// Type checking pass
use crate::ast::*;
use anyhow::{bail, Result};
use std::collections::HashMap;

pub fn typecheck(program: &Program) -> Result<()> {
    let mut ctx = TypeContext::new();

    // Register all type definitions
    for type_def in &program.types {
        ctx.register_type(type_def)?;
    }

    // Check each agent
    for agent in &program.agents {
        check_agent(&ctx, agent)?;
    }

    Ok(())
}

struct TypeContext {
    types: HashMap<String, TypeDef>,
}

impl TypeContext {
    fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    fn register_type(&mut self, type_def: &TypeDef) -> Result<()> {
        if self.types.contains_key(&type_def.name) {
            bail!("Duplicate type definition: {}", type_def.name);
        }
        self.types.insert(type_def.name.clone(), type_def.clone());
        Ok(())
    }

    fn get_type(&self, name: &str) -> Option<&TypeDef> {
        self.types.get(name)
    }

    fn variant_exists(&self, type_name: &str, variant_name: &str) -> bool {
        if let Some(type_def) = self.get_type(type_name) {
            type_def.variants.iter().any(|v| v.name == variant_name)
        } else {
            false
        }
    }
}

fn check_agent(ctx: &TypeContext, agent: &AgentDef) -> Result<()> {
    let mut env = HashMap::new();

    // Add state variables to environment
    for state_var in &agent.state {
        env.insert(state_var.name.clone(), state_var.ty.clone());
    }

    // Check each handler
    for handler in &agent.handlers {
        check_handler(ctx, &env, handler)?;
    }

    Ok(())
}

fn check_handler(ctx: &TypeContext, env: &HashMap<String, Type>, handler: &Handler) -> Result<()> {
    let mut local_env = env.clone();

    // Add handler parameters to environment (basic checking)
    for param in &handler.params {
        local_env.insert(param.clone(), Type::Int); // Simplified: assume Int for now
    }

    // Check each statement
    for stmt in &handler.body {
        check_stmt(ctx, &local_env, stmt)?;
    }

    Ok(())
}

fn check_stmt(ctx: &TypeContext, env: &HashMap<String, Type>, stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::Assign { target, value } => {
            if !env.contains_key(target) {
                bail!("Undefined variable: {}", target);
            }
            let _value_ty = infer_expr(env, value)?;
            // Should check types match, but simplified for v0
            Ok(())
        }
        Stmt::Send {
            target,
            msg_variant: _,
            args: _,
        } => {
            let _target_ty = infer_expr(env, target)?;
            // Should check target is Ref[T] and variant exists in T
            Ok(())
        }
        Stmt::Effect { name: _, args: _ } => {
            // Effects are checked at runtime via capabilities
            Ok(())
        }
    }
}

fn infer_expr(env: &HashMap<String, Type>, expr: &Expr) -> Result<Type> {
    match expr {
        Expr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Undefined variable: {}", name)),
        Expr::Int(_) => Ok(Type::Int),
        Expr::Str(_) => Ok(Type::String),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::BinOp {
            op: _,
            left,
            right,
        } => {
            let _left_ty = infer_expr(env, left)?;
            let _right_ty = infer_expr(env, right)?;
            Ok(Type::Int) // Simplified
        }
        Expr::FieldAccess { obj, field: _ } => infer_expr(env, obj), // Simplified
    }
}
