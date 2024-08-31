use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::person::Person;

///
/// People: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct PeopleService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// PeopleService Repository implementation
/// 
impl DbRepository for PeopleService {

    type TEntity = Person;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("PeopleService::init()");       
        let provider = OracleDbProvider::init(&config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the people
    /// 
    fn list(&self) -> Result<Vec<Person>,Exception>{ 
        info!("PeopleService::list()");  
     
        let people : Vec<Person> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(people)
    } 
    ///
    /// gets one people record
    /// 
    fn get(&self, _id: &u64) -> Result<Person, Exception>{
        info!("PeopleService::get()");

        let people : Person = Person::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(people)
    }
    ///
    /// Adds a person
    /// 
    fn add(&self, _item: &Person) -> Result<Person,Exception>{
        info!("PeopleService::add()");

        let people : Person = Person::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(people)
    }
    ///
    /// Updates the Person 
    /// 
    fn update(&self, _item: &Person) -> Result<Person,Exception>{
        info!("PeopleService::update()");
        
        let people : Person = Person::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(people)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Person, Exception> {
        let people = Person
        {
            id: 1,
            name: "Person".to_string()
        };

        Ok(people)
    }
}