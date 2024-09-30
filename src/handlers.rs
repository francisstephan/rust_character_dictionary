use crate::AppState;
use actix_web::{delete, get, post, put, web, web::Data, HttpRequest, HttpResponse, Responder};
use std::fs;
use tera::{Context, Tera};

use crate::dbase;
use crate::forms;

#[get("/")]
pub async fn index(req: HttpRequest, tera: Data<Tera>) -> impl Responder {
    let ipaddr = actix_remote_ip::get_remote_ip(&req);
    let mut ctx = Context::new();
    if ipaddr.is_loopback() {
        ctx.insert("contenu", "Connected to local server");
    } else {
        ctx.insert("contenu", "Caution: all modifications to the database will be discarded at the end of the fly.io session");
    }
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
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[get("/getziform")]
pub async fn getziform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert: String = forms::ziform();
    ctx.insert("content", &insert);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/getparseform")]
pub async fn getparseform(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    let insert: String = forms::zistringform();
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
    zidata: web::Form<dbase::Idzi>, // deserialization by serde of form info
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chaine = dbase::update_db(zidata, data).await;
    let mut ctx = Context::new();
    ctx.insert("content", &chaine);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[delete("/dodelete/{id}")]
pub async fn dodelete(
    iddata: web::Path<dbase::IdData>, // deserialization by serde of path info
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = iddata.id;
    let chaine = dbase::delete_db(id, data).await;
    let mut ctx = Context::new();
    ctx.insert("content", &chaine);
    HttpResponse::Ok().body(tera.render("components/content.html", &ctx).unwrap())
}

#[get("/cancel")]
pub async fn cancel(tera: Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("content", "Form canceled");
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
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[post("/stringparse")]
pub async fn stringparse(
    formdata: web::Form<dbase::ZiStrData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chain = ammonia::clean(&formdata.zistr); // check String sanity
    let mut chars = chain.chars();
    let mut parsed = String::new();
    let mut unknown = Vec::<String>::new();
    while let Some(carac) = chars.next() {
        // 1. If carac is not a chinese character, simply append it to parsed
        if (carac as i64) < 0x2000 || "。，“”（）、《》—；！".find(carac) != None {
            parsed = format!("{}{}", parsed, carac)
        } else {
            // 2. get all pinyin for the carac character in the database
            let disp = dbase::zi_to_py(&data, carac).await;
            if disp.len() > 0 {
                // 3. The character exists in the database: give all pinyin separated by /
                parsed = format!("{}{}", parsed, " "); // insert space for better readibility
                for (i, py) in disp.iter().enumerate() {
                    if i > 0 {
                        parsed = format!("{}{}", parsed, "/");
                    }
                    parsed = format!("{}{}", parsed, py);
                }
            } else {
                // 4. The character is not in the base: add it to the unknown Vec
                // 5. and append it as such (unparsed) to parsed
                unknown.push(carac.to_string());
                parsed = format!("{}{}", parsed, carac);
            }
        }
    }

    let mut ctx = Context::new();
    ctx.insert("query", &chain);
    ctx.insert("parsedstr", &parsed);
    ctx.insert("unknownzi", &unknown);

    HttpResponse::Ok().body(tera.render("components/parsed.html", &ctx).unwrap())
}

#[post("/pylist")]
pub async fn pylist(
    formdata: web::Form<dbase::PinyinData>,
    tera: Data<Tera>,
    data: web::Data<AppState>,
) -> impl Responder {
    let chain = &formdata.pinyin_ton;
    let mut ctx = Context::new();
    ctx.insert("query", &chain);

    let disp = dbase::list_for_py(data, String::from(chain)).await;
    ctx.insert("dico", &disp);
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}

#[get("/listdic")]
pub async fn listdic(tera: Data<Tera>, data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("query", "List dictionary");

    let disp = dbase::readdic(&data, "").await;
    ctx.insert("dico", &disp);
    HttpResponse::Ok().body(tera.render("components/zilist.html", &ctx).unwrap())
}
