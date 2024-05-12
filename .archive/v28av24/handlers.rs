use crate::AppState;
use actix_web::{get, post, web, web::Data, HttpResponse, Responder};
use std::fs;
use tera::{Context, Tera};

use crate::dbase;
use crate::forms;

#[get("/")]
pub async fn index(tera: Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    HttpResponse::Ok().body(tera.render("index.html", &ctx).unwrap())
}

#[get("/size")]
pub async fn size(tera: Data<Tera>, data: web::Data<AppState>) -> impl Responder {
    // https://stackoverflow.com/questions/669092/sqlite-getting-number-of-rows-in-a-database
    let mut ctx = Context::new();

    let size = dbase::getsize(data).await;
    let metadata = fs::metadata("vol/zidian.db").expect("Failed to read file metadata");
    let time = metadata.modified().unwrap();
    use chrono::prelude::{DateTime, Utc};
    let dt: DateTime<Utc> = time.clone().into();
    ctx.insert(
        "content",
        format!(
            "The dictionary presently contains {} entries. Last updated on {}",
            &size,
            &dt.format("%Y-%m-%d")
        )
        .as_str(),
    );
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getziform")]
pub async fn getziform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::ziform();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/remove")]
pub async fn remove(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = "Form canceled";
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[post("/zilist")]
pub async fn zilist(
    formdata: web::Form<dbase::CharData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chain = &formdata.carac;
    let first: char = chain.chars().next().unwrap();
    let disp = dbase::list_for_zi(data, format!("{:X}", first as u32));
    let mut ctx = Context::new();
    ctx.insert("content", &disp.await);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hello/{name}")]
pub async fn say_hello(path: web::Path<String>) -> impl Responder {
    let resp = format!("hello, {}!!", path.into_inner());
    HttpResponse::Ok().body(resp)
}
