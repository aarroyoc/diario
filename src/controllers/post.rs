use crate::models::Comment;
use crate::schema::{comment, post, tag, username};
use crate::Database;

use rocket::request::FlashMessage;
use rocket_dyn_templates::Template;

use chrono::prelude::*;
use diesel::prelude::*;

use regex::Regex;

use crate::services::captcha::get_captcha;

#[derive(Queryable)]
struct PostViewDB {
    pub display_name: String,
    pub id: i32,
    pub content: String,
    pub title: String,
    pub excerpt: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct CommentViewTera {
    pub author: String,
    pub content: String,
    pub date: String,
    pub email_hash: String,
    pub url: Option<String>,
}

#[derive(Serialize)]
struct PostViewTera {
    pub display_name: String,
    pub content: String,
    pub title: String,
    pub date: NaiveDateTime,
    pub name: String,
    pub excerpt: String,
    pub img: String,
    pub id: i32,
    pub comments: Vec<CommentViewTera>,
    pub tags: Vec<String>,
    pub captcha_text: String,
    pub captcha_n: u8,
    pub sent_comment: bool,
}

#[get("/<slug>")]
pub async fn post_view(slug: &str, flash: Option<FlashMessage<'_>>, conn: Database) -> Option<Template> {
    let s = slug.to_string();
    let post = conn.run(move |c| {
	post::table
	    .inner_join(username::table)
	    .select((
		username::display_name,
		post::id,
		post::content,
		post::title,
		post::excerpt,
		post::date,
	    ))
	    .filter(post::slug.eq(&s).and(post::status.ne("hidden")))
            .first::<PostViewDB>(c)
    }).await;

    let s = slug.to_string();
    let comments = conn.run(move |c| {
	comment::table
	    .select((
		comment::id,
		comment::date,
		comment::content,
		comment::status,
		comment::post_id,
		comment::author_name,
		comment::author_mail,
		comment::author_url,
		comment::author_useragent,
	    ))
	    .inner_join(post::table)
	    .filter(post::slug.eq(&s).and(comment::status.eq("approved")))
	    .order(comment::date)
            .load::<Comment>(c)
    }).await;

    let s = slug.to_string();
    let tags = conn.run(move |c| {
	tag::table
	    .select(tag::name)
	    .inner_join(post::table)
	    .filter(post::slug.eq(&s))
	    .load::<String>(c)
            .unwrap()
    }).await;

    if let Ok(post) = post {
        if let Ok(comments) = comments {
            let mut comments_view = vec![];
            for comment in comments {
                let author_mail = comment
                    .author_mail
                    .unwrap_or_else(|| "NoExiste@YaYeYo.com".to_string());
                let digest = md5::compute(author_mail);
                comments_view.push(CommentViewTera {
                    author: comment.author_name,
                    content: comment.content,
                    date: comment.date.format("%e/%m/%Y").to_string(),
                    email_hash: format!("{:x}", digest),
                    url: comment.author_url,
                });
            }

            /* Find first image in post */
            let regex =
                Regex::new(r#"(https://files.adrianistan.eu/[^>]*.(png|jpeg|jpg|webp|gif))"#)
                    .unwrap();
            let captures = regex.captures(&post.content);
            let img = captures
                .and_then(|c| c.get(1))
                .map(|c| c.as_str())
                .unwrap_or("");
            let img = img.to_string();

            let (captcha_text, captcha_n) = get_captcha();

            let post = PostViewTera {
                display_name: post.display_name,
                content: post.content,
                title: post.title,
                name: slug.to_string(),
                id: post.id,
                img,
                excerpt: post.excerpt,
                date: post.date,
                comments: comments_view,
                tags,
                captcha_text: captcha_text.to_string(),
                captcha_n,
                sent_comment: flash.map_or(false, |msg| msg.kind() == "success"),
            };

            Some(Template::render("post", &post))
        } else {
            None
        }
    } else {
        None
    }
}

/* Be compatible with WordPress paths, but set canonical page to SLUG */
#[get("/<year>/<month>/<day>/<slug>")]
pub async fn post_date(
    year: i32,
    month: u32,
    day: u32,
    slug: &str,
    conn: Database,
) -> Option<Template> {
    let date = NaiveDate::from_ymd(year, month, day);
    let date = date.and_hms(0, 0, 0);
    let s = slug.to_string();
    let d = date.clone();
    let post_x = conn.run(move |c| {
	post::table
            .filter(post::slug.eq(&s).and(post::date.eq(d)))
            .first::<crate::models::Post>(c)
    }).await;
    if post_x.is_ok() {
        post_view(slug, None, conn).await
    } else {
        None
    }

    //let debug = diesel::debug_query::<diesel::pg::Pg, _>(&post);
    //println!("The insert query: {:?}", debug);
}
