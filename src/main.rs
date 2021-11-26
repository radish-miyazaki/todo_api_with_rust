mod entities;
mod errors;
mod handlers;
mod request;
mod response;

use actix_web::{App, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::handlers::{add_todo, delete_todo, edit_todo, index_todo};

// actix用のRuntimeライブラリを用いてmain関数を非同期に
#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    /*
       Sqliteとのコネクション確立
    */
    // Sqliteとのコネクションを管理するManagerを生成
    let manager = SqliteConnectionManager::file("todo.db");
    // Managerから、Sqliteとのコネクションをプールするインスタンスを生成
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool");
    // プールからコネクションを取り出して、DB操作用のインスタンスを生成
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool");

    /*
       SQLを実行して、テーブルを生成
    */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (\
            id INTEGER PRIMARY KEY AUTOINCREMENT,\
            text TEXT NOT NULL\
      )",
        params![],
    )
    .expect("Failed to create a table `todo`");

    /*
       APIサーバを立ち上げる
    */
    // コネクションプールを渡す
    HttpServer::new(move || {
        App::new()
            .service(index_todo)
            .service(add_todo)
            .service(edit_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
