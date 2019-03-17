use std::io::Read;
use std::process::*;

use rocket::data::{self, FromDataSimple};
use rocket::http::{ContentType, Status};
use rocket::response::content::Content;
use rocket::{Data, Outcome, Outcome::*, Request};

const LIMIT: u64 = 1024;

pub struct Sparql(String);

impl FromDataSimple for Sparql {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let sparql_ct = ContentType::new("application", "sparql-query");
        if req.content_type() != Some(&sparql_ct) {
            return Outcome::Forward(data);
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(Sparql(string))
    }
}

#[post("/api", data = "<sparql>")]
pub fn api(sparql: Sparql) -> Option<Content<String>> {
    let cmd = Command::new("python3")
        .arg("scripts/sparql.py")
        .arg(sparql.0)
        .output()
        .expect("Failed to execute SPARQL query");
    if let Ok(out) = String::from_utf8(cmd.stdout) {
        let ct = ContentType::new("application", "sparql-results+json");
        Some(Content(ct, out))
    } else {
        None
    }
}
