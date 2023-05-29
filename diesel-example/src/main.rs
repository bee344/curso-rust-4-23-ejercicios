extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;
use service::PostService;
use std::env;
use std::ops::Add;

mod schema;
mod models;
mod repository;
mod service;

use crate::repository::PostRepository;

use self::models::{NewPost};
use self::schema::posts::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &mut SqliteConnection, new_title: &'a str, new_body: &'a str) -> Post{
    let new_post = NewPost {
        title : String::from(new_title),
        body : String::from(new_body),
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts.order(id.desc()).first(conn).unwrap()
}

fn update_post(conn: &mut SqliteConnection, post: &Post, new_title: &str) -> Post {
    diesel::update(posts.find(post.id))
        .set(title.eq(new_title))
        .execute(conn)
        .unwrap();

    posts.find(post.id).first(conn).unwrap()
}

fn main() {

    //////////////////////////////////////////////////
    ////////  Trabajar unicamente con Diesel   ///////
    //////////////////////////////////////////////////
    // let mut conn = establish_connection();

    // let new_title = "Ejemplo de Post";
    // let new_body = "Este es un ejemplo funcional para almacenar un POST";

    // let new_post = create_post(&mut conn, new_title, new_body);
    // println!("{:?}", new_post);

    // let modified_title = String::from(new_title);
    // let modified_post = update_post(&mut conn, &new_post, &modified_title.add(" (MODIFICADO)"));
    // println!("{:?}", modified_post);

    // let data = posts.load::<Post>(&mut conn).unwrap();
    // for post in data {
    //     println!("{:?}", post);
    // }

    ///////////////////////////////////////////////////////
    ////////  Trabajar Patrones Diseño con Diesel   ///////
    ///////////////////////////////////////////////////////
    let new_title = "Ejemplo de Post";
    let new_body = "Este es un ejemplo funcional para almacenar un POST";

    let mut post_service = PostService::new();

    let new_post = post_service.create_post(new_title, new_body);

    let data_from_repository = post_service.get_posts();
    for post in data_from_repository {
        println!("{:?}", post);
    }


}