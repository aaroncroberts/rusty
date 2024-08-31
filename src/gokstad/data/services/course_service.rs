use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::course::Course;

///
/// People: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct CourseService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// CourseService Repository implementation
/// 
impl DbRepository for CourseService {

    type TEntity = Course;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("CourseService::init()");       
        let provider = OracleDbProvider::init(config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the people
    /// 
    fn list(&self) -> Result<Vec<Course>,Exception>{ 
        info!("CourseService::list()");  
     
        let course : Vec<Course> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(course)
    } 
    ///
    /// gets one people record
    /// 
    fn get(&self, _id: &u64) -> Result<Course, Exception>{
        info!("CourseService::get()");

        let course : Course = Course::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(course)
    }
    ///
    /// Adds a person
    /// 
    fn add(&self, _item: &Course) -> Result<Course,Exception>{
        info!("CourseService::add()");

        let course : Course = Course::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(course)
    }
    ///
    /// Updates the Person 
    /// 
    fn update(&self, _item: &Course) -> Result<Course,Exception>{
        info!("CourseService::update()");
        
        let course : Course = Course::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(course)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Course, Exception> {
        let course = Course::new();

        Ok(course)
    }
}