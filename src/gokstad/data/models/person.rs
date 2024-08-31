use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_entity::DbEntity;

///
/// Person struct
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Person
{
    pub id: u64,
    pub name: String,   
}

///
/// Person: DbEntity implementation
/// 
impl DbEntity for Person {
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
        info!("Person::new()");

        Self {
            id: 0,
            name: String::from("<name />")
        }
    }
}
///
/// Format implementation
/// 
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id: {}, name: {}",self.id, self.name)
    }
}