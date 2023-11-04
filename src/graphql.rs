// src/graphql.rs
use crate::models::{create_sample_users, User};
use async_graphql::{Context, Object, Result, Schema, ID};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        Ok(create_sample_users())
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<User>> {
        let users = create_sample_users();
        Ok(users.into_iter().find(|user| user.id == id))
    }
}

pub type SocialNetworkSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
