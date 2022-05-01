use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

//To be added based on special query

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "posts")]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub intro: String,
    pub content: String,
    pub created_date: chrono::NaiveDate,
    pub category_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "posts")]
pub struct CreatePosts {
    pub title: String,
    pub slug: String,
    pub intro: String,
    pub content: String,
    pub created_date: chrono::NaiveDate,
    pub category_id: i32,
    pub user_id: i32,
}
