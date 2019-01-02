use rocket_contrib::templates::Template;
use std::collections::HashMap;
use diesel::sql_query;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::Database;
use crate::schema::{post,username};

#[derive(Queryable)]
struct PostAuthor {
    pub email: String,
    pub title: String,
}

#[get("/")]
pub fn index(conn: Database) -> Template {
    let mut context: HashMap<String,String> = HashMap::new();
    //let posts: Vec<crate::models::Post> = sql_query("SELECT * FROM post ORDER BY id").load(&conn).unwrap();
    //let users = username::table.load::<Username>(&conn.0).expect("Error loading users");
    /*let posts = post::table
        .load::<crate::models::Post>(&conn.0)
        .expect("Error loading posts");
    for post in posts {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.content);
    }*/
    /*let p = post::table
        .inner_join(username::table)
        .select(PostAuthor)
        .load::<PostAuthor>(&conn.0)
        .expect("ERROR");*/

    /*
    for post in p {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.email);
    }*/
    //let debug = diesel::debug_query::<diesel::pg::Pg, _>(&p);
    //println!("The insert query: {:?}", debug);


    Template::render("index",&context)
}

#[get("/page/<page>")]
pub fn index_page(page: u32) -> Option<Template> {
    let mut context: HashMap<String,String> = HashMap::new();
    Some(Template::render("index",&context))
}