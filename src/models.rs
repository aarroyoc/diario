use diesel::{Queryable,deserialize::QueryableByName,associations};
use std::time::SystemTime;
use diesel::prelude::*;


#[derive(Queryable)]
pub struct Username {
    pub id: i32,
    pub password: String,
    pub email: String,
    pub display_name: String
}

#[derive(Queryable,Serialize)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub date: SystemTime,
    pub content: String,
    pub title: String,
    pub excerpt: String,
    pub status: String,
    pub comment_status: String,
    pub name: String,
    pub modified: SystemTime,
}