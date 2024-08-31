
use log::info;
use oracle::{Connection, Error};
use crate::gokstad::error::Exception;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_provider::DbProvider;

pub struct OracleDbProvider {
    pub config: DbConfig,
}

impl DbProvider for OracleDbProvider {
    /// 
    ///
    /// Constructs a new PostgresDbProvider
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("OracleDbProvider::init()...");
        Self {
            config: config.clone()
        }
    }
    ///
    /// Executes a query against the database
    /// 
    fn execute_query(&self, query: &str) -> Result<bool, Exception> {     

        let conn = Connection::connect("C6006113", "9cBiZ2i&4&FzS4UK", "cprd-db.csuohio.edu:1678/cprd")
            .expect("Unable to connect to Oracle"); 

        let sql = query;
        let rows = conn.query(sql, &[]).unwrap();
        for r in rows {
            let row = r.unwrap();
            let conto: String = row.get("CONTO").unwrap();
            println!("{}", conto);
        }
        
        Ok(true)     
    }

    ///
    /// Executes a non_query against the database
    /// 
    fn execute_non_query(&self, query: &str) -> Result<bool, Exception>{
        let connected = OracleDbProvider::connect(&self).unwrap();

        info!("connected: {:?}", connected);
        info!("non-query: {:?}", query); 

        Ok(true)             
    }

    ///
    /// Executes a scalar against the database
    /// 
    fn execute_scalar(&self, query: &str) -> Result<bool, Exception>{
        let connected = OracleDbProvider::connect(&self).unwrap();

        info!("connected: {:?}", connected);
        info!("scalar: {:?}", query); 

        Ok(true)             
    }

    ///
    /// Connects to the dbStore
    /// 
    fn connect(&self) -> Result<bool, Exception> {  

        // status                       
        info!("{}", &self.config.to_string());

        Ok(true)
    } 
}