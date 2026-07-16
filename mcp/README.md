# ENDA MCP SERVER

A Rust-based MCP (Model Context Protocol) server that securely exposes selected ENDA Loyalty backend endpoints as MCP tools.

## Features

- Exposes ENDA backend endpoints as MCP tools
- Retrieves data from the ENDA REST API
- Supports testing with the MCP Inspector
- OAuth 2.0 Authorization Code Flow with PKCE
- Automatic access token refresh
- Secure communication with protected ENDA backend endpoints
- JSON serialization/deserialization using Serde

---

## Tools

At startup, the server downloads the My Enda OpenAPI document and exposes one
MCP tool for every documented HTTP operation. Tool names are stable, descriptive
method/path names such as `enda_get_client_classes` and
`enda_post_reward_requests_by_id_approve`.

Each tool's input schema is generated from the API contract: path, query and
header parameters are individually typed, while an endpoint's request payload
is a typed `body` object with the API's required fields, enums, nested schemas,
and validation rules. Responses are returned as formatted JSON so added backend
response fields remain available without updating the MCP server.

For multipart reward endpoints, provide `body.image` as a data URL such as
`data:image/png;base64,iVBORw0...`; it is sent to the API as the binary image part.

---

## Technologies and Crates

- Rust
- RMCP
- Tokio
- Reqwest
- OAuth2
- Serde
- dotenvy

---

## Running the Project

### Build the project

```bash
cargo build
```

### Run the server

```bash
cargo run
```

### Using the MCP Inspector

Launch the MCP Inspector:

```bash
npx @modelcontextprotocol/inspector
```

When creating a new connection:

- **Transport:** `STDIO`
- **Command:** Browse to your compiled Rust executable.

Example:

```text
C:\Users\<username>\Documents\RustCodes\enda-loyalty-mcp-server\target\debug\enda-loyalty-mcp-server.exe
```

Leave the **Arguments** field empty unless additional startup arguments are required.

---

## Testing

1. Build and run the MCP server.
2. Launch the MCP Inspector.
3. Connect to the server using the **STDIO** transport.
4. Select your compiled Rust executable as the command.
5. A browser window will automatically open for Keycloak authentication.
6. Log in using your ENDA account.
7. After successful authentication, a confirmation page will be displayed.
8. Execute any of the available MCP tools from the Inspector.

---

## Required Environment Variables

Create a `.env` file in the project root using the following variables:

```env
ENDA_API_BASE_URL=
ENDA_OPENAPI_URL=https://api.hederacourt.site/v3/api-docs
ENDA_KEYCLOAK_BASE=
ENDA_KEYCLOAK_REALM=
ENDA_KEYCLOAK_ID=
ENDA_REDIRECT_URI=
```

---

## Project Structure

```text
src/
├── api_client.rs    # Authenticated HTTP client for ENDA backend requests
├── auth.rs          # OAuth authentication and token management
├── config.rs        # Loads configuration from environment variables
├── main.rs          # Application entry point
├── models.rs        # API response models
├── server.rs        # MCP server and tool definitions
├── service.rs       # Backend service layer
```

---

## Additional Notes

- All backend requests are authenticated using OAuth 2.0 Bearer tokens.
- Access tokens are automatically refreshed when they expire.
- The server uses the Authorization Code Flow with PKCE for secure authentication.
- Create a `.env` file before running the project.

## Verifying the Server

The server is considered to be working correctly when the following can be successfully demonstrated:

- The MCP Inspector connects to the server over **STDIO**.
- A browser automatically opens and prompts the user to authenticate with Keycloak.
- After a successful login, the browser displays a confirmation page indicating the authentication was successful.
- All available MCP tools execute successfully from the MCP Inspector.
- The `enda_current_user` tool returns the currently authenticated user's information, role, and permissions.
- Access tokens are automatically refreshed when they expire without requiring the user to log in again.
