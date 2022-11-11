use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


/// New link
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewLink {
    pub url: String,
}

impl NewLink {
    pub fn new(url: String) -> Self {
        Self {
            url
        }
    }
}

/// Existing link
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
    pub lastScraped: std::option::Option<NaiveDateTime>,
    pub scraped: bool,
    pub siteId: std::option::Option<i32>,
}

impl Link {
    pub fn new(url: String) -> Self {
        Self {
            id: 0,
            url,
            createdAt: chrono::Utc::now().naive_utc(),
            updatedAt: chrono::Utc::now().naive_utc(),
            lastScraped: None,
            scraped: false,
            siteId: Some(0),
        }
    }

    /// Get the link id
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the link url
    pub fn url(&self) -> &str {
        &self.url
    }
}