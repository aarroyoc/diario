#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod controllers;
pub mod models;
pub mod schema;
mod services;

use rocket::fs::FileServer;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

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

#[launch]
fn rocket() -> _ {
    let r = rocket::build();

    r.attach(Template::fairing())
        .attach(Database::fairing())
        .mount(
            "/",
            routes![
                controllers::index::index,
                controllers::index::index_date,
                controllers::index::tag_view,
                controllers::index::tag_date,
                controllers::post::post_view,
                controllers::post::post_date,
                controllers::comment::post_comment,
                controllers::contact::get_contact,
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
        .mount("/static", FileServer::from("static"))
}
