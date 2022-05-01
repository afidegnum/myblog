use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

//To be added based on special query

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "tags")]
pub struct Tags {
    pub id: i32,
    pub slug: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper)]
#[pg_mapper(table = "tags")]
pub struct CreateTags {
    pub slug: String,
    pub description: String,
    pub icon: String,
}
