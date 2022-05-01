use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

//To be added based on special query

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "category")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "category")]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
}
