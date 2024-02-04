use crate::models::*;
use crate::schema::*;
use crate::Database;

use crate::controllers::comment::CommentInsert;

use std::collections::HashMap;

use rocket::http::{Cookie, CookieJar};
use rocket::request::*;
use rocket::response::*;
use rocket_dyn_templates::Template;
use rocket::form::Form;
use rocket::outcome::IntoOutcome;
use rocket::http::Status;
use rocket::outcome::try_outcome;

use chrono::prelude::*;
use data_encoding::HEXLOWER;
use diesel::prelude::*;
use diesel::dsl::insert_into;
use ring::digest;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Username {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let db = try_outcome!(request.guard::<Database>().await);
        let cookie: Option<String> = request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok());
	match cookie {
	    Some(id) => {
                db.run(|c| {
                    username::table
                        .filter(username::email.eq(id))
                        .first::<Username>(c)
			.ok()
		}).await
	    },
	    _ => None
	}.or_forward(Status::Unauthorized)
    }
}

#[derive(Queryable, Serialize)]
struct ListingPost {
    pub id: i32,
    pub title: String,
}

#[get("/login")]
pub async fn login_get() -> Template {
    let m: HashMap<String, String> = HashMap::new();
    Template::render("admin_login", m)
}

#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
pub async fn login_post(cookies: &CookieJar<'_>, login: Form<LoginForm>, conn: Database) -> Redirect {
    let password = digest::digest(&digest::SHA256, &login.password.as_ref());
    let password = HEXLOWER.encode(password.as_ref());
    let email = login.email.clone();
    let res = conn.run(move |c| {
	username::table
	    .filter(
		username::email
		    .eq(&email)
		    .and(username::password.eq(&password)),
	    )
            .first::<Username>(c)
    }).await;

    if let Ok(_res) = res {
        cookies.add_private(Cookie::new("user_id", login.email.clone()));
        return Redirect::to("/admin/posts");
    }
    Redirect::to("/login")
}

#[get("/admin/posts")]
pub async fn list_posts(_user: Username, conn: Database) -> Template {
    let mut data = HashMap::new();
    let posts = conn.run(|c|{
	post::table
	    .select((post::id, post::title))
	    .order(post::date.desc())
	    .load::<ListingPost>(c)
            .unwrap()
    }).await;
    data.insert("posts", posts);
    Template::render("admin_posts", &data)
}

#[get("/admin/post/new")]
pub async fn post_view_new(_user: Username) -> Template {
    let m: HashMap<String, String> = HashMap::new();
    Template::render("admin_new", &m)
}

#[derive(Serialize)]
struct PostEditTera {
    pub post: Post,
    pub tags: String,
}

