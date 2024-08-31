use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::offering::Offering;

///
/// Offering: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct OfferingService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// OfferingService Repository implementation
/// 
impl DbRepository for OfferingService {

    type TEntity = Offering;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("OfferingService::init()");       
        let provider = OracleDbProvider::init(config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the Offerings
    /// 
    fn list(&self) -> Result<Vec<Offering>,Exception>{ 
        info!("OfferingService::list()");  
     
        let offering : Vec<Offering> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(offering)
    } 
    ///
    /// gets one Offering record
    /// 
    fn get(&self, _id: &u64) -> Result<Offering, Exception>{
        info!("OfferingService::get()");

        let offering : Offering = Offering::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(offering)
    }
    ///
    /// Adds an Offering
    /// 
    fn add(&self, _item: &Offering) -> Result<Offering,Exception>{
        info!("OfferingService::add()");

        let offering : Offering = Offering::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(offering)
    }
    ///
    /// Updates the Offering 
    /// 
    fn update(&self, _item: &Offering) -> Result<Offering,Exception>{
        info!("OfferingService::update()");
        
        let offering : Offering = Offering::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(offering)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Offering, Exception> {
        let offering = Offering::new();
        

        Ok(offering)
    }
}