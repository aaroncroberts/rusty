///
/// 
/// Initializes the tracing and logging
/// 

use std::io;
use log::info;
use tracing_subscriber::{ Registry, prelude::* };
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::{        
    self, 
    format, 
};

///
/// Initalizes the Logging
pub fn init_logger() -> WorkerGuard {
    // log file appender
    let log_file = tracing_appender::rolling::daily("log", "rusty.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(log_file);

    // console appender
    let stdout = io::stdout.with_max_level(tracing::Level::INFO);  
    
    // Layer - not colored for file
    let log_file = fmt::Layer::new()
        .event_format(format()               
            .with_target(true)                                              
            .with_file(true)  
            .with_level(true)                  
            .with_line_number(true)
            .with_ansi(false))
        .with_writer(non_blocking_file);

    // Layer - colored for stdout
    let stdout = fmt::Layer::new()
        .event_format(format()
            .with_target(true)                                              
            .with_file(true)  
            .with_level(true)                  
            .with_line_number(true)
            .with_ansi(true))
        .with_writer(stdout);

        // Init the subscriber with the layers
        let _subscriber = Registry::default()
            .with(log_file)
            .with(stdout)
            .init();

    info!("Tracing configured!");
                                        
    _guard
}