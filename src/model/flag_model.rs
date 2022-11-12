pub struct Flag {
    pub id: i32,
    pub name: String,
    pub site_id: i32
}

impl Flag {
    pub fn new(name: String, site_id: i32) -> Self {
        Self {
            id: 0,
            name,
            site_id
        }
    }

    /// Get the flag id
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the flag name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the flag site_id
    pub fn site_id(&self) -> i32 {
        self.site_id
    }
}