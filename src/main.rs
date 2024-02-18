pub mod memory;
pub mod app;
pub mod devices;
pub mod graphics;
pub mod screen;

use app::config::Config;
use app::core::Application;

fn main() {
    let config = Config::new(String::from("config/config.yml")); 

    let mut app = Application::new(&config);
    app.init();
    app.run();
}
