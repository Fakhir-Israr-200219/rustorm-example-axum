# RustORM Axum Example

A complete REST API example using [RustORM](https://github.com/Fakhir-Israr-200219/RustORM) with Axum.

This example demonstrates how RustORM's schema-driven code generation integrates seamlessly with Axum — generated entities include `Serialize` and `Deserialize` derives, so they work directly with Axum's `Json` extractor without any wrapper types.

## Quick Start

### Prerequisites

- Rust toolchain (`rustup`)
- PostgreSQL database running locally
- RustORM CLI installed (`cargo install --path .` from RustORM-CLI)

### Setup

```bash
# Clone and navigate
cd rustorm-example-axum

# Initialize RustORM (creates .rustorm/ folder with schema, .env, config)
rustorm init

# Edit the schema if needed
# .rustorm/schema.rustorm already has User and Post models

# Generate entities and migrations (or use watch mode)
rustorm dev
# Press Ctrl+C after initial generation

# Apply migration to database
rustorm migrate --apply

# Run the server
cargo run
```

The server starts at `http://localhost:3000`.

## Project Structure

```text
rustorm-example-axum/
├── Cargo.toml
├── src/
│   └── main.rs              ← Axum app with CRUD routes
└── .rustorm/
    ├── schema.rustorm        ← Database schema definition
    ├── .env                  ← Database URL (auto-generated)
    ├── config.toml           ← Structured config (auto-generated)
    ├── .gitignore            ← Protects .env from git
    ├── generated/
    │   └── entities.rs       ← Auto-generated (do not edit manually)
    └── migrations/
        ├── 0001_initial.sql  ← Generated migration SQL
        └── snapshot.json     ← Change detection state
```

## Schema

```rust
model User {
    id    Int    @id @default(autoincrement())
    name  String
    email String @unique
    posts Post[]
}

model Post {
    id      Int    @id @default(autoincrement())
    title   String
    content String?
    user_id Int
    user    User @relation(fields: [user_id], references: [id])
}
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/users` | List all users |
| `GET` | `/users/{id}` | Get user by ID |
| `POST` | `/users` | Create a new user |
| `PUT` | `/users/{id}` | Update a user |
| `DELETE` | `/users/{id}` | Delete a user |
| `GET` | `/posts` | List all posts |
| `POST` | `/posts` | Create a new post |

## Usage Examples

### Create a User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Fakhir", "email": "fakhir@example.com"}'
```

Response:
```json
{
  "id": 1,
  "name": "Fakhir",
  "email": "fakhir@example.com"
}
```

### List Users

```bash
curl http://localhost:3000/users
```

### Update a User

```bash
curl -X PUT http://localhost:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name"}'
```

### Delete a User

```bash
curl -X DELETE http://localhost:3000/users/1
```

### Create a Post

```bash
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{"title": "First Post", "content": "Hello World", "user_id": 1}'
```

### List Posts

```bash
curl http://localhost:3000/posts
```

## Key Takeaway

Generated types (`UserModel`, `UserCreate`, `UserUpdate`, `PostModel`, `PostCreate`) include `serde::Serialize` and `serde::Deserialize` derives automatically. This means:

- **No wrapper types needed** — use generated structs directly in Axum handlers
- **No manual `From` implementations** — `Json<UserCreate>` works out of the box
- **No separate DTO layer** — generated code is production-ready

```rust
// ✅ Direct usage — no boilerplate
async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<UserCreate>,       // ← Generated type, works directly
) -> Result<(StatusCode, Json<UserModel>), StatusCode> {
    let user = User::create(&state.db, input).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(user))) // ← UserModel serializes automatically
}
```

## Development Workflow

```bash
# Start watch mode (auto-regenerates on schema change)
rustorm dev

# Edit .rustorm/schema.rustorm in your editor
# Save → entities and migrations regenerate automatically

# Apply updated migration
rustorm migrate --apply

# Restart server
cargo run
```

## Configuration

Database URL is read automatically from `.rustorm/.env`:

```text
DATABASE_URL=postgres://postgres:admin@localhost/rustorm_gen
```

Priority order: CLI argument > environment variable > `.env` > `config.toml`

## Dependencies

```toml
[dependencies]
rustorm = { path = "../Rust-ORM/rustorm" }
axum = { version = "0.7", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
```

## License

MIT OR Apache-2.0
```

---

## Summary

Yeh README cover karta hai:
- ✅ Quick start (5 commands se setup)
- ✅ Project structure explanation
- ✅ Schema reference
- ✅ All 7 API endpoints with curl examples
- ✅ **Key takeaway** — why no wrapper types are needed
- ✅ Development workflow with watch mode
- ✅ Configuration priority
- ✅ Dependencies list
