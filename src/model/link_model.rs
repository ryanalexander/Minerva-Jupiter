use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// New link
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewLink {
    pub url: String,
}

impl NewLink {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Domain {
    pub domain: std::option::Option<String>
}



/// Existing link
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub domain: std::option::Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_scraped: std::option::Option<NaiveDateTime>,
    pub scraped: bool,
    pub site_id: std::option::Option<i32>,
}

impl Link {
    pub fn new(url: String) -> Self {
        Self {
            id: 0,
            url,
            domain: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            last_scraped: None,
            scraped: false,
            site_id: Some(0),
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
