
use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_query::DbQuery;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DbConfig {
    pub server: String,
    pub username: String,
    pub password: String,
    pub port: u32,
    pub queries: Vec<DbQuery>
}
///
/// Format Displau for the Config
/// 
impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"|> {}:{} || @u: {}",self.server, self.port, self.username)
    }
}
///
/// DbConfig Implementation
/// 
impl DbConfig {
    ///
    /// Constructs a new DbConfig
    /// 
    pub fn new() -> DbConfig {
        info!("Creating new DbConfig...");

        // Create the storage
        let queries: Vec<DbQuery> = Vec::new();

        DbConfig 
        {
            server: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            port: 1234,
            queries: queries
        }
    }
}
