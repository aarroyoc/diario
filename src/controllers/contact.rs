use crate::services::mail::send_mail;
use crate::Config;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

#[get("/contacto")]
pub fn get_contact() -> Template {
    let m: HashMap<i32, i32> = HashMap::new();
    Template::render("contact", &m)
}

#[derive(FromForm)]
pub struct ContactForm {
    email: String,
    title: String,
    content: String,
}

#[post("/contacto", data = "<contact>")]
pub fn post_contact(contact: Form<ContactForm>, config: State<Config>) -> Redirect {
    /* Me he dado cuenta que la mayoría de spam bots no rellenan el campo title */
    if contact.title.is_empty() {
        return Redirect::to("/");
    }

    let status = send_mail(
        "adrian.arroyocalle@gmail.com".to_owned(),
        contact.email.clone(),
        contact.title.clone(),
        contact.content.clone(),
        &config,
    );
    if let Err(msg) = status {
        println!("{}", msg);
    }

    Redirect::to("/")
}
