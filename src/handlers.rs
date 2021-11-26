use actix_web::dev::HttpResponseBuilder;
use actix_web::{delete, get, post, put, web, HttpResponse};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::entities::TodoEntity;
use crate::errors::AppError;
use crate::request::{AddTodoRequest, DeleteTodoRequest, UpdateTodoRequest};
use crate::response::TodoResponse;

#[get("/")]
pub async fn index_todo(
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, AppError> {
    // コネクションプールからDBインスタンス取得
    let conn = db.get()?;

    // SQL文を実行用の型に変換
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;

    // SQLを実行し、結果をEntityに変換
    let mut entities = Vec::new();
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntity { id, text })
    })?;
    for row in rows {
        entities.push(row?);
    }

    // EntityをResponse用の構造体に格納
    let mut resp = Vec::new();
    for entity in entities {
        resp.push(TodoResponse {
            id: entity.id,
            text: entity.text,
        })
    }

    // ステータスコード200とデータを返す
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/add")]
pub async fn add_todo(
    req: web::Json<AddTodoRequest>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponseBuilder, AppError> {
    let conn = db.get()?;

    let mut statement = conn.prepare("INSERT INTO todo (text) VALUES (?)")?;
    // TODO: 本来はSQL実行前にEntityに置き換えたほうが良い
    statement.execute(&[&req.text])?;

    // ステータスコード201を返す
    Ok(HttpResponse::Created())
}

#[put("/update")]
pub async fn edit_todo(
    req: web::Json<UpdateTodoRequest>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponseBuilder, AppError> {
    let conn = db.get()?;

    let mut statement = conn.prepare("UPDATE todo SET text = ?1 WHERE id = ?2")?;
    statement.execute(params![&req.text, req.id])?;

    Ok(HttpResponse::NoContent())
}

#[delete("/delete")]
pub async fn delete_todo(
    req: web::Json<DeleteTodoRequest>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponseBuilder, AppError> {
    let conn = db.get()?;

    let mut statement = conn.prepare("DELETE FROM todo WHERE id = ?")?;
    statement.execute(&[&req.id])?;

    // ステータスコード204を返す
    Ok(HttpResponse::NoContent())
}
