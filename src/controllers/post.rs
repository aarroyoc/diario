use crate::Database;
use crate::schema::{post,username,comment,tag};
use crate::models::Comment;

use rocket_contrib::templates::Template;

use diesel::prelude::*;
use chrono::prelude::*;



#[derive(Queryable)]
struct PostViewDB {
    pub display_name: String,
    pub id: i32,
    pub content: String,
    pub title: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct CommentViewTera {
    pub author: String,
    pub content: String,
    pub date: String,
    pub email_hash: String,
}

#[derive(Serialize)]
struct PostViewTera {
    pub display_name: String,
    pub content: String,
    pub title: String,
    pub date: String,
    pub name: String,
    pub id: i32,
    pub comments: Vec<CommentViewTera>,
    pub tags: Vec<String>,
}

#[get("/<slug>")]
pub fn post(slug: String, conn: Database) -> Option<Template>{
    let post = post::table
        .inner_join(username::table)
        .select((
            username::display_name,
            post::id,
            post::content,
            post::title,
            post::date
        ))
        .filter(post::slug.eq(&slug))
        .first::<PostViewDB>(&conn.0);

    let comments = comment::table
        .select((
            comment::id,
            comment::date,
            comment::content,
            comment::status,
            comment::post_id,
            comment::author_name,
            comment::author_mail,
            comment::author_url,
            comment::author_useragent
        ))
        .inner_join(post::table)
        .filter(post::slug.eq(&slug).and(comment::status.eq("approved")))
        .load::<Comment>(&conn.0);

    let tags = tag::table
        .select(
            tag::name
        )
        .inner_join(post::table)
        .filter(post::slug.eq(&slug))
        .load::<String>(&conn.0)
        .unwrap();
    

    if let Ok(post) = post {
        if let Ok(comments) = comments {
            let mut comments_view = vec![];
            for comment in comments {
                let author_mail = comment.author_mail.unwrap_or_else(|| "NoExiste@YaYeYo.com".to_string());
                let digest = md5::compute(author_mail);
                comments_view.push(CommentViewTera{
                    author: comment.author_name,
                    content: comment.content,
                    date: comment.date.format("%e/%m/%Y").to_string(),
                    email_hash: format!("{:x}", digest)
                });
            }
            let post = PostViewTera{
                display_name: post.display_name,
                content: post.content,
                title: post.title,
                name: slug,
                id: post.id,
                date: post.date.format("%e/%m/%Y").to_string(),
                comments: comments_view,
                tags
            };

            return Some(Template::render("post",&post));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

/* Be compatible with WordPress paths, but set canonical page to SLUG */
#[get("/<year>/<month>/<day>/<slug>")]
pub fn post_date(year: i32, month: u32, day: u32, slug: String, conn: Database) -> Option<Template> {

    let date = NaiveDate::from_ymd(year, month, day);
    let date = date.and_hms(0, 0, 0);
    let post_x = post::table
        .filter(post::slug.eq(&slug).and(post::date.eq(date)))
        .first::<crate::models::Post>(&conn.0);
    if post_x.is_ok() {
        post(slug,conn)
    } else {
        None
    }

    //let debug = diesel::debug_query::<diesel::pg::Pg, _>(&post);
    //println!("The insert query: {:?}", debug);
}