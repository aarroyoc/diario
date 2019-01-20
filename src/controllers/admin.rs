use crate::Database;
use std::collections::HashMap;
use crate::models::{Username,Post,Comment};
use crate::schema::*;
use rocket_contrib::templates::Template;
use diesel::prelude::*;
use rocket::request::*;
use rocket::outcome::IntoOutcome;
use ring::{digest, test};
use rocket::http::{Cookie,Cookies};
use rocket::response::Redirect;
use data_encoding::HEXLOWER;
use chrono::prelude::*;


impl<'a, 'r> FromRequest<'a, 'r> for Username {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Username, ()> {
        let db = request.guard::<Database>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: String| username::table.filter(username::email.eq(id)).first::<Username>(&db.0).ok())
            .or_forward(())
    }
}

#[derive(Queryable,Serialize)]
struct ListingPost{
    pub id: i32,
    pub title: String,
}

#[get("/login")]
pub fn login_get() -> Template {
    let m: HashMap<String,String> = HashMap::new();
    Template::render("admin_login",m)
}

#[derive(FromForm)]
pub struct LoginForm{
    pub email: String,
    pub password: String,
}

#[post("/login",data="<login>")]
pub fn login_post(mut cookies: Cookies, login: Form<LoginForm>, conn: Database) -> Redirect {
    let password = digest::digest(&digest::SHA256, &login.password.as_ref());
    let password = HEXLOWER.encode(password.as_ref());
    let res = username::table
                .filter(username::email.eq(&login.email).and(username::password.eq(&password)))
                .first::<Username>(&conn.0);

    if let Ok(res) = res {
        cookies.add_private(Cookie::new("user_id", login.email.clone()));
        return Redirect::to("/admin/posts");
    }
    Redirect::to("/login")
}

#[get("/admin/posts")]
pub fn list_posts(user: Username, conn: Database) -> Template {
    let mut data = HashMap::new();
    let posts = post::table
                .select((
                    post::id,
                    post::title
                ))
                .order(post::date.desc())
                .load::<ListingPost>(&conn.0)
                .unwrap();
    data.insert("posts",posts);
    Template::render("admin_posts",&data)
}

#[get("/admin/post/new")]
pub fn post_view_new(_user: Username, conn: Database) -> Template {
    let m: HashMap<String,String> = HashMap::new();
    Template::render("admin_new",&m)
}


#[get("/admin/post/<id>")]
pub fn post_view(_user: Username, id: i32, conn: Database) -> Option<Template> {
    let post = post::table
                .filter(post::id.eq(id))
                .first::<Post>(&conn.0);
    if let Ok(post) = post {
        return Some(Template::render("admin_edit",&post));
    }
    None
}

#[derive(FromForm)]
pub struct EditPostForm {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
}

#[post("/admin/post", rank=1, data="<p>")]
pub fn post_edit(_user: Username, p: Form<EditPostForm>, conn: Database) -> Redirect {
    let id = p.id.parse::<i32>().unwrap();
    diesel::update(post::table)
        .filter(post::id.eq(id).and(post::status.eq("draft")))
        .set((
            post::date.eq(Utc::now().naive_local())
        ))
        .execute(&conn.0);

    diesel::update(post::table)
        .filter(post::id.eq(id))
        .set((
            post::title.eq(&p.title),
            post::content.eq(&p.content),
            post::slug.eq(&p.slug),
            post::status.eq(&p.status),
            post::excerpt.eq(p.content.lines().next().unwrap_or(&p.content))
        ))
        .execute(&conn.0);
    Redirect::to("/")
}

#[derive(FromForm)]
pub struct NewPostForm {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct NewPost {
    pub author: i32,
    pub date: NaiveDateTime,
    pub content: String,
    pub title: String,
    pub excerpt: String,
    pub status: String, /* draft, published */
    pub comment_status: String,
    pub slug: String,
}

#[post("/admin/post/new", data="<p>")]
pub fn post_new(_user: Username, p: Form<NewPostForm>, conn: Database) -> Redirect {
    diesel::insert_into(post::table)
        .values(NewPost{
            author: 1,
            date: Utc::now().naive_local(),
            content: p.content.clone(),
            title: p.title.clone(),
            excerpt: p.content.lines().next().unwrap_or(&p.content).to_string(),
            status: p.status.clone(),
            comment_status: "open".to_string(),
            slug: p.slug.clone()
        })
        .execute(&conn.0);
    Redirect::to("/")
}

#[derive(Queryable,Serialize)]
pub struct CommentModeration{
    pub id: i32,
    pub author_name: String,
    pub content: String,
    pub title: String,
}

#[get("/admin/comments")]
pub fn list_comments(_user: Username, conn: Database) -> Option<Template> {
    let comments = comment::table
                    .inner_join(post::table)
                    .select((
                        comment::id,
                        comment::author_name,
                        comment::content,
                        post::title
                    ))
                    .filter(comment::status.eq("pending"))
                    .order(comment::date.asc())
                    .limit(20)
                    .load::<CommentModeration>(&conn.0);
    
    if let Ok(comments) = comments {
        let mut m = HashMap::new();
        m.insert("comments",comments);
        return Some(Template::render("admin_comments", &m));
    }
    None
}

#[get("/admin/delete/<id>")]
pub fn comment_delete(_user: Username, id: i32, conn: Database) -> Redirect {
    diesel::delete(comment::table)
        .filter(comment::id.eq(id))
        .execute(&conn.0);
    Redirect::to("/admin/comments")
}

#[get("/admin/approve/<id>")]
pub fn comment_approve(_user: Username, id: i32, conn: Database) -> Redirect{
    diesel::update(comment::table)
        .filter(comment::id.eq(id))
        .set((
            comment::status.eq("approved")
        ))
        .execute(&conn.0);
    Redirect::to("/admin/comments")
}