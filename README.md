# Rust GraphQL DataLoader

A small GraphQL API built with Rust, `Axum`, `Tokio`, and `async-graphql`.

This project demonstrates how a DataLoader batches organization lookups to avoid the N+1 query problem when resolving `User.org`.

## Requirements

- Rust and Cargo
- `make`
- `curl`
- `jq`

## Run the project

The repository includes a `Makefile` with commands for running the server and testing the GraphQL queries.

Start the server:

```bash
make run
```

The GraphQL endpoint will be available at:

```text
http://localhost:3000/graphql
```

Keep the server running, then open a **second terminal** and run either query:

```bash
make get_user
```

Fetch all users, including their organizations:

```bash
make get_users
```

You can use these Makefile targets to test the API instead of manually preparing HTTP requests.

## DataLoader demonstration

The `get_users` query resolves each user's organization through an `async-graphql` DataLoader.

The sample data contains three users but only two distinct organizations. The loader batches the requested organization IDs and deduplicates repeated keys, so the organization lookup is performed as one batch rather than one lookup per user.

The DataLoader is created per GraphQL request, keeping its cache scoped to that request.

## Project structure

```text
src/
├── main.rs
├── db.rs
├── query_engine.rs
├── user_service.rs
└── organization_loader.rs
```

- `main.rs` — Axum server and GraphQL schema setup.
- `query_engine.rs` — Root GraphQL queries.
- `user_service.rs` — GraphQL user and organization types.
- `organization_loader.rs` — DataLoader batch implementation.
- `db.rs` — In-memory sample data and batch lookup.
