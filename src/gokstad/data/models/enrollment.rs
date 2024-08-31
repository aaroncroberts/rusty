use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};

use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::data::models::section::Section;
use crate::gokstad::data::models::person::Person;
use crate::gokstad::data::models::role::Role;

///
/// Enrollment struct
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Enrollment
{
    pub id: u64,
    pub person: Person,
    pub section: Section,   
    pub role: Role
}

///
/// Enrollment: DbEntity implementation
/// 
impl DbEntity for Enrollment {
    ///
    /// key implementation for DbEntity
    /// 
    fn key(&self) -> u64 {
        self.id
    }

    ///
    /// new() implementation for the Enrollment
    /// 
    fn new() -> Self {
        info!("Enrollment::new()");

        Self {
            id: 0,
            person: Person::new(),
            section: Section::new(),
            role: Role::new()
        }
    }
}
///
/// Format implementation
/// 
impl fmt::Display for Enrollment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id: {}, person: {}, section: {}, role: {}",self.id, self.person, self.section, self.role)
    }
}