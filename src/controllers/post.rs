use rocket_contrib::templates::Template;
use diesel::prelude::*;
use crate::Database;
use crate::schema::{post,username};

#[get("/<slug>")]
pub fn post(slug: String, conn: Database) -> Option<Template>{
    let post = post::table
        .filter(post::name.eq(slug))
        .first::<crate::models::Post>(&conn.0);

    if let Ok(post) = post {
        return Some(Template::render("post",&post));
    } else {
        return None;
    }
}

#[get("/<year>/<month>/<day>/<slug>")]
pub fn post_date(year: u32, month: u8, day: u8, slug: String, conn: Database) -> Option<Template> {

    // CHECK DATE ALSO
    let post = post::table
        .filter(post::name.eq(slug))
        .first::<crate::models::Post>(&conn.0);

    //let debug = diesel::debug_query::<diesel::pg::Pg, _>(&post);
    //println!("The insert query: {:?}", debug);

    if let Ok(post) = post {
        return Some(Template::render("post",&post));
    } else {
        return None;
    }
}