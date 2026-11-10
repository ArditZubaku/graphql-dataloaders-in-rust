use async_graphql::Object;

use crate::{db::DB, user_service::User};

pub struct Query {
    pub db: DB,
}

#[Object]
impl Query {
    // Here one defines the "endpoints"

    async fn get_user(&self, id: String) -> Option<User> {
        self.db.get_data().into_iter().find(|user| user.id.0 == id)
    }

    async fn get_users(&self) -> Vec<User> {
        self.db.get_data()
    }
}
