use actix_files;
use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::{Context, Tera};

// https://dev.to/bitecode/day11-write-web-app-with-actix-web-100dayofrust-1lkn : attention, dépassé
// https://stackoverflow.com/questions/76999829/actix-web-tera-tamplates-give-me-actix-webdata-failed-to-extract-datater

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("vol/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut ctx = Context::new();
    let name = "Rust zidian".to_string();
    ctx.insert("content", &name);
    match TEMPLATES.render("index.html", &ctx) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(err) => {
            eprintln!("## Tera error: {}", err);
            Err(error::ErrorInternalServerError(err))
        }
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hello/{name}")]
async fn say_hello(path: web::Path<String>) -> impl Responder {
    let resp = format!("hello, {}!!", path.into_inner());
    HttpResponse::Ok().body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .service(say_hello)
            .service(actix_files::Files::new("/assets", "./vol/assets").show_files_listing())
        // making /assets files accessible
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
