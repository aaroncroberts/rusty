use log::{
    info
};
use crate::gokstad::data::providers::oracle::OracleDbProvider;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::db_config::DbConfig;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::error::Exception;
use crate::gokstad::data::models::role::Role;

///
/// RoleService: DbRepository implementation
/// 
#[allow(dead_code)]
pub struct RoleService
{
    config: DbConfig,
    provider: OracleDbProvider,
}
///
/// RoleService Repository implementation
/// 
impl DbRepository for RoleService {

    type TEntity = Role;
    type TProvider = OracleDbProvider;

    ///
    /// Initializes a new Provider to work with
    /// 
    fn init(config: &DbConfig) -> Self {
        info!("RoleService::init()");       
        let provider = OracleDbProvider::init(&config);  

        Self {
            config: config.clone(),
            provider: provider
        }
    }
    ///
    /// lists all the Role
    /// 
    fn list(&self) -> Result<Vec<Role>,Exception>{ 
        info!("RoleService::list()");  
     
        let role : Vec<Role> = Vec::new();
        let query = &self.provider.config.queries[1].text;
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(role)
    } 
    ///
    /// gets one Role record
    /// 
    fn get(&self, _id: &u64) -> Result<Role, Exception>{
        info!("RoleService::get()");

        let role : Role = Role::new();
        let query = &self.provider.config.queries[0].text;        
        let _success = &self.provider.execute_query(query);

        // Map to object

        Ok(role)
    }
    ///
    /// Adds a Role
    /// 
    fn add(&self, _item: &Role) -> Result<Role,Exception>{
        info!("RoleService::add()");

        let role : Role = Role::new();
        let query = &self.provider.config.queries[2].text; 
        let _success = &self.provider.execute_query(query);

        // Map to object
        
        Ok(role)
    }
    ///
    /// Updates the Role 
    /// 
    fn update(&self, _item: &Role) -> Result<Role,Exception>{
        info!("RoleService::update()");
        
        let role : Role = Role::new();
        let query = &self.provider.config.queries[3].text; 
        let _success = &self.provider.execute_query(query);
    
        // Map to object;

        Ok(role)
    }
    ///
    /// Creates an instance of an Entity
    /// 
    fn map(&self) -> Result<Role, Exception> {
        let role = Role::new();

        Ok(role)
    }
}