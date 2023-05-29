use crate::models::{Post, NewPost};
use crate::repository::{PostRepository};

use diesel::SqliteConnection;
use diesel::result::Error;
use crate::diesel::Connection;
use dotenv::dotenv;
use std::env;


pub struct PostService {
    pub repository: PostRepository,
}

impl PostService{

    pub fn new() -> Self {
        PostService { 
            repository: PostRepository::new() 
        }
    }

    pub fn create_post(&mut self, title: &str, body: &str) -> Result<Post, Error> {
        let new_post = NewPost {
            title : String::from(title),
            body : String::from(body),
        };
        self.repository.create(&new_post)
    }

    pub fn get_posts(&mut self) ->  Result<Vec<Post>, diesel::result::Error> {
        self.repository.find_all()
    }

}