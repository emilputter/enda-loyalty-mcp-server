# ENDA MCP SERVER

A Rust based MCP server that uses selected ENDA Loyalty backend endpoints as MCP tools.

## Features

- Exposes ENDA backend endpoints as MCP tools
- Retrieves data from the ENDA REST API
- Supports testing with the MCP Inspector
- JSON serialization/deserialization using Serde
- OAuth 2.0 Authorization Code Flow with PKCE
- Automatic access token refresh
- Secure communication with protected ENDA backend endpoints

## Implemented Tools

- enda_list_client_classes
- enda_list_rewards
- enda_list_regions
- enda_current_user

## Technologies and crates

- Rust
- RMCP
- Tokio
- Reqwest
- OAuth2
- Serde
- dotenvy

## Running the Project
```bash
cargo build
cargo run

Using the MCP inspector tool without running simply use:
npx.cmd @modelcontextprotocol/inspector  
```

## How to test

- The project can be tested using the MCP Inspector by connecting to the server over **STDIO**
- Login into the browser when promted
- After login will a custom browser appear acknowleging the success

## Required enviroment variables:
```env
ENDA_API_BASE_URL=
ENDA_KEYCLOAK_BASE=
ENDA_KEYCLOAK_REALM=
ENDA_KEYCLOAK_ID=
ENDA_REDIRECT_URI=
```

## Project structure

```bash
src/
api_clients.rs -> Shared HTTP client for communication with ENDA backend
auth.rs -> OAuth authentication and token management
config.rs -> Loads config from environment variables
main.rs -> Entry point for application
models.rs -> Response models for API
server.rs -> MCP tool and server implementation
service.rs -> Backend API communication
```

## Additional Notes

- Current MCP tools retrieve data from the ENDA backend REST API
- PostgreSQL module has been kept for potential future use
- Create a `.env` file om the project route (ENDA_API_BASE_URL=YourLink)