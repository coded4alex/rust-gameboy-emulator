use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Display {
    handle: Option<JoinHandle<()>>,
}

pub fn create_display(window_name: &str) -> Display {
    let window_name_clone = String::from(window_name);
    let thandle = thread::spawn(move || {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(window_name_clone.as_str(), 800, 600).position_centered().build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    });
    Display { handle: Some(thandle) }
}

impl Display {
    pub fn join(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}
