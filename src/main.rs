mod entities {
    include!("../.rustorm/generated/entities.rs");
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use entities::{Post, PostCreate, User, UserCreate, UserUpdate};
use rustorm::Entity;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// ==================== USER ROUTES ====================

async fn list_users(State(state): State<AppState>) -> Json<Vec<entities::UserModel>> {
    let users = User::find().all(&state.db).await.unwrap_or_default();
    Json(users)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<entities::UserModel>, StatusCode> {
    let users = User::find()
        .where_(User::id.eq(id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    users.into_iter().next()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<UserCreate>,
) -> Result<(StatusCode, Json<entities::UserModel>), StatusCode> {
    let user = User::create(&state.db, input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<UserUpdate>,
) -> Result<Json<entities::UserModel>, StatusCode> {
    let user = User::update(&state.db, id, input)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    match User::delete(&state.db, id).await {
        Ok(affected) if affected > 0 => StatusCode::NO_CONTENT,
        _ => StatusCode::NOT_FOUND,
    }
}

// ==================== POST ROUTES ====================

async fn list_posts(State(state): State<AppState>) -> Json<Vec<entities::PostModel>> {
    let posts = Post::find().all(&state.db).await.unwrap_or_default();
    Json(posts)
}

async fn create_post(
    State(state): State<AppState>,
    Json(input): Json<PostCreate>,
) -> Result<(StatusCode, Json<entities::PostModel>), StatusCode> {
    let post = Post::create(&state.db, input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(post)))
}

// ==================== MAIN ====================

#[tokio::main]
async fn main() {
    let db_url = std::fs::read_to_string(".rustorm/.env")
        .ok()
        .and_then(|content| {
            content.lines()
                .find(|line| line.starts_with("DATABASE_URL="))
                .map(|line| line.strip_prefix("DATABASE_URL=").unwrap().to_string())
        })
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL not found in .rustorm/.env or environment");

    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to database");

    println!("🚀 RustORM Axum Example");
    println!("📡 Server running at http://localhost:3000");
    println!();
    println!("Endpoints:");
    println!("  GET    /users          List all users");
    println!("  GET    /users/:id      Get user by ID");
    println!("  POST   /users          Create user");
    println!("  PUT    /users/:id      Update user");
    println!("  DELETE /users/:id      Delete user");
    println!("  GET    /posts          List all posts");
    println!("  POST   /posts          Create post");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/posts", get(list_posts).post(create_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}