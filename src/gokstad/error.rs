use std::error::Error;
use std::fmt;

///
/// Exception Struct definition
/// 
#[derive(Debug)]
pub struct Exception {
    details: String,
}

#[allow(dead_code)]
///
/// Exception Implementation
/// 
impl Exception {
    ///
    /// Constructs a new Exception
    /// 
    pub fn new(message: &str) -> Exception {
        Exception
        { 
            details: message.to_string()
        }
    }
}
///
/// Display Formatter for Exception
/// 
impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}
///
/// 
/// 
impl Error for Exception {
    fn description(&self) -> &str {
        &self.details
    }
}

// // a test function that returns our error result
// fn raises_my_error(yes: bool) -> Result<(),Exception> {
//     if yes {
//         Err(Exception::new("borked"))
//     } else {
//         Ok(())
//     }
// }