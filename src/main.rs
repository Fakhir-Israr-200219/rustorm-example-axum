#[allow(unused_imports, dead_code)]
mod entities {
    include!("../.rustorm/generated/entities.rs");
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use entities::{
    CommentModel, Post, PostCreate, PostModel, Tag, TagCreate, User, UserCreate, UserModel,
    UserProfileModel, UserUpdate,
};
use rustorm::Entity;
use serde::Serialize;
use sqlx::PgPool;

// ==================== APP STATE ====================

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// ==================== RESPONSE DTOs ====================

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    email: String,
    place: Option<String>,
    age: i32,
    bio: Option<String>,
}

impl From<&UserModel> for UserResponse {
    fn from(user: &UserModel) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            place: user.place.clone(),
            age: user.age,
            bio: user.bio.clone(),
        }
    }
}

#[derive(Serialize)]
struct PostResponse {
    id: i32,
    title: String,
    content: Option<String>,
    published: bool,
    view_count: i32,
    user_id: i32,
}

impl From<&PostModel> for PostResponse {
    fn from(post: &PostModel) -> Self {
        Self {
            id: post.id,
            title: post.title.clone(),
            content: post.content.clone(),
            published: post.published,
            view_count: post.view_count,
            user_id: post.user_id,
        }
    }
}

#[derive(Serialize)]
struct CommentResponse {
    id: i32,
    body: String,
    post_id: i32,
    author_id: i32,
}

impl From<&CommentModel> for CommentResponse {
    fn from(comment: &CommentModel) -> Self {
        Self {
            id: comment.id,
            body: comment.body.clone(),
            post_id: comment.post_id,
            author_id: comment.author_id,
        }
    }
}

#[derive(Serialize)]
struct ProfileResponse {
    id: i32,
    user_id: i32,
    avatar: Option<String>,
    website: Option<String>,
}

impl From<&UserProfileModel> for ProfileResponse {
    fn from(profile: &UserProfileModel) -> Self {
        Self {
            id: profile.id,
            user_id: profile.user_id,
            avatar: profile.avatar.clone(),
            website: profile.website.clone(),
        }
    }
}

#[derive(Serialize)]
struct TagWithPostsResponse {
    id: i32,
    name: String,
    posts: Vec<PostResponse>,
}

// ==================== RELATION RESPONSE DTOs ====================

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
struct UserWithNestedRelationsResponse {
    #[serde(flatten)]
    user: UserResponse,
    posts: Vec<PostWithCommentsResponse>,
}

#[derive(Serialize)]
struct UserWithProfileResponse {
    #[serde(flatten)]
    user: UserResponse,
    profile: Option<ProfileResponse>,
}

#[derive(Serialize)]
struct PostWithUserResponse {
    #[serde(flatten)]
    post: PostResponse,
    user: Option<UserResponse>,
}

// ==================== HELPER ====================

fn db_err(e: impl std::fmt::Debug, context: &str) -> StatusCode {
    eprintln!("❌ [{context}] DB Error: {e:?}");
    StatusCode::INTERNAL_SERVER_ERROR
}

// ==================== USER CRUD ROUTES ====================

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<UserModel>>, StatusCode> {
    let users = User::find()
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "list_users"))?;
    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserModel>, StatusCode> {
    let users = User::find()
        .where_(User::id.eq(id))
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "get_user"))?;

    users
        .into_iter()
        .next()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<UserCreate>,
) -> Result<(StatusCode, Json<UserModel>), StatusCode> {
    eprintln!("📥 create_user input: {:?}", input);
    let user = User::create(&state.db, input)
        .await
        .map_err(|e| db_err(e, "create_user"))?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<UserUpdate>,
) -> Result<Json<UserModel>, StatusCode> {
    let user = User::update(&state.db, id, input)
        .await
        .map_err(|e| db_err(e, "update_user"))?;
    Ok(Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let affected = User::delete(&state.db, id)
        .await
        .map_err(|e| db_err(e, "delete_user"))?;

    if affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// ==================== POST CRUD ROUTES ====================

async fn list_posts(State(state): State<AppState>) -> Result<Json<Vec<PostModel>>, StatusCode> {
    let posts = Post::find()
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "list_posts"))?;
    Ok(Json(posts))
}

async fn create_post(
    State(state): State<AppState>,
    Json(input): Json<PostCreate>,
) -> Result<(StatusCode, Json<PostModel>), StatusCode> {
    eprintln!("📥 create_post input: {:?}", input);
    let post = Post::create(&state.db, input)
        .await
        .map_err(|e| db_err(e, "create_post"))?;
    Ok((StatusCode::CREATED, Json(post)))
}

// ==================== EAGER LOADING ROUTES ====================

async fn list_users_with_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserWithPostsResponse>>, StatusCode> {
    eprintln!("🔍 list_users_with_posts");
    let users = User::find()
        .with(User::posts)
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "users_with_posts"))?;

    eprintln!("✅ Found {} users with posts", users.len());
    let response = users
        .iter()
        .map(|user| UserWithPostsResponse {
            user: UserResponse::from(user),
            posts: user.posts.iter().map(PostResponse::from).collect(),
        })
        .collect();

    Ok(Json(response))
}

