use mysql::*;
use mysql::prelude::*;

use lazy_static::lazy_static;

pub struct PostEntity(pub u64, pub String, pub String, pub String);

lazy_static! {
    static ref DB_CONNECTION_POOL: Pool = {
        let url = "mysql://root:1111@localhost:3306/blogdb";
        let pool = Pool::new(url).unwrap();
        pool
    };
}

pub fn fetch_posts() -> Result<Vec<PostEntity>> {
    let mut conn = DB_CONNECTION_POOL.get_conn().unwrap();

    conn.query_map("SELECT ID, TITLE, POST_TEXT, POST_TIME FROM POSTS",
        |(id, title, post_text, post_time)| {
            println!("TIME: {}", &post_time);
            PostEntity(id, title, post_text, post_time)
        }
    )
}

pub fn insert_post(title: &str, post_text: &str) -> Result<()> {
    let mut conn = DB_CONNECTION_POOL.get_conn().unwrap();

    conn.exec_drop(r"INSERT INTO POSTS (TITLE, POST_TEXT) VALUES (:title, :post_text)",
                           params! {"title" => title, "post_text" => post_text}
    )
}