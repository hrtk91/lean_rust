use std::{net::SocketAddr};

use serve::cqrs::commands::Command;
use crate::{models::Post, repositories::Repository};

pub struct PostCommand {
    pub post: Post,
}

impl PostCommand {
    pub fn new(address: SocketAddr, content: String) -> Self {
        PostCommand {
            post: Post {
                address,
                content
            }
        }
    }
}

impl Command<Repository<Post>, Vec<Post>> for PostCommand {
    fn exec(self, repo: &mut Repository<Post>) -> Vec<Post> {
        if repo.len() > 50 {
            repo.pop_front();
        }

        repo.push_back(self.post);
        repo.clone().into()
    }
}
