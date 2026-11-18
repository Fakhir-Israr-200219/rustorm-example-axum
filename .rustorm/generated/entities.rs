use serde::{Serialize, Deserialize};
use rustorm::{
    entity::{Column, Entity, RelationKey, RelationLoader, SingleRelationLoader},
    field::Field,
    query::relation::{Relation, ManyToOne},
};
use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub place: Option<String>,
    pub email: String,
    #[sqlx(skip)]
    pub posts: Vec<PostModel>,
}

pub struct User;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub place: Option<String>,
    pub email: String,
}

impl rustorm::executor::InsertData<User> for UserCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["name", "place", "email"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.name).clone()),
            match &self.place {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
            rustorm::value::BindValue::String((&self.email).clone()),
        ]
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub place: Option<Option<String>>,
    pub email: Option<String>,
}

impl rustorm::executor::UpdateData<User> for UserUpdate {
    fn columns(&self) -> Vec<&'static str> {
        let mut columns = Vec::new();
        if self.name.is_some() {
            columns.push("name");
        }
        if self.place.is_some() {
            columns.push("place");
        }
        if self.email.is_some() {
            columns.push("email");
        }
        columns
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        let mut values = Vec::new();
        if let Some(value) = &self.name {
            values.push(rustorm::value::BindValue::String((value).clone()));
        }
        if let Some(value) = &self.place {
            match value {
                Some(value) => values.push(rustorm::value::BindValue::String((value).clone())),
                None => values.push(rustorm::value::BindValue::Null),
            }
        }
        if let Some(value) = &self.email {
            values.push(rustorm::value::BindValue::String((value).clone()));
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
        Column::new("place"),
        Column::new("email"),
    ];
}

impl User {
    #[allow(non_upper_case_globals)]
    pub const id: Field<Self, i32> = Field::new("id");

    #[allow(non_upper_case_globals)]
    pub const name: Field<Self, String> = Field::new("name");

    #[allow(non_upper_case_globals)]
    pub const place: Field<Self, Option<String>> = Field::new("place");

    #[allow(non_upper_case_globals)]
    pub const email: Field<Self, String> = Field::new("email");

    #[allow(non_upper_case_globals)]
    pub const posts: Relation<Self, Post> = Relation::new(Self::id, Post::user_id);

}

impl RelationKey for UserModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
            _ => None,
        }
    }
}

impl RelationLoader<PostModel> for UserModel {
    fn load_relation(&mut self, related: Vec<PostModel>) {
        self.posts = related;
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i32,
    #[sqlx(skip)]
    pub user: Option<Arc<UserModel>>,
}

pub struct Post;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostCreate {
    pub title: String,
    pub content: Option<String>,
    pub user_id: i32,
}

impl rustorm::executor::InsertData<Post> for PostCreate {
    fn columns(&self) -> &'static [&'static str] {
        &["title", "content", "user_id"]
    }

    fn values(&self) -> Vec<rustorm::value::BindValue> {
        vec![
            rustorm::value::BindValue::String((&self.title).clone()),
            match &self.content {
                Some(value) => rustorm::value::BindValue::String((value).clone()),
                None => rustorm::value::BindValue::Null,
            },
            rustorm::value::BindValue::I64(i64::from(*(&self.user_id))),
        ]
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub content: Option<Option<String>>,
    pub user_id: Option<i32>,
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
        if self.user_id.is_some() {
            columns.push("user_id");
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
        if let Some(value) = &self.user_id {
            values.push(rustorm::value::BindValue::I64(i64::from(*(value))));
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
        Column::new("user_id"),
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
    pub const user_id: Field<Self, i32> = Field::new("user_id");

    #[allow(non_upper_case_globals)]
    pub const user: Relation<Self, User, ManyToOne> = Relation::new(Self::user_id, User::id);

}

impl RelationKey for PostModel {
    fn relation_key(&self, column: Column) -> Option<i64> {
        match column.name() {
            "id" => Some(self.id as i64),
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

