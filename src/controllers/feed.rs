use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::Database;
use crate::schema::*;

use diesel::prelude::*;
use chrono::prelude::*;

#[derive(Queryable)]
struct ListingPost{
    pub title: String,
    pub slug: String,
    pub content: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct ListingPostTera{
    pub title: String,
    pub slug: String,
    pub content: String,
    pub date_rfc822: String,
}

#[derive(Serialize)]
struct ListTera {
    pub posts: Vec<ListingPostTera>,
}

#[derive(Serialize)]
struct ListString {
    pub posts: Vec<String>,
}

#[get("/feed")]
pub fn feed() -> Redirect {
    Redirect::to("/rss.xml")
}

#[get("/rss.xml")]
pub fn feed_rss_xml(conn: Database) -> Option<Template> {

    let posts = post::table
        .select((
            post::title,
            post::slug,
            post::content,
            post::date
        ))
        .filter(post::status.eq("published"))
        .order(post::date.desc())
        .load::<ListingPost>(&conn.0)
        .expect("Error loading posts");

    let posts: Vec<ListingPostTera> = posts.into_iter().map(|p|{
        let date: DateTime<FixedOffset> = DateTime::from_utc(p.date,FixedOffset::west_opt(0).unwrap());
        ListingPostTera{
            title: p.title,
            slug: p.slug,
            content: p.content,
            date_rfc822: date.to_rfc2822(),
        }
    }).collect();

    Some(Template::render("rss",ListTera{ posts }))
}

#[get("/sitemap.xml")]
pub fn sitemap(conn: Database) -> Option<Template> {
    let posts = post::table
        .select(post::slug)
        .filter(post::status.eq("published"))
        .load::<String>(&conn.0)
        .expect("Error loading posts");
    Some(Template::render("sitemap",ListString{ posts }))
}

#[get("/category/programacion/feed")]
pub fn programacion_rss(conn: Database) -> Option<Template> {
    let posts = tag::table
        .inner_join(post::table)
        .select((
            post::title,
            post::slug,
            post::content,
            post::date
        ))
        .filter(post::status.eq("published")
            .and(tag::name.eq("programacion")))
        .order(post::date.desc())
        .load::<ListingPost>(&conn.0)
        .expect("Error loading posts");
    let posts: Vec<ListingPostTera> = posts.into_iter().map(|p|{
        let date: DateTime<FixedOffset> = DateTime::from_utc(p.date,FixedOffset::west_opt(0).unwrap());
        ListingPostTera{
            title: p.title,
            slug: p.slug,
            content: p.content,
            date_rfc822: date.to_rfc2822(),
        }
    }).collect();
    Some(Template::render("rss",ListTera{ posts }))
}