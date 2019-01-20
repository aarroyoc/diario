use crate::Database;
use crate::schema::*;

use rocket_contrib::templates::Template;

use diesel::prelude::*;
use chrono::prelude::*;


/*#[derive(Queryable)]
struct PostAuthor {
    pub email: String,
    pub title: String,
}*/

#[get("/")]
pub fn index(conn: Database) -> Option<Template> {
    index_date(Utc::now().naive_local().timestamp(),conn)
    
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
        .select((
            username::email,
            post::title
        ))
        .load::<PostAuthor>(&conn.0)
        .expect("ERROR");

    
    for post in p {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.email);
    }*/
    //let debug = diesel::debug_query::<diesel::pg::Pg, _>(&p);
    //println!("The insert query: {:?}", debug);
}

#[derive(Queryable,Serialize)]
struct ListingPost{
    pub title: String,
    pub name: String,
    pub excerpt: String,
    pub date: NaiveDateTime,
}

#[derive(Serialize)]
struct IndexPageData {
    pub posts: Vec<ListingPost>,
    pub last_date: i64,
    pub tag: Option<String>,
}

#[get("/?<date>")]
pub fn index_date(date: i64, conn: Database) -> Option<Template> {
    let date = NaiveDateTime::from_timestamp(date, 0);

    let posts = post::table
        .select((
            post::title,
            post::slug,
            post::excerpt,
            post::date
        ))
        .filter(post::date.lt(date).and(post::status.eq("published")))
        .order(post::date.desc())
        .limit(10)
        .load::<ListingPost>(&conn.0)
        .expect("Error loading posts");

    let last_date = posts[posts.len()-1].date.timestamp();

    let data = IndexPageData{
        posts,
        last_date,
        tag: None
    };

    Some(Template::render("index",&data))
}

#[get("/tag/<tag>")]
pub fn tag(tag: String, conn: Database) -> Option<Template> {
    tag_date(tag,Utc::now().naive_local().timestamp(),conn)
}

#[get("/tag/<tag>?<date>")]
pub fn tag_date(tag: String, date: i64, conn: Database) -> Option<Template> {
    let date = NaiveDateTime::from_timestamp(date, 0);

    let posts = tag::table
                .inner_join(post::table)
                .select((
                    post::title,
                    post::slug,
                    post::excerpt,
                    post::date
                ))
                .filter(post::date.lt(date)
                    .and(post::status.eq("published"))
                    .and(tag::name.eq(&tag)))
                .order(post::date.desc())
                .limit(10)
                .load::<ListingPost>(&conn.0);

    if let Ok(posts) = posts {
        if posts.is_empty() {
            return None;
        }
        let last_date = posts[posts.len()-1].date.timestamp();

        let data = IndexPageData{
            posts,
            last_date,
            tag: Some(tag.clone())
        };
        return Some(Template::render("index",&data));
    }
    None
}