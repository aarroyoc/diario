use rocket::response::Redirect;
use rocket::response::NamedFile;

#[get("/feed")]
pub fn feed() -> Redirect {
    Redirect::to("/rss.xml")
}

#[get("/rss.xml")]
pub fn feed_rss_xml() -> Option<NamedFile> {
    NamedFile::open("static/rss.xml").ok()
}

#[get("/sitemap.xml")]
pub fn sitemap() -> Option<NamedFile> {
    NamedFile::open("static/sitemap.xml").ok()
}

#[get("/category/programacion/feed")]
pub fn programacion_rss() -> Option<NamedFile> {
    NamedFile::open("static/programacion.rss.xml").ok()
}