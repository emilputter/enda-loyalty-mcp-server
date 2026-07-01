# ENDA MCP SERVER

A Rust based MCP server that uses selected ENDA Loyalty backend endpoints as MCP tools.

## Features

- Exposes ENDA backend endpoints as MCP tools
- Retrieves data from the ENDA REST API
- Supports testing with the MCP Inspector
- JSON serialization/deserialization using Serde

## Implemented Tools

- enda_list_client_classes
- enda_list_rewards
- enda_list_regions

## Technologies and crates

- Rust
- RMCP
- Tokio
- Reqwest
- Serde
- SQLx
- PostgreSQL

## Running the Project
```bash
cargo build
cargo run
```

## How to test

- The project can be tested using the MCP Inspector by connecting to the server over **STDIO**

## Project structure

```bash
src/
database.rs -> PostgreSQL connection utilities
main.rs -> Entry point for application
models.rs -> Response models for API
server.rs -> MCP tool and server implementation
service.rs -> Backend API communication
```
## Additional Notes

- Current MCP tools retrieve data from the ENDA backend REST API
- PostgreSQL module has been kept for potential future use
- Mods.rs is not currently used and can be used future use