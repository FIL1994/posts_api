#[macro_use]
extern crate tower_web;
extern crate tokio;

use sqlite;
use tokio::prelude::*;
use tower_web::ServiceBuilder;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct PostsApp;

/// This will be the JSON response
#[derive(Response)]
struct PostsResponse {
    posts: Vec<Post>,
}

#[derive(Debug, Serialize)]
struct Post {
    id: i64,
    title: String,
    body: String,
}

impl_web! {
    impl PostsApp {
        #[get("/")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<PostsResponse, ()> {
            let conn = sqlite::open("../my_db.db").unwrap();

            let mut posts:Vec<Post> = vec![];

            let mut cursor = conn.prepare("SELECT id, title, body FROM posts").unwrap().cursor();

            while let Some(row) = cursor.next().unwrap() {
                let id = row[0].as_integer().unwrap();
                let title = row[1].as_string().unwrap().to_string();
                let body = match row[2].as_string() {
                    Some(body) => body,
                    None => ""
                }.to_string();

                let post = Post {
                    id, title, body
                };

                posts.push(post)
            }

            Ok(PostsResponse {
                posts
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:3100".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new().resource(PostsApp).run(&addr).unwrap();
}
