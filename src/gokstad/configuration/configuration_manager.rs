use log::info;
use std::{fs::File};
use std::io::Read;
use serde_json::{ Result };
use crate::gokstad::configuration::app_settings::*;

pub struct ConfigurationManager{
    pub file: String,
    pub app_settings: AppSettings
}

///
/// ConfigurationManager implementation 
/// 
impl ConfigurationManager { 
    ///
    /// Construcst a new ConfigurationManager
    /// 
    pub fn new() -> ConfigurationManager {

        info!("Creating new DbQuery...");

        let settings = AppSettings::new();
        ConfigurationManager 
        { 
            file: "".to_string(),
            app_settings: settings,
        }
    }   
    ///
    ///  Loads a config from the file
    /// 
    pub fn init(&self, path: &str) -> Result<AppSettings> {
        info!("Loading config: {}", path);
        
        // Open the file
        let file = File::open(path);
        let mut file =  match file {
            Ok(file) => file,
            Err(error) => panic!("Error opening file: {:?}", error),
        };
        
        // Read in the file contents 
        let mut data = String::new();
        let size = match file.read_to_string(&mut data) {
            Ok(size) => size,
            Err(error) => panic!("Unable to deserialize object: {:?}", error),
        };    
        
        // Check if we have data
        if size < 1 {
            panic!("No data was loaded from the file!")
        }
                
        // Desrialize the object
        let settings = serde_json::from_str(&data.as_str());
        let settings = match settings {
            Ok(settings) => settings,
            Err(error) => panic!("Unable to deserialize object: {:?}", error),
        };

        Ok(settings)        
    }  
}
