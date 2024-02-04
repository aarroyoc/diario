use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::schema::*;
use crate::Database;

use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
struct ListingPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct ListingPostTera {
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
pub async fn feed_rss_xml(conn: Database) -> Option<Template> {
    conn.run(|c| {
	let posts = post::table
	    .select((post::title, post::slug, post::content, post::date))
	    .filter(post::status.eq("published"))
	    .order(post::date.desc())
	    .limit(10)
	    .load::<ListingPost>(c)
	    .expect("Error loading posts");

	let posts: Vec<ListingPostTera> = posts
	    .into_iter()
	    .map(|p| {
		let date: DateTime<FixedOffset> =
		    DateTime::from_utc(p.date, FixedOffset::west_opt(0).unwrap());
		ListingPostTera {
		    title: p.title,
		    slug: p.slug,
		    content: p.content,
		    date_rfc822: date.to_rfc2822(),
		}
	    })
	    .collect();

	Some(Template::render("rss", ListTera { posts }))
    }).await
}

#[get("/sitemap.xml")]
pub async fn sitemap(conn: Database) -> Option<Template> {
    conn.run(|c| {
	let posts = post::table
	    .select(post::slug)
	    .filter(post::status.eq("published"))
	    .load::<String>(c)
	    .expect("Error loading posts");
	Some(Template::render("sitemap", ListString { posts }))
    }).await
}

#[get("/category/programacion/feed")]
pub async fn programacion_rss(conn: Database) -> Option<Template> {
    conn.run(|c| {
	let posts = tag::table
	    .inner_join(post::table)
	    .select((post::title, post::slug, post::content, post::date))
	    .filter(
		post::status
		    .eq("published")
		    .and(tag::name.eq("programacion")),
	    )
	    .order(post::date.desc())
	    .limit(10)
	    .load::<ListingPost>(c)
	    .expect("Error loading posts");
	let posts: Vec<ListingPostTera> = posts
	    .into_iter()
	    .map(|p| {
		let date: DateTime<FixedOffset> =
		    DateTime::from_utc(p.date, FixedOffset::west_opt(0).unwrap());
		ListingPostTera {
		    title: p.title,
		    slug: p.slug,
		    content: p.content,
		    date_rfc822: date.to_rfc2822(),
		}
	    })
	    .collect();
	Some(Template::render("rss", ListTera { posts }))
    }).await
}
