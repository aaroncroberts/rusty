use crate::gokstad::error::Exception;
use crate::gokstad::data::db_config::DbConfig;

///
/// A DbProvider trait specification
/// 
pub trait DbProvider {
    ///
    /// Creates a new DbProvider
    /// 
    fn init(config: &DbConfig) -> Self;  
    ///
    /// A function to connect to the database
    /// 
    fn connect(&self) -> Result<bool,Exception>;
    ///
    /// A function to execute a query against a database
    /// 
    fn execute_query(&self, query: &str) -> Result<bool,Exception>;
    ///
    /// A function to exeucte a non result set query 
    /// 
    fn execute_non_query(&self, query: &str) -> Result<bool,Exception>;
    ///
    /// A function to execute a scalar return (single value)
    /// 
    fn execute_scalar(&self, query: &str) -> Result<bool,Exception>;
}