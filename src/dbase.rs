use crate::AppState;
use actix_web::web::{Data, Form};
use serde::{Deserialize, Serialize};

// https://github.com/launchbadge/sqlx : excellent tutorial on sqlx

#[derive(Deserialize)]
pub struct CharData {
    pub carac: String,
}

#[derive(Deserialize)]
pub struct ZiStrData {
    pub zistr: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct PinyinData {
    pub pinyin_ton: String,
}

#[derive(Deserialize)]
pub struct IdData {
    pub id: i64,
}

#[derive(Serialize)]
pub struct Zi {
    pub id: i64,
    pub pinyin_ton: String,
    pub unicode: String,
    pub hanzi: char,
    pub sens: String,
}

#[derive(sqlx::FromRow)]
pub struct DBidzi {
    id: i64,
    pinyin_ton: String,
    unicode: String,
    sens: String,
}

#[derive(Deserialize)]
pub struct Idzi {
    pub id: i64,
    pub pinyin_ton: String,
    pub unicode: String,
    pub sens: String,
}

pub async fn getsize(data: Data<AppState>) -> i64 {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pyhz")
        .fetch_one(&data.db)
        .await
        .unwrap();

    result.0
}

async fn read_query(query: &str, data: &Data<AppState>) -> Vec<Zi> {
    let mut disp = Vec::<Zi>::new();
    let dic = sqlx::query_as::<_, DBidzi>(&query)
        .fetch_all(&data.db)
        .await
        .unwrap();

    for dbidzi in dic.iter() {
        //https://stackoverflow.com/questions/69152223/unicode-codepoint-to-rust-string
        let unicodestring = dbidzi.unicode.as_str();
        let unicode = u32::from_str_radix(unicodestring, 16).unwrap();
        let carac = char::from_u32(unicode).unwrap();
        let zi = Zi {
            id: dbidzi.id,
            pinyin_ton: dbidzi.pinyin_ton.clone(),
            unicode: dbidzi.unicode.clone(),
            hanzi: carac,
            sens: dbidzi.sens.clone(),
        };
        disp.push(zi);
    }

    disp
}

pub async fn list_last(data: Data<AppState>) -> Vec<Zi> {
    // https://stackoverflow.com/questions/24494182/how-to-read-the-last-record-in-sqlite-table
    let query = "SELECT * FROM pyhz ORDER BY id DESC LIMIT 1";
    read_query(query, &data).await
}

pub async fn update_db(zi: Form<Idzi>, data: Data<AppState>) -> String {
    //   https://github.com/addies/rust-crud-axum-sqlx-postgresql/blob/main/src/main.rs
    let query = format!(
        "UPDATE pyhz SET  sens='{}' WHERE id='{}'",
        ammonia::clean(&zi.sens),
        zi.id
    );
    let result = sqlx::query(&query).execute(&data.db).await;
    match result {
        Ok(_) => return format!("Successful update, entry Id = {}", zi.id),
        Err(_) => return String::from("Internal server error"),
    }
}

pub async fn delete_db(id: i64, data: Data<AppState>) -> String {
    let query = format!("DELETE FROM pyhz WHERE id= {} ", id);

    let result = sqlx::query(&query).execute(&data.db).await;
    match result {
        Ok(_) => return format!("Successful deleted entry Id = {}", id),
        Err(_) => return String::from("Internal server error"),
    }
}

pub async fn addzi_db(zi: Form<Idzi>, data: Data<AppState>) -> String {
    //   https://github.com/addies/rust-crud-axum-sqlx-postgresql/blob/main/src/main.rs
    // 1. check zi not already in dictionary:
    let whereclause = format!(
        " pinyin_ton ='{}' AND unicode = '{}'",
        zi.pinyin_ton, zi.unicode
    );
    let disp = readdic(&data, &whereclause).await;
    if disp.len() > 0 {
        return String::from("Character already in dictionary. Use update to modify");
    }

    // 2. add zi:
    let query = format!(
        "INSERT INTO pyhz (pinyin_ton, unicode, sens) VALUES ('{}', '{}', '{}')",
        zi.pinyin_ton,
        zi.unicode,
        ammonia::clean(&zi.sens) // sanitize zi.sens, which has no specific format
    );
    let result = sqlx::query(&query).execute(&data.db).await;
    match result {
        Ok(res) => return format!("Inserted new zi with id {}", res.last_insert_rowid()),
        Err(err) => return err.to_string(),
    }
}

pub async fn readdic(data: &Data<AppState>, whereclause: &str) -> Vec<Zi> {
    let basequery = "SELECT id, pinyin_ton, unicode, sens FROM pyhz";
    let qq: String;
    let query = if !whereclause.is_empty() {
        qq = format!("{} WHERE {}", basequery, whereclause);
        &qq
    } else {
        basequery
    };
    read_query(query, &data).await
}

pub async fn list_for_zi(data: Data<AppState>, first: String) -> Vec<Zi> {
    let whereclause = format!(" unicode = '{}' ORDER BY pinyin_ton", &first);
    readdic(&data, &whereclause).await
}

pub async fn zi_to_py(data: &Data<AppState>, carac: char) -> Vec<String> {
    // get unicode from carac:
    let mut unicode = format!("{:x}", carac as u32);
    unicode = unicode.to_uppercase();
    let query = format!(
        "SELECT pinyin_ton FROM pyhz WHERE unicode = '{}' ORDER BY pinyin_ton",
        unicode
    );
    let mut disp = Vec::<String>::new();
    let dic = sqlx::query_as::<_, PinyinData>(&query)
        .fetch_all(&data.db)
        .await
        .unwrap();
    for pinyindata in dic.iter() {
        disp.push(pinyindata.pinyin_ton.clone());
    }
    disp
}

pub async fn list_for_py(data: Data<AppState>, first: String) -> Vec<Zi> {
    let last_char = &first.chars().last().unwrap();
    let cond = matches!(last_char, '0'..='4');
    let whereclause = if !cond {
        // no tone given: check all tones 0 to 4
        format!(" pinyin_ton = '{}0' OR pinyin_ton = '{}1' OR pinyin_ton = '{}2' OR pinyin_ton = '{}3' OR pinyin_ton = '{}4' ORDER BY pinyin_ton, unicode"
            , &first, &first, &first, &first, &first)
    } else {
        format!(" pinyin_ton = '{}' ORDER BY unicode", &first)
    };
    readdic(&data, &whereclause).await
}

pub async fn get_for_id(data: Data<AppState>, id: i64) -> Vec<Zi> {
    let whereclause = format!(" id = '{}'", id);
    readdic(&data, &whereclause).await
}
