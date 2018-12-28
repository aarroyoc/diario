use rocket_contrib::templates::Template;
use std::collections::HashMap;
use diesel::sql_query;
use diesel::RunQueryDsl;
use crate::Database;

#[get("/")]
pub fn index(conn: Database) -> Template {
    let mut context: HashMap<String,String> = HashMap::new();
    //let posts: Vec<crate::models::Post> = sql_query("SELECT * FROM post ORDER BY id").load(&conn).unwrap();
    let posts = crate::schema::post::table.load::<crate::models::Post>(&conn.0).expect("Error loading posts");
    for post in posts {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    Template::render("index",&context)
}

#[get("/page/<page>")]
pub fn index_page(page: u32) -> Option<Template> {
    let mut context: HashMap<String,String> = HashMap::new();
    Some(Template::render("index",&context))
}