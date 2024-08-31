
use log::info;
use serde::{ Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DbQuery {
    pub name: String,
    pub text: String
}

#[allow(dead_code)]
///
/// DbQuery Implementation
/// 
impl DbQuery {
    ///
    /// creates a new DbQuery instance
    /// 
    pub fn new() -> DbQuery {
        info!("Creating new DbQuery...");
        
        DbQuery {
            name: "".to_string(),
            text: "".to_string()
        }
    }
}