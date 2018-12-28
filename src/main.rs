#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod controllers;
pub mod models;
pub mod schema;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

/* RUTAS
Cronologico (paginado)
Tag (paginado)
Post
Páginas
Contacto
RSS
Sitemap
Robots.txt
(RDFa, RDF/XML)
(Comentarios, contacto, encuestas)
Admin
(API MicroPub? Python?)
(Analíticas)
Cookie: cZSiY8L2Tlpi9p+XEeAZ6f8uAIsJD5V3yXAuHGjojkk=
*/

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .attach(Database::fairing())
    .mount("/", routes![
        controllers::index::index,
        controllers::index::index_page,
        controllers::post::post
    ])
    .mount("/static", StaticFiles::from("/static"))
    .launch();
}
