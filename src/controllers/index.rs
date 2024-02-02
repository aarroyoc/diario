use crate::schema::*;
use crate::Database;

use rocket_dyn_templates::Template;

use chrono::prelude::*;
use diesel::prelude::*;

#[get("/")]
pub async fn index(conn: Database) -> Option<Template> {
    index_date(Utc::now().naive_local().timestamp(), conn).await
}

#[derive(Queryable, Serialize)]
struct ListingPost {
    pub title: String,
    pub name: String,
    pub excerpt: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct IndexPageData {
    pub posts: Vec<ListingPost>,
    pub last_date: Option<i64>,
    pub tag: Option<String>,
}

#[get("/?<date>")]
pub async fn index_date(date: i64, conn: Database) -> Option<Template> {
    let date = NaiveDateTime::from_timestamp(date, 0);

    let d = date.clone();
    let posts = conn.run(move |c| {
	post::table
	    .select((post::title, post::slug, post::excerpt, post::date))
	    .filter(post::date.lt(d).and(post::status.eq("published")))
	    .order(post::date.desc())
	    .limit(10)
	    .load::<ListingPost>(c)
            .expect("Error loading posts")
    }).await;

    let last_date = if posts.is_empty() {
        None
    } else {
        Some(posts[posts.len() - 1].date.timestamp())
    };

    let data = IndexPageData {
        posts,
        last_date,
        tag: None,
    };

    Some(Template::render("index", &data))
}

#[get("/tag/<tag>")]
pub async fn tag_view(tag: &str, conn: Database) -> Option<Template> {
    tag_date(tag, Utc::now().naive_local().timestamp(), conn).await
}

#[get("/tag/<tag>?<date>")]
pub async fn tag_date(tag: &str, date: i64, conn: Database) -> Option<Template> {
    let date = NaiveDateTime::from_timestamp(date, 0);

    let t = tag.to_string();
    let posts = conn.run(move |c| {
	tag::table
	    .inner_join(post::table)
	    .select((post::title, post::slug, post::excerpt, post::date))
	    .filter(
		post::date
		    .lt(date)
		    .and(post::status.eq("published"))
		    .and(tag::name.eq(&t)),
	    )
	    .order(post::date.desc())
	    .limit(10)
            .load::<ListingPost>(c)
    }).await;

    if let Ok(posts) = posts {
        let last_date = if posts.is_empty() {
            None
        } else {
            Some(posts[posts.len() - 1].date.timestamp())
        };

        let data = IndexPageData {
            posts,
            last_date,
            tag: Some(tag.to_string()),
        };
        return Some(Template::render("index", &data));
    }
    None
}
