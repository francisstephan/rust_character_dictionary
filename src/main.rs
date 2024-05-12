use actix_files;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use tera::Tera;

mod dbase;
mod forms;
mod handlers;

// https://dev.to/bitecode/day11-write-web-app-with-actix-web-100dayofrust-1lkn : caution, no longer fully valid
// https://stackoverflow.com/questions/76999829/actix-web-tera-tamplates-give-me-actix-webdata-failed-to-extract-datater
// https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/
// https://kojinglick.com/using-htmx-with-rust-quickstart  (htmx + actix, very inspiring)
// https://stackoverflow.com/questions/29026066/what-is-the-idiomatic-way-to-convert-a-string-to-str

pub struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    log::debug!("Starting Server");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    let tera = Data::new(Tera::new("./vol/templates/**/*.html").unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(Logger::new("%U %r %s"))
            .service(handlers::index)
            .service(handlers::say_hello)
            .service(handlers::size)
            .service(handlers::getziform)
            .service(handlers::zilist)
            .service(handlers::getpyform)
            .service(handlers::pylist)
            .service(handlers::listdic)
            .service(handlers::getaddziform)
            .service(handlers::addzi)
            .service(handlers::getselupdate)
            .service(handlers::updatezi)
            .service(handlers::doupdate)
            .service(handlers::getseldelete)
            .service(handlers::deletezi)
            .service(handlers::do_delete)
            .service(handlers::showlast)
            .service(handlers::remove)
            .service(actix_files::Files::new("/assets", "./vol/assets").show_files_listing())
        // making /assets files accessible
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