#[get("/admin/post/<id>")]
pub async fn post_view(_user: Username, id: i32, conn: Database) -> Option<Template> {
    let post = conn.run(move |c| post::table.filter(post::id.eq(id)).first::<Post>(c)).await;
    let tags = conn.run(move |c| {
	tag::table
	    .select(tag::name)
	    .filter(tag::post_id.eq(id))
	    .load::<String>(c)
            .unwrap()
    }).await;

    if let Ok(post) = post {
        return Some(Template::render(
            "admin_edit",
            PostEditTera {
                post,
                tags: tags.join(","),
            },
        ));
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
    pub tags: String,
}

#[derive(Insertable)]
#[diesel(table_name = tag)]
pub struct TagInsert {
    pub name: String,
    pub post_id: i32,
}

#[post("/admin/post", rank = 1, data = "<p>")]
pub async fn post_edit(_user: Username, p: Form<EditPostForm>, conn: Database) -> Redirect {
    let id = p.id.parse::<i32>().unwrap();
    let tags = p.tags.clone();
    let res = conn.run(move |c| {
	diesel::update(post::table)
	    .filter(post::id.eq(id).and(post::status.eq("draft")))
	    .set(post::date.eq(Utc::now().naive_local()))
            .execute(c)
    }).await;
    if let Err(err) = res {
        eprintln!("error adding tags: {:?}", err);
    }

    let res = conn.run(move |c| {
	diesel::update(post::table)
	    .filter(post::id.eq(id))
	    .set((
		post::title.eq(&p.title),
		post::content.eq(&p.content),
		post::slug.eq(&p.slug),
		post::status.eq(&p.status),
		post::excerpt.eq(p.content.lines().next().unwrap_or(&p.content)),
	    ))
            .execute(c)
    }).await;
    if let Err(err) = res {
        eprintln!("error adding tags: {:?}", err);
    }

    let res = conn.run(move |c| {
	diesel::delete(tag::table)
            .filter(tag::post_id.eq(id))
            .execute(c)
    }).await;
    if let Err(err) = res {
        eprintln!("error adding tags: {:?}", err);
    }

    for tag in tags.split(',') {
	let t = tag.to_string();
        let res = conn.run(move |c| {
	    diesel::insert_into(tag::table)
		.values(TagInsert {
		    name: t,
		    post_id: id,
		})
		.execute(c)
	}).await;

        if let Err(err) = res {
            eprintln!("error adding tags: {:?}", err);
        }
    }
    Redirect::to("/")
}

#[derive(FromForm)]
pub struct NewPostForm {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub tags: String,
}

#[derive(Insertable)]
#[diesel(table_name = post)]
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

#[post("/admin/post/new", data = "<p>")]
pub async fn post_new(_user: Username, p: Form<NewPostForm>, conn: Database) -> Redirect {
    let slug = p.slug.clone();
    let tags = p.tags.clone();
    let res = conn.run(move |c| {
	diesel::insert_into(post::table)
	    .values(NewPost {
		author: 1,
		date: Utc::now().naive_local(),
		content: p.content.clone(),
		title: p.title.clone(),
		excerpt: p.content.lines().next().unwrap_or(&p.content).to_string(),
		status: p.status.clone(),
		comment_status: "open".to_string(),
		slug: p.slug.clone(),
	    })
            .execute(c)
    }).await;

    if let Err(err) = res {
        eprintln!("error creating post: {:?}", err);
    }

    let id = conn.run(move |c| {
	post::table
	    .select(post::id)
	    .filter(post::slug.eq(&slug))
	    .first::<i32>(c)
            .unwrap()
    }).await;

    for tag in tags.split(',') {
	let t = tag.to_string();
        let res = conn.run(move |c| {
	    diesel::insert_into(tag::table)
		.values(TagInsert {
		    name: t,
		    post_id: id,
		})
		.execute(c)
	}).await;
        if let Err(err) = res {
            eprintln!("error adding tags: {:?}", err);
        }
    }
    Redirect::to("/")
}

#[derive(Queryable, Serialize)]
pub struct CommentModeration {
    pub id: i32,
    pub author_name: String,
    pub content: String,
    pub title: String,
    pub slug: String,
}

#[get("/admin/comments")]
pub async fn list_comments(_user: Username, conn: Database) -> Option<Template> {
    let comments = conn.run(|c| {
	comment::table
	    .inner_join(post::table)
	    .select((
		comment::id,
		comment::author_name,
		comment::content,
		post::title,
		post::slug,
	    ))
	    .filter(comment::status.eq("pending"))
	    .order(comment::date.asc())
	    .limit(20)
            .load::<CommentModeration>(c)
    }).await;

    if let Ok(comments) = comments {
        let mut m = HashMap::new();
        m.insert("comments", comments);
        return Some(Template::render("admin_comments", &m));
    }
    None
}

#[get("/admin/delete/<id>")]
pub async fn comment_delete(_user: Username, id: i32, conn: Database) -> Redirect {
    let res = conn.run(move |c| {
	diesel::delete(comment::table)
            .filter(comment::id.eq(id))
            .execute(c)
    }).await;

    if let Err(err) = res {
        eprintln!("error deleting comment: {:?}", err);
    }
    Redirect::to("/admin/comments")
}

#[get("/admin/approve/<id>")]
pub async fn comment_approve(_user: Username, id: i32, conn: Database) -> Redirect {
    let res = conn.run(move |c| {
	diesel::update(comment::table)
	    .filter(comment::id.eq(id))
	    .set(comment::status.eq("approved"))
            .execute(c)
    }).await;
    if let Err(err) = res {
        eprintln!("error approving comment: {:?}", err);
    }
    Redirect::to("/admin/comments")
}

#[derive(FromForm)]
pub struct CommentResponseForm {
    pub content: String,
}

#[post("/admin/approve/<id>", data = "<comment>")]
pub async fn comment_approve_response(user: Username, id: i32, comment: Form<CommentResponseForm>, conn: Database) -> Redirect {
    let post_id = conn.run(move |c| {
	comment::table
	    .select(comment::post_id)
	    .filter(comment::id.eq(id))
	    .load::<i32>(c)
            .unwrap()[0]
    }).await;

    let response = CommentInsert{
        date: Utc::now().naive_local(),
        content: comment.content.clone(),
        status: "approved".to_string(),
        post_id: post_id,
        author_name: user.display_name.clone(),
        author_mail: Some(user.email.clone()),
        author_url: None,
        author_useragent: None,
    };

    let res = conn.run(|c| {
	insert_into(comment::table)
            .values(response)
            .execute(c)
    }).await;

    if let Err(err) = res {
        eprintln!("error: {:?}", err);
    }

    comment_approve(user, id, conn).await
}
