use crate::AppState;
use actix_web::web::Data;
use serde::Deserialize;
// use sqlx::sqlite::SqlitePool;

// https://github.com/launchbadge/sqlx : excellent tutorial on sqlx

#[derive(Deserialize)]
pub struct CharData {
    pub carac: String,
}

#[derive(sqlx::FromRow)]
struct DBidzi {
    id: i64,
    pinyin_ton: String,
    unicode: String,
    sens: String,
}

pub async fn getsize(data: Data<AppState>) -> i64 {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pyhz")
        .fetch_one(&data.db)
        .await
        .unwrap();

    result.0
}

pub async fn readdic(data: Data<AppState>, whereclause: &str) -> String {
    let basequery = "SELECT id, pinyin_ton, unicode, sens FROM pyhz";
    let qq: String;
    let query = if !whereclause.is_empty() {
        qq = format!("{} WHERE {}", basequery, whereclause);
        &qq
    } else {
        basequery
    };

    let mut disp = String::from("");
    let dic = sqlx::query_as::<_, DBidzi>(&query)
        .fetch_all(&data.db)
        .await
        .unwrap();
    if dic.len() == 0 {
        return String::from("");
    };

    for dbidzi in dic.iter() {
        disp = format!(
            "{}<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            disp, dbidzi.id, dbidzi.pinyin_ton, dbidzi.unicode, dbidzi.sens
        );
    }
    disp = format!("<table>{}</table>", disp);
    disp
}

pub async fn list_for_zi(data: Data<AppState>, first: String) -> String {
    let whereclause = format!(" unicode = '{}'", &first);
    readdic(data, &whereclause).await
}
