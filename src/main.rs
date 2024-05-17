mod blink_reader;
mod message;

use blink_reader::BlinkReader;
use iced::Application;

fn main() -> iced::Result {
    BlinkReader::run(Default::default())
}