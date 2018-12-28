#[get("/<year>/<month>/<day>/<slug>")]
pub fn post(year: u32, month: u8, day: u8, slug: String) -> &'static str {
    "Hello, POST"
}