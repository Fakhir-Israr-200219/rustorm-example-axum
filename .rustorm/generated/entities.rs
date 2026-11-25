use rustorm::{
    entity::{Column, Entity, RelationKey, RelationLoader, SingleRelationLoader},
    field::Field,
    query::relation::{Relation, ManyToMany, ManyToOne, OneToOne},
};
use std::sync::Arc;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    #[sqlx(rename = "user_bio")]
    pub bio: Option<String>,
    pub created_at: rustorm::DateTime,
    #[sqlx(skip)]
    pub profile: Option<Arc<UserProfileModel>>,
    #[sqlx(skip)]
    pub posts: Vec<PostModel>,
}

pub struct User;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub bio: Option<String>,
}

impl rustorm::executor::InsertData<User> for UserCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["name", "email", "age", "user_bio"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.name).clone()),
            rustorm::value::BindValue::String((&self.email).clone()),
            rustorm::value::BindValue::I64(i64::from(*(&self.age))),
            match &self.bio {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub bio: Option<Option<String>>,
    pub created_at: Option<rustorm::DateTime>,
}

impl rustorm::executor::UpdateData<User> for UserUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.name.is_some() {
            columns.push("name");
        }
        if self.email.is_some() {
            columns.push("email");
        }
        if self.age.is_some() {
            columns.push("age");
        }
        if self.bio.is_some() {
            columns.push("user_bio");
        }
        if self.created_at.is_some() {
            columns.push("created_at");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.name {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        if let Some(value) = &self.email {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        if let Some(value) = &self.age {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.bio {
            match value {
                Some(value) => values.push(rustorm::value::BindValue::String((value).clone())),
                None => values.push(rustorm::value::BindValue::Null),
            }
        }
        if let Some(value) = &self.created_at {
            values.push(rustorm::value::BindValue::DateTime((value).into_inner()));
        }
        values
    }
}

impl Entity for User {
    type Model = UserModel;
    const TABLE: &'static str = "users";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("name"),
        Column::new("email"),
        Column::new("age"),
        Column::new("user_bio"),
        Column::new("created_at"),
    ];
}

impl User {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");

    #[allow(non_upper_case_globals)]
    pub const email: Field<Self, String> = Field::new("email");

    #[allow(non_upper_case_globals)]
    pub const age: Field<Self, i32> = Field::new("age");

    #[allow(non_upper_case_globals)]
    pub const bio: Field<Self, Option<String>> = Field::new("user_bio");

    #[allow(non_upper_case_globals)]
    pub const created_at: Field<Self, rustorm::DateTime> = Field::new("created_at");

    #[allow(non_upper_case_globals)]
    pub const profile: Relation<Self, UserProfile, OneToOne> = Relation::new(Self::id, UserProfile::user_id);

    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, Post> = Relation::new(Self::id, Post::user_id);

}

impl RelationKey for UserModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "age" => Some(self.age as i64),
            _ => None,
        }
    }
}

impl SingleRelationLoader<Arc<UserProfileModel>> for UserModel {
    fn load_relation(&mut self, related: Option<Arc<UserProfileModel>>) {
        self.profile = related;
    }
}

