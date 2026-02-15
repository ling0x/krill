# Krill 🦐

A minimal statically-typed actor-based language for AI agents, compiled to bytecode and executed on a Rust runtime.

## Features

- **Typed actors**: Each agent has a typed mailbox (`Ref[MessageType]`)
- **Message passing**: Asynchronous send with Tokio channels
- **Effect system**: Capability-gated side effects (HTTP, logging, etc.)
- **Static typing**: Compile-time type checking of messages and effects
- **Supervision**: Automatic restart on agent crashes

## Quick Start

### Prerequisites

- Rust 1.75+ and Cargo

### Install & Run

```bash
# Clone the repo
git clone https://github.com/ling0x/krill.git
cd krill

# Build
cargo build --release

# Run Rust runtime example
cargo run --example hello
```

## Language Syntax

```agent
// Define message types
type TicketMsg {
  NewTicket { id: Int, priority: String, replyTo: Ref[Response] }
  Status { replyTo: Ref[StatusResponse] }
}

type Response {
  Ack { ticket_id: Int }
}

type StatusResponse {
  Count { num: Int }
}

// Define an agent
agent TicketHandler {
  // Initial state
  state {
    tickets: Int = 0
  }
  
  // Message handlers
  on NewTicket { id, priority, replyTo } -> {
    state.tickets = state.tickets + 1;
    log("Processing ticket", id);
    send replyTo Ack { ticket_id: id };
  }
  
  on Status { replyTo } -> {
    send replyTo Count { num: state.tickets };
  }
}
```

## Examples

- Language-level `.agent` examples live in `examples/`.
- Rust runtime embedding/examples live in `agentr/examples/`.

## Project Structure

```
krill/
├── Cargo.toml              # Workspace manifest
├── agentr/                 # Runtime crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Public API
│       ├── actor.rs        # Actor system
│       ├── mailbox.rs      # Typed mailboxes
│       └── effects.rs      # Effect system
├── agentc/                 # Compiler crate
│   ├── Cargo.toml
│   ├── build.rs            # LALRPOP build script
│   └── src/
│       ├── main.rs         # CLI entry point
│       ├── grammar.lalrpop # Parser grammar
│       ├── ast.rs          # AST definitions
│       ├── typechecker.rs  # Type checking
│       ├── bytecode.rs     # Bytecode IR
│       └── interpreter.rs  # Bytecode executor
└── examples/               # Krill language examples (.agent)
```

## Roadmap

- [x] Basic actor system
- [x] Static type checking
- [x] Effect system
- [ ] BDI-style goals and plans
- [ ] Distributed runtime (NATS)
- [ ] Rust code generation backend
- [ ] Hot code reload

## License

MIT
