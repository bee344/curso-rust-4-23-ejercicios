use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;


use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::*;

use std::env;

pub struct PostRepository {
    pub conn: SqliteConnection,

}

impl PostRepository {

    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");        

        PostRepository { 
            conn: SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
        }
    }

    pub fn find_all(&mut self) -> Result<Vec<Post>, Error>  {
        posts.load::<Post>(&mut self.conn)
    }


    pub fn find_by_id(&mut self, uniq_id: i32) -> Result<Post, Error> {
        posts.find(uniq_id).get_result::<Post>(&mut self.conn)
    }

    pub fn create(&mut self, new_post: &NewPost) -> Result<Post, Error> {
        diesel::insert_into(posts)
        .values(new_post)
        .execute(&mut self.conn)
        .expect("Error saving new post");

        posts.order(id.desc()).first(&mut self.conn)
    }

    pub fn update(&mut self, uniq_id: i32, post: Post) -> Result<Post, Error> {
        diesel::update(posts.find(post.id))
        .set(title.eq(&post.title))
        .execute(&mut self.conn)
        .unwrap();

        posts.find(post.id).first(&mut self.conn)
    }

    pub fn delete(&mut self, uniq_id: i32) -> Result<usize, Error> {
         diesel::delete(posts.find(uniq_id)).execute(&mut self.conn)
    }
    
}