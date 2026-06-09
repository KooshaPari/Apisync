//! GraphQL adapter using async-graphql
//!
//! Provides schema parsing, query execution, and mutation handling.

use async_graphql::{EmptySubscription, InputObject, Object, Schema, SimpleObject, ID};

/// GraphQL representation of an item.
#[derive(SimpleObject, Clone)]
pub struct GraphItem {
    pub id: ID,
    pub name: String,
    pub description: String,
}

/// Input for creating a new item.
#[derive(InputObject)]
pub struct CreateGraphItem {
    pub name: String,
    pub description: String,
}

/// Input for updating an existing item.
#[derive(InputObject)]
pub struct UpdateGraphItem {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Query root for the GraphQL schema.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Fetch a single item by id.
    async fn item(&self, id: ID) -> Option<GraphItem> {
        Some(GraphItem {
            id,
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
        })
    }

    /// List all items.
    async fn items(&self) -> Vec<GraphItem> {
        vec![
            GraphItem {
                id: "1".into(),
                name: "Item1".to_string(),
                description: "Desc1".to_string(),
            },
            GraphItem {
                id: "2".into(),
                name: "Item2".to_string(),
                description: "Desc2".to_string(),
            },
        ]
    }
}

/// Mutation root for the GraphQL schema.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new item.
    async fn create_item(&self, input: CreateGraphItem) -> GraphItem {
        GraphItem {
            id: "1".into(),
            name: input.name,
            description: input.description,
        }
    }

    /// Update an existing item.
    async fn update_item(&self, id: ID, input: UpdateGraphItem) -> Option<GraphItem> {
        Some(GraphItem {
            id,
            name: input.name.unwrap_or_else(|| "Unnamed".to_string()),
            description: input.description.unwrap_or_else(|| "No description".to_string()),
        })
    }
}

/// The GraphQL schema type used by this adapter.
pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Build and return a new GraphQL schema.
pub fn build_schema() -> GraphQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}

/// Execute a GraphQL query against the provided schema.
pub async fn execute_query(schema: &GraphQLSchema, query: &str) -> async_graphql::Response {
    schema.execute(query).await
}

/// Execute a GraphQL mutation against the provided schema.
pub async fn execute_mutation(schema: &GraphQLSchema, mutation: &str) -> async_graphql::Response {
    schema.execute(mutation).await
}
