use log::info;

use crate::app::config::Config;

use crate::memory::access::Memory;
use crate::memory::access::create_memory;

use crate::memory::router::Bank;
use crate::memory::router::create_banks;
use crate::memory::router::init_banks;

pub struct Application {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub memory: Memory,
    pub banks: Vec<Bank>,
}

impl Application {
    pub fn new(config: &Config) -> Application {
        Application {
            name: config.name.clone(),
            version: config.version.clone(),
            author: config.author.clone(),
            description: config.description.clone(),

            memory: create_memory(),
            banks: create_banks(),
        }
    }

    pub fn init(&mut self) {
        info!("Initializing {}", self.name);

        info!("Initializing memory");
        info!("Initializing banks");
        init_banks(&mut self.banks);
    }

    pub fn run(&mut self) {
        println!("{} v{} by {} ({})", self.name, self.version, self.author, self.description);
    }
}
   