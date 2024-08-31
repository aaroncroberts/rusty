use std::fmt;
use log::info;
use serde::{ Deserialize, Serialize};
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::data::models::course::Course;
use crate::gokstad::data::models::section::Section;
use crate::gokstad::data::models::term::Term;

///
/// Offering struct
/// 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Offering
{
    pub id: u64,
    pub code: String,
    pub name: String,
    pub course: Course,
    pub term: Term,
    pub sections: Vec<Section>,
}

///
/// Offering: DbEntity implementation
/// 
impl DbEntity for Offering {
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
        info!("Offering::new()");

        Self {
            id: 0,
            code: String::from("<code />"),
            name: String::from("<name />"),
            course: Course::new(),
            term: Term::new(),
            sections: Vec::new()
        }
    }
}
///
/// Format implementation
/// 
impl fmt::Display for Offering {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"id:{}, code: {}, name: {}, term: {}",self.id, self.code, self.name, self.term)
    }
}