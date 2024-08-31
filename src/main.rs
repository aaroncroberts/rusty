use std::io;
use log::{
    info,
    error
};

mod gokstad;
use crate::gokstad::configuration::configuration_manager::{
    ConfigurationManager
};
use crate::gokstad::logging;
use crate::gokstad::data::db_repository::DbRepository;
use crate::gokstad::data::services::people_service::PeopleService;
use crate::gokstad::data::services::term_service::TermService;
use crate::gokstad::data::services::course_service::CourseService;

/// 
/// Main program execution
/// 
fn main() {

    let _gaurd = logging::init_logger();

    info!("booting up");

    println!("Welcome to Rust!");
    println!("---------------------------");
   
    // Init our configuration
    let settings = ConfigurationManager::new()
        .init("src/appsettings.json")
        .expect("Unable to load configuration file!");

    // Create our dbStore instance
    let config = settings.db_config;
    let people = PeopleService::init(&config);
    let terms = TermService::init(&config);
    let courses = CourseService::init(&config);

    //
    // Get some people...
    let result = people.list();
    let _result = match result {
        Ok(result) => {
            info!("Success!");   
            for item in result {
                info!("  name: {}, id: {}", item.name, item.id);
            }        
        },
        Err(error) => error!("{}",error)
    };
    
    //
    // Get a Term
    let id : u64 = 1;
    let result = terms.get(&id);
    let _result = match result {
        Ok(result) => {
            let item = &result;
            info!("{:}", item);   
        },
        Err(error) => error!("{}",error)
    };

    //
    // List the Courses
    let id : u64 = 123;
    let result = courses.get(&id);
    let _result = match result {
        Ok(result) => {
            let item = &result;
            info!("{:}", item);   
        },
        Err(error) => error!("{}",error)
    };

    
    // Input
    let mut guess = String::new();
    // Read
    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read input!");   
}