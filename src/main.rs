mod blink_reader;
mod message;

use blink_reader::BlinkReader;
use iced::{Application, Settings};

fn main() -> iced::Result {
    BlinkReader::run(Settings{
        antialiasing: true,
        ..Settings::default()
    })
}