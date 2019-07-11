use lettre::{SmtpClient, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::SmtpResult;
use lettre_email::{Email, mime::TEXT_PLAIN};
use crate::Config;
use rocket::State;

pub fn send_mail(to: String, from: String, subject: String, text: String, config: &State<Config>) -> SmtpResult {
    let email = Email::builder()
        .to(to.clone())
        .from(to)
        .reply_to(from)
        .subject(subject)
        .text(text)
        .build()
        .unwrap();
    
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        //.hello_name(ClientId::Domain("my.hostname.tld".to_string()))
        // Add credentials for authentication
        .credentials(Credentials::new("adrian.arroyocalle@gmail.com".to_string(), config.gmail_password.clone()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        .transport();
    mailer.send(email.into())
}