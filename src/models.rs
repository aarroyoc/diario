use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::*;

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
    pub date: NaiveDateTime,
    pub content: String,
    pub title: String,
    pub excerpt: String,
    pub status: String, /* draft, published */
    pub comment_status: String,
    pub name: String,
}

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub date: NaiveDateTime,
    pub content: String,
    pub status: String, /* pending, approved, deleted */
    pub post_id: i32,
    pub author_name: String,
    pub author_mail: Option<String>,
    pub author_url: Option<String>,
    pub author_useragent: Option<String>,
}