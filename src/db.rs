use std::collections::HashMap;

use async_graphql::ID;

use crate::user_service::{Organization, User};

#[derive(Clone, Copy)]
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
                org_id: String::from("org-1"),
            },
            User {
                id: ID::from("2"),
                name: String::from("Bob"),
                email: String::from("bob@email.com"),
                phone: String::from("123-456-7891"),
                address: String::from("321 Main Street"),
                city: String::from("Paris"),
                org_id: String::from("org-2"),
            },
            User {
                id: ID::from("3"),
                name: String::from("Charlie"),
                email: String::from("charlie@email.com"),
                phone: String::from("123-456-7892"),
                address: String::from("456 Main Street"),
                city: String::from("London"),
                org_id: String::from("org-1"),
            },
        ]
    }

    pub fn get_organizations_by_ids(&self, ids: &[String]) -> HashMap<String, Organization> {
        let organizations = vec![
            Organization {
                id: ID::from("org-1"),
                title: String::from("Tech Inc."),
            },
            Organization {
                id: ID::from("org-2"),
                title: String::from("Finance Corp."),
            },
        ];

        organizations
            .into_iter()
            .filter(|org| ids.contains(&org.id.0))
            .map(|org| (org.id.0.clone(), org))
            .collect()
    }
}
