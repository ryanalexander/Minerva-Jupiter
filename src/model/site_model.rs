use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewSite {
    pub name: String,
    pub url: String,
    pub last_online: Option<NaiveDateTime>,
}

impl NewSite { 
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            last_online: Some(chrono::Utc::now().naive_utc())
        }
    }
}

/// Existing site

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_online: Option<NaiveDateTime>,
    pub last_scrape: NaiveDateTime,
}

impl Site {
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: 0,
            name,
            url,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            last_online: None,
            last_scrape: chrono::Utc::now().naive_utc(),
        }
    }

    /// Get the site id
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the site name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the site url
    pub fn url(&self) -> &str {
        &self.url
    }
}