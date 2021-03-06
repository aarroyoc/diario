#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod controllers;
mod export;
pub mod models;
pub mod schema;
mod services;

use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

pub struct Config {
    pub gmail_password: String,
    pub hostname: String,
}

/* RUTAS
Robots.txt
(RDFa, RDF/XML)
(OpenSearch)
(Creative Commons)
(Generar BibTeX)
(ActivityPub feed)
(SPARQL Endpoint? and individual RDF resources bajo demanda)
(una vez al dia, se reconstruye la base de datos RDF global) - Hacerlo en Python
(Comentarios, contacto, encuestas)
(API MicroPub? Python?)
(Analíticas)
(AdSense) - DONE
(Print version, PDF)
Cookie: cZSiY8L2Tlpi9p+XEeAZ6f8uAIsJD5V3yXAuHGjojkk=
DOCUMENTAR TODO
*/

fn main() {
    let r = rocket::ignite();
    let mut postgres = String::new();
    {
        let url = r
            .config()
            .get_table("databases")
            .unwrap()
            .get("postgres_db")
            .unwrap()
            .get("url")
            .unwrap();
        postgres.push_str(url.as_str().unwrap());
    }

    r.attach(Template::fairing())
        .attach(Database::fairing())
        .mount(
            "/",
            routes![
                controllers::index::index,
                controllers::index::index_date,
                controllers::index::tag,
                controllers::index::tag_date,
                controllers::post::post,
                controllers::post::post_date,
                controllers::comment::post_comment,
                controllers::contact::get_contact,
                controllers::contact::post_contact,
                controllers::admin::list_posts,
                controllers::admin::login_get,
                controllers::admin::login_post,
                controllers::admin::post_view,
                controllers::admin::post_new,
                controllers::admin::post_edit,
                controllers::admin::post_view_new,
                controllers::admin::list_comments,
                controllers::admin::comment_approve,
                controllers::admin::comment_delete,
                controllers::admin::comment_approve_response,
                controllers::feed::feed,
                controllers::feed::feed_rss_xml,
                controllers::feed::sitemap,
                controllers::feed::programacion_rss
            ],
        )
        .mount("/static", StaticFiles::from("static"))
        .attach(AdHoc::on_attach("ConfigState", |rocket| {
            let gmail_password = rocket
                .config()
                .get_str("gmail_password")
                .unwrap()
                .to_string();
            let hostname = rocket.config().get_str("host").unwrap().to_string();
            Ok(rocket.manage(Config {
                hostname,
                gmail_password,
            }))
        }))
        .launch();
}