async fn list_users_with_posts_comments(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserWithNestedRelationsResponse>>, StatusCode> {
    eprintln!("🔍 list_users_with_posts_comments (NESTED)");
    let users = User::find()
        .with_nested(User::posts, |query| query.with(Post::comments))
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "users_with_posts_comments"))?;

    eprintln!("✅ Found {} users with nested posts+comments", users.len());
    let response = users
        .iter()
        .map(|user| UserWithNestedRelationsResponse {
            user: UserResponse::from(user),
            posts: user
                .posts
                .iter()
                .map(|post| PostWithCommentsResponse {
                    post: PostResponse::from(post),
                    comments: post.comments.iter().map(CommentResponse::from).collect(),
                })
                .collect(),
        })
        .collect();

    Ok(Json(response))
}

async fn list_users_with_profile(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserWithProfileResponse>>, StatusCode> {
    eprintln!("🔍 list_users_with_profile");
    let users = User::find()
        .with(User::profile)
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "users_with_profile"))?;

    eprintln!("✅ Found {} users with profile", users.len());
    let response = users
        .iter()
        .map(|user| UserWithProfileResponse {
            user: UserResponse::from(user),
            profile: user.profile.as_deref().map(ProfileResponse::from),
        })
        .collect();

    Ok(Json(response))
}

async fn list_posts_with_user(
    State(state): State<AppState>,
) -> Result<Json<Vec<PostWithUserResponse>>, StatusCode> {
    eprintln!("🔍 list_posts_with_user");
    let posts = Post::find()
        .with(Post::user)
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "posts_with_user"))?;

    eprintln!("✅ Found {} posts with user", posts.len());
    let response = posts
        .iter()
        .map(|post| PostWithUserResponse {
            post: PostResponse::from(post),
            user: post.user.as_deref().map(UserResponse::from),
        })
        .collect();

    Ok(Json(response))
}
// GET /tags/with-posts (M2M)
async fn list_tags_with_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<TagWithPostsResponse>>, StatusCode> {
    eprintln!("🔍 list_tags_with_posts (M2M)");
    let tags = Tag::find()
        .with(Tag::posts)
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "tags_with_posts"))?;

    eprintln!("✅ Found {} tags with posts", tags.len());
    let response = tags
        .iter()
        .map(|tag| TagWithPostsResponse {
            id: tag.id,
            name: tag.name.clone(),
            posts: tag
                .posts
                .iter()
                .map(|p| PostResponse::from(p.as_ref()))
                .collect(),
        })
        .collect();

    Ok(Json(response))
}
async fn list_tags(
    State(state): State<AppState>,
) -> Result<Json<Vec<entities::TagModel>>, StatusCode> {
    let tags = Tag::find()
        .all(&state.db)
        .await
        .map_err(|e| db_err(e, "list_tags"))?;
    Ok(Json(tags))
}

async fn create_tag(
    State(state): State<AppState>,
    Json(input): Json<TagCreate>,
) -> Result<(StatusCode, Json<entities::TagModel>), StatusCode> {
    eprintln!("📥 create_tag input: {:?}", input);
    let tag = Tag::create(&state.db, input)
        .await
        .map_err(|e| db_err(e, "create_tag"))?;
    Ok((StatusCode::CREATED, Json(tag)))
}
// ==================== DATABASE CONFIGURATION ====================

fn get_database_url() -> String {
    std::fs::read_to_string(".rustorm/.env")
        .ok()
        .and_then(|content| {
            content.lines().find_map(|line| {
                line.trim()
                    .strip_prefix("DATABASE_URL=")
                    .map(|value| value.trim().trim_matches('"').to_string())
            })
        })
        .or_else(|| std::env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL not found in .rustorm/.env or environment")
}

// ==================== MAIN ====================

#[tokio::main]
async fn main() {
    let db_url = get_database_url();
    eprintln!(
        "🔗 Connecting to: {}",
        db_url.replace(
            |c: char| c.is_ascii_alphanumeric() || c == ':' || c == '/' || c == '@' || c == '_',
            "*"
        )
    );

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    eprintln!("✅ Database connected!");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/posts", get(list_posts).post(create_post))
        .route("/users/with-posts", get(list_users_with_posts))
        .route(
            "/users/with-posts-comments",
            get(list_users_with_posts_comments),
        )
        .route("/users/with-profile", get(list_users_with_profile))
        .route("/posts/with-user", get(list_posts_with_user))
        .route("/tags/with-posts", get(list_tags_with_posts))
        .route("/tags", get(list_tags).post(create_tag))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind server");
    println!("\n🚀 RustORM Axum Integration Test Server");
    println!("📡 Listening on http://localhost:3000\n");

    axum::serve(listener, app)
        .await
        .expect("Axum server failed");
}
