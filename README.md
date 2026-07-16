# ENDA Loyalty AI Platform

A monorepo containing the ENDA AI platform. The project consists of a web frontend, an AI backend, and a Model Context Protocol (MCP) server that communicates with the ENDA Loyalty backend.

---

## Architecture

```
                ┌──────────────────┐
                │    Frontend      │
                │   (Next.js)      │
                └────────┬─────────┘
                         │ HTTP
                         ▼
                ┌──────────────────┐
                │   AI Backend     │
                │     (Rust)       │
                └────────┬─────────┘
                         │ MCP
                         ▼
                ┌──────────────────┐
                │    MCP Server    │
                │     (Rust)       │
                └────────┬─────────┘
                         │ REST API
                         ▼
                 ENDA Loyalty Backend
```

---

# Repository Structure

```
backend/     Rust AI backend
frontend/    Next.js web application
mcp/         Rust MCP server
```

---

# Components

## Frontend

- Built with Next.js
- Provides the user interface
- Sends user prompts to the AI backend

Run:

```bash
cd frontend
npm install
npm run dev
```

---

## AI Backend

- Built in Rust
- Receives requests from the frontend
- Communicates with the MCP server
- Connects to AI providers (OpenRouter)

Run:

```bash
cd backend
cargo run
```

---

## MCP Server

- Built in Rust using RMCP
- Exposes ENDA Loyalty functionality as MCP tools
- Handles OAuth authentication
- Communicates with the ENDA REST API

Implemented tools:

- `enda_current_user`
- `enda_list_client_classes`
- `enda_list_rewards`
- `enda_list_regions`

---

Run:

```bash
cd mcp
cargo run
```

To use the MCP Inspector:

```bash
npx @modelcontextprotocol/inspector
```

---

# Technologies

## Backend

- Rust
- Tokio
- Reqwest
- OAuth2
- Serde

## MCP

- Rust
- RMCP
- OAuth2
- Reqwest
- Serde

## Frontend

- Next.js
- React
- TypeScript
- Tailwind CSS

---

# Environment Variables

Each project has its own configuration.

Create the required `.env` files inside the appropriate project directories.

Example:

```
mcp/.env
backend/.env
```

Refer to the `.env.example` files where available.

---

# Current Status

Current implementation includes:

- Repository restructured into a monorepo
- Next.js frontend
- Rust AI backend
- Rust MCP server
- OAuth authentication
- ENDA Loyalty API integration
- OpenRouter integration
