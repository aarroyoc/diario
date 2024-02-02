use rocket_dyn_templates::Template;

use std::collections::HashMap;

#[get("/contacto")]
pub fn get_contact() -> Template {
    let m: HashMap<i32, i32> = HashMap::new();
    Template::render("contact", &m)
}
