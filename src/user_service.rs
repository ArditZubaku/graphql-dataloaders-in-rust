use async_graphql::{ID, SimpleObject, ComplexObject};

#[derive(Clone, SimpleObject)] // SimpleObject generates the getters with the correct return type mappings
#[graphql(complex)] // Tells SimpleObject that there's a complex companion block
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,

    #[graphql(skip)] // Skip auto-generating the getter for this field (I'll implement the logic myself)
    pub org: String,
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
    async fn org(&self) -> Organization {
        // Custom logic or custom return type here
        Organization {
            id: ID::from("org-123"),
            title: self.org.clone(),
        }
    }
}
