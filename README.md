> **Work state:** RELEASE · **Progress:** `████████░░` 90%

Apisync is a Rust API toolkit for building REST, GraphQL, and WebSocket services with async-first handlers, middleware, and type-safe routing. It is designed as a foundation for API synchronization and integration workflows, with a compact core that keeps REST, schema, and connection-management concerns aligned in one library.

## Usage / Quickstart

```bash
cargo add apisync
cargo build
cargo test
```

```rust
use std::sync::Arc;
use apisync::adapters::rest::HyperServer;
use apisync::endpoints::ItemCrudEndpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let endpoint = Arc::new(ItemCrudEndpoint::new());
    let server = HyperServer::new("127.0.0.1:8080".parse()?, endpoint).await?;
    server.run().await?;
    Ok(())
}
```

Module map: `apisync::domain` (core types + `Endpoint` trait),
`apisync::endpoints` (`ItemCrudEndpoint`, `HealthzEndpoint`, `ReadyzEndpoint`),
`apisync::adapters::{rest, graphql, websocket}` (transports), `apisync::application`
(`Router`, `Handler`), `apisync::infrastructure::logging::init()`.

See the [docs site](https://kooshapari.github.io/apisync/) for the full
[quick start](docs/quickstart.md), [API reference](docs/api.md), and
[architecture](docs/architecture.md).

<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->

![Downloads](https://img.shields.io/github/downloads/KooshaPari/Apisync/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/Apisync?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/Apisync?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.

<!-- AI-DD-META:END -->

> **Work state:** RELEASE · **Progress:** `████████░░ 90%`
> Rust API toolkit (REST/GraphQL/WebSocket); docs + governance complete, first release cut · updated 2026-08-13

## Work State

| Field          | Value                                          |
| -------------- | ---------------------------------------------- |
| Latest release | v0.2.9 (crates.io: `apisync`)                  |
| License        | MIT OR Apache-2.0                              |
| Focus          | Universal API toolkit (REST/GraphQL/WebSocket) |

Progress: ████████░░ 90%

# Apisync

> API synchronization and integration platform

## Overview

Apisync provides automated API synchronization, versioning, and conflict resolution across distributed systems.

## Features

- **Schema Synchronization**: Auto-sync OpenAPI/GraphQL schemas
- **Version Management**: Track API versions and migrations
- **Conflict Resolution**: Intelligent merge strategies
- **Webhook Integration**: Real-time sync triggers
- **Audit Logging**: Complete change history

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Apisync Platform                           │
│                                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Schema    │  │   Sync      │  │  Conflict   │             │
│  │  Registry   │  │  Engine     │  │  Resolver   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│                                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │  Webhook    │  │   Audit     │  │   Version   │             │
│  │  Handler    │  │   Log       │  │   Manager   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
# Install
npm install -g @phenotype/apisync

# Initialize project
apisync init

# Sync APIs
apisync sync --source ./api-v1.yaml --target ./api-v2.yaml

# Start monitoring
apisync watch --config apisync.yaml
```

## Documentation

- [Specification](SPEC.md) - Technical details
- [Implementation Plan](PLAN.md) - Development roadmap

## License

MIT

/// @trace APIS-001

<!-- code-review signal 4 -->

<!-- code-review signal 5 -->

<!-- code-review signal 7 -->

<!-- code-review signal 8 -->

<!-- code-review signal 9 -->

<!-- code-review signal 10 -->

<!-- code-review signal 11 -->

<!-- code-review signal 12 -->

<!-- code-review signal 13 -->

<!-- code-review signal 14 -->

<!-- code-review signal 15 -->

<!-- code-review signal 16 -->

<!-- code-review signal 17 -->

<!-- code-review signal 18 -->

<!-- code-review signal 19 -->

<!-- code-review signal 20 -->

<!-- code-review signal 21 -->

<!-- code-review signal 22 -->

<!-- code-review signal 23 -->

<!-- code-review signal 24 -->

<!-- code-review signal 25 -->

<!-- code-review signal 26 -->

<!-- code-review signal 27 -->

<!-- code-review signal 28 -->

<!-- code-review signal 29 -->

<!-- code-review signal 30 -->

<!-- code-review signal 31 -->

<!-- code-review signal 32 -->

<!-- code-review signal 33 -->

<!-- code-review signal 34 -->

<!-- code-review signal 35 -->

<!-- code-review signal 36 -->

<!-- code-review signal 37 -->

<!-- code-review signal 38 -->

<!-- code-review signal 39 -->

<!-- code-review signal 40 -->

<!-- code-review signal 41 -->

<!-- code-review signal 42 -->

<!-- code-review signal 43 -->

<!-- code-review signal 44 -->

<!-- code-review signal 45 -->

<!-- code-review signal 46 -->

<!-- code-review signal 47 -->

<!-- code-review signal 48 -->

<!-- code-review signal 49 -->

<!-- code-review signal 50 -->

<!-- code-review signal 51 -->

<!-- code-review signal 52 -->

<!-- code-review signal 53 -->

<!-- code-review signal 54 -->

<!-- code-review signal 55 -->

<!-- code-review signal 56 -->

<!-- code-review signal 57 -->

<!-- code-review signal 58 -->

<!-- code-review signal 59 -->

<!-- code-review signal 60 -->

<!-- code-review signal 61 -->

<!-- code-review signal 62 -->

<!-- code-review signal 63 -->

<!-- code-review signal 64 -->

<!-- code-review signal 65 -->

<!-- code-review signal 66 -->

<!-- code-review signal 67 -->

<!-- code-review signal 68 -->

<!-- code-review signal 69 -->

<!-- code-review signal 70 -->

<!-- code-review signal 71 -->

<!-- code-review signal 72 -->

<!-- code-review signal 73 -->

<!-- code-review signal 74 -->

<!-- code-review signal 75 -->

<!-- code-review signal 76 -->

<!-- code-review signal 77 -->

<!-- code-review signal 78 -->

<!-- code-review signal 79 -->

<!-- code-review signal 80 -->

<!-- code-review signal 81 -->

<!-- code-review signal 82 -->

<!-- code-review signal 83 -->

<!-- code-review signal 84 -->

<!-- code-review signal 85 -->

<!-- code-review signal 86 -->

<!-- code-review signal 87 -->

<!-- code-review signal 88 -->

<!-- code-review signal 89 -->

<!-- code-review signal 90 -->

<!-- code-review signal 91 -->

<!-- code-review signal 92 -->
