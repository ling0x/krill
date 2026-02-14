// Simple working example
use agentr::{spawn_actor, Effect, EffectContext, Message};
use anyhow::Result;
use std::sync::Arc;

#[derive(Debug, Clone)]
enum TicketMsg {
    NewTicket { id: i32, priority: String },
}

impl Message for TicketMsg {}

#[tokio::main]
async fn main() -> Result<()> {
    println!("AgentLang Hello World Example\n");

    let effect_ctx = Arc::new(EffectContext::new());
    let log_cap = effect_ctx.grant(Effect::Log).await;

    // Spawn ticket handler agent
    let (ticket_ref, _handle) = spawn_actor(10, move |msg: TicketMsg| {
        let effect_ctx = effect_ctx.clone();
        let log_cap = log_cap.clone();

        async move {
            match msg {
                TicketMsg::NewTicket { id, priority } => {
                    effect_ctx
                        .execute(
                            &log_cap,
                            &[
                                "Processing ticket".to_string(),
                                id.to_string(),
                                "with priority".to_string(),
                                priority,
                            ],
                        )
                        .await?;
                }
            }
            Ok(())
        }
    });

    // Send some messages
    ticket_ref
        .send(TicketMsg::NewTicket {
            id: 1,
            priority: "high".to_string(),
        })
        .await?;

    ticket_ref
        .send(TicketMsg::NewTicket {
            id: 2,
            priority: "low".to_string(),
        })
        .await?;

    // Give actors time to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n✓ Example completed successfully");

    Ok(())
}
