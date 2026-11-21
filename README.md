Bhai, yeh lo **updated README** jo aaj ke nested eager loading + DTO pattern milestone ko properly reflect karta hai. Purana basic CRUD content preserve kiya hai aur naye verified features add kiye hain:

```markdown
# RustORM Axum Example

A complete REST API example using [RustORM](https://github.com/Fakhir-Israr-200219/RustORM) with Axum.

This example demonstrates how RustORM's schema-driven code generation integrates seamlessly with Axum — including **nested eager loading**, **DTO response patterns**, and **wrapper types**. Generated entities include `Serialize` and `Deserialize` derives, so they work directly with Axum's `Json` extractor.

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
# .rustorm/schema.rustorm already has User, Post, Comment, UserProfile models

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
│   └── main.rs              ← Axum app with CRUD + eager loading routes
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
    id        Int       @id @default(autoincrement())
    name      String
    email     String    @unique
    place     String?
    age       Int
    bio       String?   @map("user_bio")
    createdAt DateTime  @default(now()) @map("created_at")

    posts     Post[]
    profile   UserProfile?
    comments  Comment[]

    @@map("users")
    @@index([email])
    @@index([name, age])
}

model UserProfile {
    id      Int     @id @default(autoincrement())
    user_id Int     @unique
    avatar  String?
    website String?

    user    User    @relation(fields: [user_id], references: [id], onDelete: Cascade)

    @@map("user_profiles")
}

model Post {
    id        Int       @id @default(autoincrement())
    title     String
    content   String?
    published Boolean   @default(false)
    viewCount Int       @default(0) @map("view_count")
    user_id   Int
    createdAt DateTime  @default(now()) @map("created_at")

    user      User      @relation(fields: [user_id], references: [id], onDelete: Cascade)
    comments  Comment[]

    @@map("posts")
    @@index([user_id])
}

model Comment {
    id        Int       @id @default(autoincrement())
    body      String
    post_id   Int
    author_id Int
    createdAt DateTime  @default(now()) @map("created_at")

    post      Post      @relation(fields: [post_id], references: [id], onDelete: Cascade)
    author    User      @relation(fields: [author_id], references: [id])

    @@map("comments")
    @@index([post_id])
}
```

## API Endpoints

### Basic CRUD

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/users` | List all users |
| `GET` | `/users/{id}` | Get user by ID |
| `POST` | `/users` | Create a new user |
| `PUT` | `/users/{id}` | Update a user |
| `DELETE` | `/users/{id}` | Delete a user |
| `GET` | `/posts` | List all posts |
| `POST` | `/posts` | Create a new post |

### Eager Loading ⭐

| Method | Path | Relation Type | Description |
|--------|------|---------------|-------------|
| `GET` | `/users/with-posts` | 1:N (OneToMany) | Users with their posts |
| `GET` | `/users/with-profile` | 1:1 (OneToOne) | Users with their profile |
| `GET` | `/users/with-posts-comments` | **2-level nested** | Users → Posts → Comments |
| `GET` | `/posts/with-user` | N:1 (ManyToOne) | Posts with their author |

## Usage Examples

### Create a User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Ali Khan",
    "email": "ali@example.com",
    "place": "Lahore",
    "age": 28,
    "bio": "Full-stack developer",
    "created_at": "2025-01-15T10:30:00"
  }'
```

### Create a Post

```bash
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Rust ORM Guide",
    "content": "Complete guide to building ORMs in Rust",
    "published": true,
    "view_count": 150,
    "user_id": 1,
    "created_at": "2025-01-15T12:00:00"
  }'
```

### List Posts with User (N:1 Eager Loading)

```bash
curl http://localhost:3000/posts/with-user
```

Response:
```json
[
  {
    "id": 1,
    "title": "Rust ORM Guide",
    "content": "Complete guide to building ORMs in Rust",
    "published": true,
    "view_count": 150,
    "user_id": 1,
    "user": {
      "id": 1,
      "name": "Ali Khan",
      "email": "ali@example.com",
      "place": "Lahore",
      "age": 28,
      "bio": "Full-stack developer"
    }
  }
]
```

