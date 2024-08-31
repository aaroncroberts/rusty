use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::enrollment::Enrollment;

///
/// EnrollmentService: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct EnrollmentService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// SectionService Repository implementation
/// 
impl DbRepository for EnrollmentService {

    type TEntity = Enrollment;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("EnrollmentService::init()");       
        let provider = OracleDbProvider::init(config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the Enrollment
    /// 
    fn list(&self) -> Result<Vec<Enrollment>,Exception>{ 
        info!("EnrollmentService::list()");  
     
        let enrollment : Vec<Enrollment> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(enrollment)
    } 
    ///
    /// gets one Enrollment record
    /// 
    fn get(&self, _id: &u64) -> Result<Enrollment, Exception>{
        info!("EnrollmentService::get()");

        let enrollment : Enrollment = Enrollment::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(enrollment)
    }
    ///
    /// Adds a Enrollment
    /// 
    fn add(&self, _item: &Enrollment) -> Result<Enrollment,Exception>{
        info!("EnrollmentService::add()");

        let enrollment : Enrollment = Enrollment::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(enrollment)
    }
    ///
    /// Updates the Enrollment 
    /// 
    fn update(&self, _item: &Enrollment) -> Result<Enrollment,Exception>{
        info!("EnrollmentService::update()");
        
        let enrollment : Enrollment = Enrollment::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(enrollment)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Enrollment, Exception> {
        let enrollment = Enrollment::new();

        Ok(enrollment)
    }
}