impl RelationLoader<PostModel> for UserModel {
    fn load_relation(&mut self, related: Vec<PostModel>) {
        self.posts = related;
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct UserProfileModel {
    pub id: i32,
    pub avatar: Option<String>,
    pub website: Option<String>,
    pub user_id: i32,
    #[sqlx(skip)]
    #[serde(skip)]
    pub user: Option<Arc<UserModel>>,
}

pub struct UserProfile;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct UserProfileCreate {
    pub avatar: Option<String>,
    pub website: Option<String>,
    pub user_id: i32,
}

impl rustorm::executor::InsertData<UserProfile> for UserProfileCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["avatar", "website", "user_id"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            match &self.avatar {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
            match &self.website {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
            rustorm::value::BindValue::I64(i64::from(*(&self.user_id))),
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct UserProfileUpdate {
    pub avatar: Option<Option<String>>,
    pub website: Option<Option<String>>,
    pub user_id: Option<i32>,
}

impl rustorm::executor::UpdateData<UserProfile> for UserProfileUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.avatar.is_some() {
            columns.push("avatar");
        }
        if self.website.is_some() {
            columns.push("website");
        }
        if self.user_id.is_some() {
            columns.push("user_id");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.avatar {
            match value {
                Some(value) => values.push(rustorm::value::BindValue::String((value).clone())),
                None => values.push(rustorm::value::BindValue::Null),
            }
        }
        if let Some(value) = &self.website {
            match value {
                Some(value) => values.push(rustorm::value::BindValue::String((value).clone())),
                None => values.push(rustorm::value::BindValue::Null),
            }
        }
        if let Some(value) = &self.user_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        values
    }
}

impl Entity for UserProfile {
    type Model = UserProfileModel;
    const TABLE: &'static str = "user_profiles";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("avatar"),
        Column::new("website"),
        Column::new("user_id"),
    ];
}

impl UserProfile {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const avatar: Field<Self, Option<String>> = Field::new("avatar");

    #[allow(non_upper_case_globals)]
    pub const website: Field<Self, Option<String>> = Field::new("website");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const user: Relation<Self, User, ManyToOne> = Relation::new(Self::user_id, User::id);

}

impl RelationKey for UserProfileModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

impl SingleRelationLoader<Arc<UserModel>> for UserProfileModel {
    fn load_relation(&mut self, related: Option<Arc<UserModel>>) {
        self.user = related;
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub published: bool,
    pub view_count: i32,
    pub user_id: i32,
    pub created_at: rustorm::DateTime,
    #[sqlx(skip)]
    #[serde(skip)]
    pub user: Option<Arc<UserModel>>,
    #[sqlx(skip)]
    pub comments: Vec<CommentModel>,
    #[sqlx(skip)]
    pub tags: Vec<Arc<TagModel>>,
}

pub struct Post;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct PostCreate {
    pub title: String,
    pub content: Option<String>,
    pub published: bool,
    pub view_count: i32,
    pub user_id: i32,
}

impl rustorm::executor::InsertData<Post> for PostCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["title", "content", "published", "view_count", "user_id"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.title).clone()),
            match &self.content {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
            rustorm::value::BindValue::Boolean(*(&self.published)),
            rustorm::value::BindValue::I64(i64::from(*(&self.view_count))),
            rustorm::value::BindValue::I64(i64::from(*(&self.user_id))),
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub content: Option<Option<String>>,
    pub published: Option<bool>,
    pub view_count: Option<i32>,
    pub user_id: Option<i32>,
    pub created_at: Option<rustorm::DateTime>,
}

impl rustorm::executor::UpdateData<Post> for PostUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.title.is_some() {
            columns.push("title");
        }
        if self.content.is_some() {
            columns.push("content");
        }
        if self.published.is_some() {
            columns.push("published");
        }
        if self.view_count.is_some() {
            columns.push("view_count");
        }
        if self.user_id.is_some() {
            columns.push("user_id");
        }
        if self.created_at.is_some() {
            columns.push("created_at");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.title {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        if let Some(value) = &self.content {
            match value {
                Some(value) => values.push(rustorm::value::BindValue::String((value).clone())),
                None => values.push(rustorm::value::BindValue::Null),
            }
        }
        if let Some(value) = &self.published {
            values.push(rustorm::value::BindValue::Boolean(*(value)));
        }
        if let Some(value) = &self.view_count {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.user_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.created_at {
            values.push(rustorm::value::BindValue::DateTime((value).into_inner()));
        }
        values
    }
}

impl Entity for Post {
    type Model = PostModel;
    const TABLE: &'static str = "posts";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("title"),
        Column::new("content"),
        Column::new("published"),
        Column::new("view_count"),
        Column::new("user_id"),
        Column::new("created_at"),
    ];
}

impl Post {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const title: Field<Self, String> = Field::new("title");

    #[allow(non_upper_case_globals)]
    pub const content: Field<Self, Option<String>> = Field::new("content");

    #[allow(non_upper_case_globals)]
    pub const published: Field<Self, bool> = Field::new("published");

    #[allow(non_upper_case_globals)]
    pub const view_count: Field<Self, i32> = Field::new("view_count");

    #[allow(non_upper_case_globals)]
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const created_at: Field<Self, rustorm::DateTime> = Field::new("created_at");

    #[allow(non_upper_case_globals)]
    pub const user: Relation<Self, User, ManyToOne> = Relation::new(Self::user_id, User::id);

    #[allow(non_upper_case_globals)]
    pub const tags: Relation<Self, Tag, ManyToMany> =
        Relation::<Self, Tag, ManyToMany>::many_to_many(
            Self::id,
            Tag::id,
            "post_tags",
            Column::new("post_id"),
            Column::new("tag_id"),
        );

    #[allow(non_upper_case_globals)]
    pub const comments: Relation<Self, Comment> = Relation::new(Self::id, Comment::post_id);

}

impl RelationKey for PostModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "view_count" => Some(self.view_count as i64),
            "user_id" => Some(self.user_id as i64),
            _ => None,
        }
    }
}

