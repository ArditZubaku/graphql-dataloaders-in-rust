use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, ID, SimpleObject};

use crate::organization_loader::OrganizationLoader;

#[derive(Clone, SimpleObject)] // SimpleObject generates the getters with the correct return type mappings
#[graphql(complex)] // Tells SimpleObject that there's a complex companion block
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,

    #[graphql(skip)]
    // Skip auto-generating the getter for this field (I'll implement the logic myself)
    pub org_id: String,
}

// Custom return type
#[derive(SimpleObject, Clone)]
pub struct Organization {
    pub id: ID,
    pub title: String,
}

// Use #[ComplexObject] instead of #[Object] for custom/overridden fields
#[ComplexObject]
impl User {
    async fn org(&self, ctx: &Context<'_>) -> Option<Organization> {
        let loader = ctx.data_unchecked::<DataLoader<OrganizationLoader>>();

        // We are no longer querying the database directly from the User resolver.
        // Instead, we delegate the lookup to the DataLoader.
        loader.load_one(self.org_id.clone()).await.ok().flatten()
    }
}
