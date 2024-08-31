use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_entity::DbEntity;

///
/// Role struct
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role
{
    pub id: u64,
    pub name: String,   
    pub description: String    
}

///
/// Role: DbEntity implementation
/// 
impl DbEntity for Role {
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
        info!("Role::new()");
        
        Self {
            id: 0,
            name: String::from("<name />"),
            description: String::from("<description />"),
        }
    }
}
///
/// Format implementation
/// 
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id: {}, name: {}, description: {}", self.id, self.name, self.description)
    }
}