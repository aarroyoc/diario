use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;
use std::process::{Command,Stdio};
use std::io::Write;
use rocket::request::Form;
use rocket::State;
use crate::GmailPassword;

#[get("/contacto")]
pub fn get_contact() -> Template {
    let m: HashMap<i32,i32> = HashMap::new();
    Template::render("contact",&m)
}

#[derive(FromForm)]
pub struct ContactForm {
    email: String,
    title: String,
    content: String,
}

#[post("/contacto",data="<contact>")]
pub fn post_contact(contact: Form<ContactForm>, gmail_password: State<GmailPassword> ) -> Redirect{
    /* Me he dado cuenta que la mayoría de spam bots no rellenan el campo title */
    if contact.title.is_empty() {
        return Redirect::to("/");
    }

    let mut cmd = Command::new("python3")
        .arg("scripts/send_email.py")
        .arg(&contact.title)
        .arg(&contact.email)
        .arg(&gmail_password.0)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to send mail");
    let mut stdin = cmd.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(contact.content.as_bytes()).expect("Failed to write to stdin");

    Redirect::to("/")
}