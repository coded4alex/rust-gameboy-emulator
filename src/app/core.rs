use crate::app::config::Config;
use log::info;

use crate::screen::display::create_display;
use crate::screen::display::Display;

use super::gameboy::Gameboy;

pub struct Application {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub display: Display,
    pub cpu: Gameboy,
}

impl Application {
    pub fn new(config: Config) -> Application {
        Application {
            name: config.name.clone(),
            version: config.version.clone(),
            author: config.author.clone(),
            description: config.description.clone(),

            display: create_display(config.name.as_str()),
            cpu: Gameboy::create(),
        }
    }

    pub fn init(&mut self) {
        info!("Initializing {}", self.name);
    }

    pub fn run(&mut self) {
        println!("{} v{} by {} ({})", self.name, self.version, self.author, self.description);
        self.display.join();
    }
}

mod test {
    use crate::app::config::Config;

    use super::Application;

    #[test]
    fn test_loads() {
        let config = Config::new(String::from("config/config.yml"));

        let mut app = Application::new(config);
        app.init();
    }
}
