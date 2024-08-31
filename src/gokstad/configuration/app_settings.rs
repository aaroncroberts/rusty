use super::super::data::db_config::DbConfig;

use serde::{ Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSettings {
    pub db_config: DbConfig
}
///
/// AppSettings Implementation
/// 
impl AppSettings {
    ///
    /// Constructs a new AppSettings instance
    /// 
    pub fn new() -> AppSettings {
       let config = DbConfig::new();
       AppSettings { db_config: config }
    }
}
