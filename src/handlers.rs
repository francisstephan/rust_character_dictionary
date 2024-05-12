use crate::AppState;
use actix_web::{delete, get, post, put, web, web::Data, HttpResponse, Responder};
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

#[get("/showlast")]
pub async fn showlast(tera: Data<Tera>, data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("query", "last entry");
    let disp = dbase::list_last(data).await;

    ctx.insert("dico", &disp);
    ctx.insert("diclen", &disp.len());
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[get("/getziform")]
pub async fn getziform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::ziform();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getpyform")]
pub async fn getpyform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::pyform();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getaddziform")]
pub async fn getaddziform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::addziform();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getselupdate")]
pub async fn getselupdate(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::selupdate();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getseldelete")]
pub async fn getseldelete(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert = forms::seldelete();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[post("/updatezi")]
pub async fn updatezi(
    iddata: web::Form<dbase::IdData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = iddata.id;
    let zi = dbase::get_for_id(data, id).await;
    let mut ctx = Context::new();
    ctx.insert("id", &id.to_string());
    ctx.insert("len", &zi.len());
    if zi.len() != 0 {
        let hanzi = &zi[0];
        ctx.insert("zi", &hanzi);
    }
    HttpResponse::Ok().body(tera.render("components/updateform.html", &ctx).unwrap())
}

#[post("/deletezi")]
pub async fn deletezi(
    iddata: web::Form<dbase::IdData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = iddata.id;
    let zi = dbase::get_for_id(data, id).await;
    let mut ctx = Context::new();
    ctx.insert("id", &id.to_string());
    ctx.insert("len", &zi.len());
    if zi.len() != 0 {
        let hanzi = &zi[0];
        ctx.insert("zi", &hanzi);
    }
    HttpResponse::Ok().body(tera.render("components/deleteform.html", &ctx).unwrap())
}

#[post("/addzi")]
pub async fn addzi(
    zidata: web::Form<dbase::Idzi>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chaine = dbase::addzi_db(zidata, data).await;
    let mut ctx = Context::new();
    ctx.insert("content", &chaine);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[put("/doupdate")]
pub async fn doupdate(
    zidata: web::Form<dbase::Idzi>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chaine = dbase::update_db(zidata, data).await;
    let mut ctx = Context::new();
    ctx.insert("content", &chaine);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[delete("/dodelete/{id}")]
pub async fn do_delete(
    path: web::Path<String>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner().parse::<i64>().unwrap();
    let chaine = dbase::delete_db(id, data).await;
    let mut ctx = Context::new();
    ctx.insert("content", &chaine);
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
    let mut ctx = Context::new();
    ctx.insert("query", &chain);
    let disp = dbase::list_for_zi(data, format!("{:X}", first as u32)).await;

    ctx.insert("dico", &disp);
    ctx.insert("diclen", &disp.len());
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[post("/pylist")]
pub async fn pylist(
    formdata: web::Form<dbase::PinyinData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chain = &formdata.pinyin;
    let mut ctx = Context::new();
    ctx.insert("query", &chain);

    let disp = dbase::list_for_py(data, String::from(chain)).await;
    ctx.insert("dico", &disp);
    ctx.insert("diclen", &disp.len());
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[get("/listdic")]
pub async fn listdic(tera: Data<Tera>, data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("query", "List dictionary");

    let disp = dbase::readdic(&data, "").await;
    ctx.insert("dico", &disp);
    ctx.insert("diclen", &disp.len());
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[get("/hello/{name}")]
pub async fn say_hello(path: web::Path<String>) -> impl Responder {
    let resp = format!("hello, {}!!", path.into_inner());
    HttpResponse::Ok().body(resp)
}
