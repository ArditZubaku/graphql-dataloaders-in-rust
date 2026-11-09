use async_graphql::ID;

use crate::user_service::User;

pub struct DB;

impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from("1"),
                name: String::from("Alice"),
                email: String::from("alice@email.com"),
                phone: String::from("123-456-7890"),
                address: String::from("123 Main Street"),
                city: String::from("New York"),
                org: String::from("Tech Inc."),
            },
            User {
                id: ID::from("2"),
                name: String::from("Bob"),
                email: String::from("bob@email.com"),
                phone: String::from("123-456-7891"),
                address: String::from("321 Main Street"),
                city: String::from("Paris"),
                org: String::from("Org 2."),
            },
        ]
    }
}
