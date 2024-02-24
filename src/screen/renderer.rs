use std::sync::mpsc::{Receiver, TryRecvError};

use super::display_info::DisplayInfo;

pub struct Renderer {
    pub receiver: Receiver<DisplayInfo>,
}

impl Renderer {
    pub fn render(&self) -> Result<DisplayInfo, TryRecvError> {
        let display_info = self.receiver.try_recv()?;
        Ok(display_info)
    }
}
