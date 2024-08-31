use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_entity::DbEntity;

///
/// Section struct
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Section
{
    pub id: u64,
    pub code: String,
    pub name: String,
    pub number: String   
}

///
/// Section: DbEntity implementation
/// 
impl DbEntity for Section {
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
        info!("Section::new()");
        
        Self {
            id: 0,
            code: String::from("<code />"),
            name: String::from("<name />"),
            number: String::from("<number />")
        }
    }
}
///
/// Section implementation
/// 
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id: {}, code: {}, name: {}, number: {}", self.id, self.code, self.name, self.number)
    }
}