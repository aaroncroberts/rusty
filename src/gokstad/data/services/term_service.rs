use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::term::Term;

///
/// TermService: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct TermService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// TermService Repository implementation
/// 
impl DbRepository for TermService {

    type TEntity = Term;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("TermService::init()");       
        let provider = OracleDbProvider::init(&config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the people
    /// 
    fn list(&self) -> Result<Vec<Term>,Exception>{ 
        info!("TermService::list()");  
     
        let term : Vec<Term> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(term)
    } 
    ///
    /// gets one people record
    /// 
    fn get(&self, _id: &u64) -> Result<Term, Exception>{
        info!("TermService::get()");

        let term : Term = Term::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(term)
    }
    ///
    /// Adds a person
    /// 
    fn add(&self, _item: &Term) -> Result<Term,Exception>{
        info!("TermService::add()");

        let term : Term = Term::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(term)
    }
    ///
    /// Updates the Person 
    /// 
    fn update(&self, _item: &Term) -> Result<Term,Exception>{
        info!("TermService::update()");
        
        let term : Term = Term::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(term)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Term, Exception> {
        let term = Term::new();

        Ok(term)
    }
}