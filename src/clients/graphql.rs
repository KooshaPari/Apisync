//! GraphQL client for making outbound queries and mutations.
//!
//! `GraphQlClient` sends HTTP POST requests to a remote GraphQL endpoint,
//! handles JSON serialization, and parses typed responses.

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A GraphQL request body.
#[derive(Debug, Serialize)]
struct GraphQlRequest<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationName")]
    operation_name: Option<&'a str>,
}

/// A GraphQL response envelope.
#[derive(Debug, Deserialize)]
struct GraphQlResponse<T> {
    data: Option<T>,
    #[serde(default)]
    errors: Vec<GraphQlError>,
}

/// A single GraphQL error from the server.
#[derive(Debug, Deserialize)]
struct GraphQlError {
    message: String,
}

/// A convenience client for GraphQL endpoints.
///
/// # Examples
///
/// ```no_run
/// # use apisync::clients::GraphQlClient;
/// # #[derive(serde::Deserialize)]
/// # struct Items { items: Vec<serde_json::Value> }
/// # async fn example() -> Result<(), apisync::error::Error> {
/// let client = GraphQlClient::new("https://api.example.com/graphql");
/// let result: Items = client.query("{ items { id name } }").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct GraphQlClient {
    client: Client,
    endpoint: String,
}

impl GraphQlClient {
    /// Create a new client targeting the given GraphQL endpoint URL.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self { client: Client::new(), endpoint: endpoint.into() }
    }

    /// Create a client with a custom [`reqwest::Client`].
    pub fn with_client(client: Client, endpoint: impl Into<String>) -> Self {
        Self { client, endpoint: endpoint.into() }
    }

    /// Return a reference to the underlying [`reqwest::Client`].
    pub fn inner(&self) -> &Client {
        &self.client
    }

    /// Return the endpoint URL.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Execute a GraphQL query and deserialize the `data` field.
    pub async fn query<T: DeserializeOwned>(&self, query: &str) -> Result<T> {
        self.execute(query, None, None).await
    }

    /// Execute a GraphQL query with variables.
    pub async fn query_with_vars<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T> {
        self.execute(query, Some(variables), None).await
    }

    /// Execute a named GraphQL operation.
    pub async fn query_named<T: DeserializeOwned>(
        &self,
        query: &str,
        operation_name: &str,
    ) -> Result<T> {
        self.execute(query, None, Some(operation_name)).await
    }

    /// Execute a GraphQL operation with full control over variables and operation name.
    pub async fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        operation_name: Option<&str>,
    ) -> Result<T> {
        let body = GraphQlRequest { query, variables, operation_name };

        let resp = self.client.post(&self.endpoint).json(&body).send().await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Http {
                status: status.as_u16(),
                body: Some(resp.text().await.unwrap_or_default()),
            });
        }

        let gql_resp: GraphQlResponse<T> = resp.json().await?;

        if !gql_resp.errors.is_empty() {
            let msgs: Vec<String> = gql_resp.errors.into_iter().map(|e| e.message).collect();
            return Err(Error::GraphQl(msgs));
        }

        gql_resp.data.ok_or_else(|| Error::Internal("GraphQL response missing data field".into()))
    }

    /// Execute a query and return the raw [`serde_json::Value`].
    pub async fn query_raw(&self, query: &str) -> Result<serde_json::Value> {
        let body = GraphQlRequest { query, variables: None, operation_name: None };

        let resp = self.client.post(&self.endpoint).json(&body).send().await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Http {
                status: status.as_u16(),
                body: Some(resp.text().await.unwrap_or_default()),
            });
        }

        let value: serde_json::Value = resp.json().await?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_construction() {
        let client = GraphQlClient::new("https://api.example.com/graphql");
        assert_eq!(client.endpoint(), "https://api.example.com/graphql");
    }

    #[test]
    fn test_client_is_clone() {
        let client = GraphQlClient::new("https://api.example.com/graphql");
        let _cloned = client.clone();
    }

    #[test]
    fn test_client_is_debug() {
        let client = GraphQlClient::new("https://api.example.com/graphql");
        let _dbg = format!("{:?}", client);
    }
}
