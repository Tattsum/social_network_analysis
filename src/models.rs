// src/models.rs
use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
}

// ユーザーのデータを仮に作成する関数
pub fn create_sample_users() -> Vec<User> {
    vec![
        User {
            id: "1".into(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: "2".into(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ]
}
