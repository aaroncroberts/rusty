
///
/// Trait for a DbEntity
/// 
pub trait DbEntity {  
    ///
    /// Creates a new Entity
    ///   
    fn new() -> Self; 
    ///
    /// Gets the key for the entity
    /// 
    fn key(&self) -> u64;
}

