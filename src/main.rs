#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;

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
*/

fn main() {
    rocket::ignite()
    .mount("/", routes![
        controllers::index::index,
        controllers::post::post
    ])
    .launch();
}
