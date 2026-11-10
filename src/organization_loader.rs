use std::collections::HashMap;
use std::convert::Infallible;

use async_graphql::dataloader::Loader;

use crate::db::DB;
use crate::user_service::Organization;

pub struct OrganizationLoader {
    db: DB,
}

impl OrganizationLoader {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl Loader<String> for OrganizationLoader {
    type Value = Organization;
    type Error = Infallible;

    async fn load(
        &self,
        keys: &[String],
    ) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("Batch loading organizations: {:?}", keys);

        let organizations = self.db.get_organizations_by_ids(keys);

        Ok(organizations)
    }
}
