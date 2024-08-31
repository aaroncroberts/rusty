
use log::info;
use crate::gokstad::error::Exception;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_provider::DbProvider;

pub struct PostgresDbProvider {
    pub config: DbConfig,
}

impl DbProvider for PostgresDbProvider {
    /// 
    ///
    /// Constructs a new PostgresDbProvider
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("init() PostgresDbProvider...");
        Self {
            config: config.clone()
        }
    }
    ///
    /// Executes a query against the database
    /// 
    fn execute_query(&self, query: &str) -> Result<bool, Exception> {
        // Connect to the instance        
        let connected = PostgresDbProvider::connect(&self).unwrap();

        info!("connected: {:?}", connected);
        info!("querying: {:?}", query);  

        Ok(true)      
    }

    ///
    /// Executes a non_query against the database
    /// 
    fn execute_non_query(&self, query: &str) -> Result<bool, Exception>{
        // Connect to the instance
        let connected = PostgresDbProvider::connect(&self).unwrap();

        info!("connected: {:?}", connected);
        info!("non-query: {:?}", query); 

        Ok(true)             
    }

    ///
    /// Executes a scalar against the database
    /// 
    fn execute_scalar(&self, query: &str) -> Result<bool, Exception>{
        // Connect to the instance
        let connected = PostgresDbProvider::connect(&self).unwrap();

        info!("connected: {:?}", connected);
        info!("scalar: {:?}", query); 

        Ok(true)             
    }

    ///
    /// Connects to the dbStore
    /// 
    fn connect(&self) -> Result<bool, Exception> {                         
        info!("Connecting to: {:?}", &self.config);

        Ok(true)
    } 
}