### List Users with Posts + Comments (2-Level Nested) ⭐

```bash
curl http://localhost:3000/users/with-posts-comments
```

Response:
```json
[
  {
    "id": 1,
    "name": "Ali Khan",
    "email": "ali@example.com",
    "place": "Lahore",
    "age": 28,
    "bio": "Full-stack developer",
    "posts": [
      {
        "id": 1,
        "title": "Rust ORM Guide",
        "content": "Complete guide to building ORMs in Rust",
        "published": true,
        "view_count": 150,
        "user_id": 1,
        "comments": []
      },
      {
        "id": 2,
        "title": "Axum Integration",
        "content": "How to integrate RustORM with Axum",
        "published": true,
        "view_count": 89,
        "user_id": 1,
        "comments": []
      }
    ]
  }
]
```

### List Users with Profile (1:1 Eager Loading)

```bash
curl http://localhost:3000/users/with-profile
```

Response:
```json
[
  {
    "id": 1,
    "name": "Ali Khan",
    "email": "ali@example.com",
    "place": "Lahore",
    "age": 28,
    "bio": "Full-stack developer",
    "profile": null
  }
]
```

## Key Features Demonstrated

### 1. Nested Eager Loading

```rust
// Single-level
let users = User::find()
    .with(User::posts)
    .all(&state.db)
    .await?;

// 2-level nested
let users = User::find()
    .with_nested(User::posts, |q| q.with(Post::comments))
    .all(&state.db)
    .await?;
```

### 2. DTO Response Pattern

Clean API responses without exposing internal model structure:

```rust
#[derive(Serialize)]
struct UserWithPostsResponse {
    #[serde(flatten)]
    user: UserResponse,
    posts: Vec<PostResponse>,
}

#[derive(Serialize)]
struct PostWithCommentsResponse {
    #[serde(flatten)]
    post: PostResponse,
    comments: Vec<CommentResponse>,
}

#[derive(Serialize)]
struct UserWithNestedResponse {
    #[serde(flatten)]
    user: UserResponse,
    posts: Vec<PostWithCommentsResponse>,
}
```

### 3. Direct Generated Type Usage

Generated types include `serde::Serialize` and `serde::Deserialize` derives automatically:

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

### 4. Wrapper Types

RustORM provides `rustorm::DateTime`, `rustorm::Decimal`, and `rustorm::JsonValue` with built-in serde + sqlx support — no need to add `chrono`, `serde_json`, or `rust_decimal` as direct dependencies.

### 5. Async Compatibility

All runtime types implement `Send + Sync`, making them fully compatible with Axum's async handler requirements. The `RelationLoad` trait uses `impl Future + Send` for proper async framework integration.

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
axum = { version = "0.7", features = ["json", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "postgres",
    "macros",
    "chrono",
    "rust_decimal",
    "json"
] }
```

## Verified Status

| Feature | Status |
|---------|--------|
| Basic CRUD (Users, Posts) | ✅ |
| N:1 Eager Loading (`posts/with-user`) | ✅ |
| 1:N Eager Loading (`users/with-posts`) | ✅ |
| 1:1 Eager Loading (`users/with-profile`) | ✅ |
| **2-Level Nested Loading** (`users/with-posts-comments`) | ✅ |
| DTO Response Pattern | ✅ |
| Wrapper Types (`DateTime`) | ✅ |
| Send + Sync Compatibility | ✅ |
| 115 Runtime Tests | ✅ |

## License

MIT OR Apache-2.0
```

### Kya Naya Add Hua?

| Section | Change |
|---|---|
| Intro | Added nested eager loading, DTO pattern, wrapper types |
| Schema | Full 4-model schema with `@map`, relations, indexes |
| Endpoints Table | Split into CRUD + Eager Loading sections |
| Usage Examples | Added all 4 eager loading curl examples with responses |
| **NEW: Key Features** | Nested loading, DTO pattern, wrapper types, async compat |
| Dependencies | Added `macros`, `chrono`, `rust_decimal`, `json` features |
| **NEW: Verified Status** | Complete feature verification table |
