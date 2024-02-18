use std::thread::{self, JoinHandle};

pub struct Display {
    handle: Option<JoinHandle<()>>,
}

pub fn create_display() -> Display {
    let thandle = thread::spawn(|| {
        println!("Hello");
    });
    Display { handle: Some(thandle) }
}

impl Display {
    pub fn join(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}
