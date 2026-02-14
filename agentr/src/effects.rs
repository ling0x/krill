// Effect system for capability-based security
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Effect types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    Log,
    Http,
    FileRead,
    FileWrite,
}

/// Capability token (unforgeable reference)
#[derive(Clone)]
pub struct Capability {
    effect: Effect,
    id: u64,
}

impl Capability {
    pub fn new(effect: Effect, id: u64) -> Self {
        Self { effect, id }
    }

    pub fn effect(&self) -> &Effect {
        &self.effect
    }
}

/// Effect context - tracks allowed effects
pub struct EffectContext {
    capabilities: Arc<RwLock<HashMap<u64, Effect>>>,
    next_id: Arc<RwLock<u64>>,
}

impl EffectContext {
    pub fn new() -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Grant a capability for an effect
    pub async fn grant(&self, effect: Effect) -> Capability {
        let mut next_id = self.next_id.write().await;
        let id = *next_id;
        *next_id += 1;

        let mut caps = self.capabilities.write().await;
        caps.insert(id, effect.clone());

        Capability::new(effect, id)
    }

    /// Verify a capability is valid
    pub async fn verify(&self, cap: &Capability) -> Result<()> {
        let caps = self.capabilities.read().await;
        match caps.get(&cap.id) {
            Some(effect) if effect == &cap.effect => Ok(()),
            _ => anyhow::bail!("Invalid capability"),
        }
    }

    /// Execute an effect operation (basic implementations)
    pub async fn execute(&self, cap: &Capability, args: &[String]) -> Result<String> {
        self.verify(cap).await?;

        match cap.effect() {
            Effect::Log => {
                let msg = args.join(" ");
                println!("[LOG] {}", msg);
                Ok(msg)
            }
            Effect::Http => {
                // Stub: would use reqwest in real implementation
                Ok(format!("HTTP request to {}", args.get(0).unwrap_or(&"unknown".to_string())))
            }
            Effect::FileRead => {
                // Stub: would use tokio::fs in real implementation
                Ok(format!("Read file: {}", args.get(0).unwrap_or(&"unknown".to_string())))
            }
            Effect::FileWrite => {
                // Stub
                Ok(format!("Wrote to file: {}", args.get(0).unwrap_or(&"unknown".to_string())))
            }
        }
    }
}

impl Default for EffectContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_capability_grant_and_verify() {
        let ctx = EffectContext::new();
        let cap = ctx.grant(Effect::Log).await;
        assert!(ctx.verify(&cap).await.is_ok());
    }

    #[tokio::test]
    async fn test_log_effect() {
        let ctx = EffectContext::new();
        let cap = ctx.grant(Effect::Log).await;
        let result = ctx.execute(&cap, &["Hello".to_string(), "World".to_string()]).await;
        assert!(result.is_ok());
    }
}
