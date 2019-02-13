#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod controllers;
pub mod models;
pub mod schema;
mod export;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::fairing::AdHoc;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

pub struct GmailPassword(String);

/* RUTAS
Cronologico (paginado) - DONE
Tag (paginado) - DONE
Post (actualizar formato) - DONE
Páginas - DONE
Contacto - DONE
RSS - DONE
Migrar imágenes - DONE
Sitemap (Transformación RDF?) - DONE
Robots.txt
Sistema Backups
Miniaturas Telegram, Facebook, Twitter - DONE
Favicon - DONE
RSS Programación (MailChimp) - DONE
Suscribirse FEED, MailChimp, Telegram
Compartir redes sociales
Subir imágenes
(RDFa, RDF/XML)
(OpenSearch)
(Creative Commons)
(Generar BibTeX)
(ActivityPub feed)
(resaltado sintaxis) - DONE
(SPARQL Endpoint? and individual RDF resources bajo demanda)
(una vez al dia, se reconstruye la base de datos RDF global) - DONE
(Comentarios, contacto, encuestas)
Admin - DONE
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
        let url = r.config().get_table("databases").unwrap().get("postgres_db").unwrap().get("url").unwrap();
        postgres.push_str(url.as_str().unwrap());
    }

    std::thread::spawn(move || {
        loop{
            export::export(&postgres);
            println!("Finished Exporting");
            std::thread::sleep(std::time::Duration::from_secs(60*60*24));
        }
    });
    

   
    r
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
        controllers::feed::feed,
        controllers::feed::feed_rss_xml,
        controllers::feed::sitemap,
        controllers::feed::programacion_rss,
        controllers::api::api,
    ])
    .mount("/static", StaticFiles::from("static"))
    .mount("/wp-content", StaticFiles::from("wp-content"))
    .attach(AdHoc::on_attach("GmailPassword",|rocket| {
            let gmail_password = rocket.config().get_str("gmail_password").unwrap().to_string();
            Ok(rocket.manage(GmailPassword(gmail_password)))
    }))
    .launch();
}