impl SingleRelationLoader<Arc<UserModel>> for PostModel {
    fn load_relation(&mut self, related: Option<Arc<UserModel>>) {
        self.user = related;
    }
}

impl RelationLoader<Arc<TagModel>> for PostModel {
    fn load_relation(&mut self, related: Vec<Arc<TagModel>>) {
        self.tags = related;
    }
}

impl RelationLoader<CommentModel> for PostModel {
    fn load_relation(&mut self, related: Vec<CommentModel>) {
        self.comments = related;
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct CommentModel {
    pub id: i32,
    pub body: String,
    pub post_id: i32,
    pub author_id: i32,
    pub created_at: rustorm::DateTime,
    #[sqlx(skip)]
    #[serde(skip)]
    pub post: Option<Arc<PostModel>>,
    #[sqlx(skip)]
    pub author: Option<Arc<UserModel>>,
}

pub struct Comment;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct CommentCreate {
    pub body: String,
    pub post_id: i32,
    pub author_id: i32,
}

impl rustorm::executor::InsertData<Comment> for CommentCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["body", "post_id", "author_id"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.body).clone()),
            rustorm::value::BindValue::I64(i64::from(*(&self.post_id))),
            rustorm::value::BindValue::I64(i64::from(*(&self.author_id))),
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct CommentUpdate {
    pub body: Option<String>,
    pub post_id: Option<i32>,
    pub author_id: Option<i32>,
    pub created_at: Option<rustorm::DateTime>,
}

impl rustorm::executor::UpdateData<Comment> for CommentUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.body.is_some() {
            columns.push("body");
        }
        if self.post_id.is_some() {
            columns.push("post_id");
        }
        if self.author_id.is_some() {
            columns.push("author_id");
        }
        if self.created_at.is_some() {
            columns.push("created_at");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.body {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        if let Some(value) = &self.post_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.author_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.created_at {
            values.push(rustorm::value::BindValue::DateTime((value).into_inner()));
        }
        values
    }
}

impl Entity for Comment {
    type Model = CommentModel;
    const TABLE: &'static str = "comments";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("body"),
        Column::new("post_id"),
        Column::new("author_id"),
        Column::new("created_at"),
    ];
}

impl Comment {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const body: Field<Self, String> = Field::new("body");

    #[allow(non_upper_case_globals)]
    pub const post_id: Field<Self, i32> = Field::new("post_id");

    #[allow(non_upper_case_globals)]
    pub const author_id: Field<Self, i32> = Field::new("author_id");

    #[allow(non_upper_case_globals)]
    pub const created_at: Field<Self, rustorm::DateTime> = Field::new("created_at");

    #[allow(non_upper_case_globals)]
    pub const post: Relation<Self, Post, ManyToOne> = Relation::new(Self::post_id, Post::id);

    #[allow(non_upper_case_globals)]
    pub const author: Relation<Self, User, ManyToOne> = Relation::new(Self::author_id, User::id);

}

