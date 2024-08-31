
use crate::gokstad::error::Exception;
use crate::gokstad::data::db_provider::DbProvider;
use crate::gokstad::data::db_entity::DbEntity;
use crate::gokstad::data::db_config::DbConfig;

///
/// A DbRepository
/// 
pub trait DbRepository {
    type TEntity: DbEntity;
    type TProvider: DbProvider;
    ///
    /// 
    /// 
    fn init(config: &DbConfig) -> Self;
    ///
    /// Lists all items
    /// 
    fn list(&self) -> Result<Vec<Self::TEntity>,Exception>; 
    ///
    /// Gets an item 
    /// 
    fn get(&self, id: &u64) -> Result<Self::TEntity,Exception>;
    ///
    /// Adds the item
    /// 
    fn add(&self, item: &Self::TEntity) -> Result<Self::TEntity,Exception>;
    ///
    /// Updates an item 
    /// 
    fn update(&self, item: &Self::TEntity) -> Result<Self::TEntity,Exception>;
    ///
    /// map function to create a TEntity
    /// 
    fn map(&self) -> Result<Self::TEntity, Exception>;
}