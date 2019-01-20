#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod controllers;
pub mod models;
pub mod schema;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

/* RUTAS
Cronologico (paginado) - DONE
Tag (paginado) - DONE
Post (actualizar formato)
Páginas
Contacto
RSS
Migrar imágenes
Sitemap (Transformación RDF?)
Robots.txt
(RDFa, RDF/XML)
(OpenSearch)
(Creative Commons)
(Generar BibTeX)
(ActivityPub feed)
(resaltado sintaxis)
(SPARQL Endpoint? and individual RDF resources bajo demanda)
(una vez al dia, se reconstruye la base de datos RDF global)
(Comentarios, contacto, encuestas)
Admin - DONE
(API MicroPub? Python?)
(Analíticas)
(Print version, PDF)
Cookie: cZSiY8L2Tlpi9p+XEeAZ6f8uAIsJD5V3yXAuHGjojkk=
*/

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .attach(Database::fairing())
    .mount("/", routes![
        controllers::index::index,
        controllers::index::index_date,
        controllers::index::tag,
        controllers::index::tag_date,
        controllers::post::post,
        controllers::post::post_date,
        controllers::comment::post_comment,
        controllers::admin::list_posts,
        controllers::admin::login_get,
        controllers::admin::login_post,
        controllers::admin::post_view,
        controllers::admin::post_new,
        controllers::admin::post_edit,
        controllers::admin::post_view_new,
        controllers::admin::list_comments,
        controllers::admin::comment_approve,
        controllers::admin::comment_delete
    ])
    .mount("/static", StaticFiles::from("static"))
    .launch();
}
