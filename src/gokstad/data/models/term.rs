use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_entity::DbEntity;

///
/// Role struct
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Term
{
    pub id: u64,
    pub code: String,  
    pub name: String, 
    pub description: String    
}

///
/// Term: DbEntity implementation
/// 
impl DbEntity for Term {
    ///
    /// key implementation for DbEntity
    /// 
    fn key(&self) -> u64 {
        self.id
    }

    ///
    /// new() implementation for the DbEntity
    /// 
    fn new() -> Self {
        info!("Term::new()");
        
        Self {
            id: 0,
            code: String::from("<code />"),
            name: String::from("<name />"),
            description: String::from("<description />"),
        }
    }
}
///
/// Format implementation
/// 
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id: {}, code: {}, name: {}, description: {}", self.id, self.code, self.name, self.description)
    }
}