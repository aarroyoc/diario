use crate::schema::comment;
use crate::Database;
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

use diesel::dsl::insert_into;

#[derive(FromForm)]
pub struct CommentForm {
    name: String,
    mail: String,
    url: String,
    slug: String,
    content: String,
    post_id: String,
    captcha_user: u8,
    captcha_n: u8,
}

#[derive(Insertable)]
#[table_name = "comment"]
pub struct CommentInsert {
    pub date: NaiveDateTime,
    pub content: String,
    pub status: String,
    pub post_id: i32,
    pub author_name: String,
    pub author_mail: Option<String>,
    pub author_url: Option<String>,
    pub author_useragent: Option<String>,
}

#[post("/comment", data = "<comment>")]
pub fn post_comment(comment: Form<CommentForm>, conn: Database) -> Flash<Redirect> {
    if comment.captcha_user != comment.captcha_n {
        return Flash::error(Redirect::to("/"), "captcha_fail");
    }

    let res = insert_into(comment::table)
        .values(CommentInsert {
            date: Utc::now().naive_local(),
            content: comment.content.clone(),
            status: "pending".to_string(),
            post_id: comment.post_id.parse::<i32>().unwrap(),
            author_name: comment.name.clone(),
            author_mail: if comment.mail == "" {
                None
            } else {
                Some(comment.mail.clone())
            },
            author_url: if comment.url == "" {
                None
            } else {
                Some(comment.url.clone())
            },
            author_useragent: None,
        })
        .execute(&conn.0);

    if let Err(err) = res {
        eprintln!("error adding tags: {:?}", err);
        return Flash::error(Redirect::to("/"), "error");
    }

    Flash::success(Redirect::to(format!("/{}", comment.slug)), "posted!")
}