impl RelationKey for CommentModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "post_id" => Some(self.post_id as i64),
            "author_id" => Some(self.author_id as i64),
            _ => None,
        }
    }
}

impl SingleRelationLoader<Arc<PostModel>> for CommentModel {
    fn load_relation(&mut self, related: Option<Arc<PostModel>>) {
        self.post = related;
    }
}

impl SingleRelationLoader<Arc<UserModel>> for CommentModel {
    fn load_relation(&mut self, related: Option<Arc<UserModel>>) {
        self.author = related;
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct TagModel {
    pub id: i32,
    pub name: String,
    #[sqlx(skip)]
    pub posts: Vec<Arc<PostModel>>,
}

pub struct Tag;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct TagCreate {
    pub name: String,
}

impl rustorm::executor::InsertData<Tag> for TagCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["name"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.name).clone()),
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct TagUpdate {
    pub name: Option<String>,
}

impl rustorm::executor::UpdateData<Tag> for TagUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.name.is_some() {
            columns.push("name");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.name {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        values
    }
}

impl Entity for Tag {
    type Model = TagModel;
    const TABLE: &'static str = "tags";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("name"),
    ];
}

impl Tag {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");

    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, Post, ManyToMany> =
        Relation::<Self, Post, ManyToMany>::many_to_many(
            Self::id,
            Post::id,
            "post_tags",
            Column::new("tag_id"),
            Column::new("post_id"),
        );

}

impl RelationKey for TagModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

impl RelationLoader<Arc<PostModel>> for TagModel {
    fn load_relation(&mut self, related: Vec<Arc<PostModel>>) {
        self.posts = related;
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize, sqlx::FromRow)]
pub struct PostTagModel {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
    #[sqlx(skip)]
    pub post: Option<Arc<PostModel>>,
    #[sqlx(skip)]
    pub tag: Option<Arc<TagModel>>,
}

pub struct PostTag;

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct PostTagCreate {
    pub post_id: i32,
    pub tag_id: i32,
}

impl rustorm::executor::InsertData<PostTag> for PostTagCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["post_id", "tag_id"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::I64(i64::from(*(&self.post_id))),
            rustorm::value::BindValue::I64(i64::from(*(&self.tag_id))),
        ]
    }
}

#[derive(Debug, rustorm::serde::Serialize, rustorm::serde::Deserialize)]
pub struct PostTagUpdate {
    pub post_id: Option<i32>,
    pub tag_id: Option<i32>,
}

impl rustorm::executor::UpdateData<PostTag> for PostTagUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.post_id.is_some() {
            columns.push("post_id");
        }
        if self.tag_id.is_some() {
            columns.push("tag_id");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.post_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        if let Some(value) = &self.tag_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
        }
        values
    }
}

impl Entity for PostTag {
    type Model = PostTagModel;
    const TABLE: &'static str = "post_tags";
    const COLUMNS: &'static [Column] = &[
        Column::new("id"),
        Column::new("post_id"),
        Column::new("tag_id"),
    ];
}

impl PostTag {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const post_id: Field<Self, i32> = Field::new("post_id");

    #[allow(non_upper_case_globals)]
    pub const tag_id: Field<Self, i32> = Field::new("tag_id");

    #[allow(non_upper_case_globals)]
    pub const post: Relation<Self, Post, ManyToOne> = Relation::new(Self::post_id, Post::id);

    #[allow(non_upper_case_globals)]
    pub const tag: Relation<Self, Tag, ManyToOne> = Relation::new(Self::tag_id, Tag::id);

}

impl RelationKey for PostTagModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            "post_id" => Some(self.post_id as i64),
            "tag_id" => Some(self.tag_id as i64),
            _ => None,
        }
    }
}

impl SingleRelationLoader<Arc<PostModel>> for PostTagModel {
    fn load_relation(&mut self, related: Option<Arc<PostModel>>) {
        self.post = related;
    }
}

impl SingleRelationLoader<Arc<TagModel>> for PostTagModel {
    fn load_relation(&mut self, related: Option<Arc<TagModel>>) {
        self.tag = related;
    }
}

