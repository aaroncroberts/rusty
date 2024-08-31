use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::section::Section;

///
/// SectionService: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct SectionService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// SectionService Repository implementation
/// 
impl DbRepository for SectionService {

    type TEntity = Section;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("SectionService::init()");       
        let provider = OracleDbProvider::init(&config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the Sections
    /// 
    fn list(&self) -> Result<Vec<Section>,Exception>{ 
        info!("SectionService::list()");  
     
        let section : Vec<Section> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(section)
    } 
    ///
    /// gets one Section record
    /// 
    fn get(&self, _id: &u64) -> Result<Section, Exception>{
        info!("SectionService::get()");

        let section : Section = Section::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(section)
    }
    ///
    /// Adds a Section
    /// 
    fn add(&self, _item: &Section) -> Result<Section,Exception>{
        info!("SectionService::add()");

        let section : Section = Section::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(section)
    }
    ///
    /// Updates the Section 
    /// 
    fn update(&self, _item: &Section) -> Result<Section,Exception>{
        info!("SectionService::update()");
        
        let section : Section = Section::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(section)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Section, Exception> {
        let section = Section::new();

        Ok(section)
    }
}