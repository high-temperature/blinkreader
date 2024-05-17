use std::time::Duration;

use async_std::path::Path;
use iced::theme::Theme;
use iced::widget::{button, column, container, Text};
use iced::{executor, time, Alignment, Application, Command, Element, Font, Length, Subscription};
use async_std::{self, fs};
use async_std::io::BufReader;
use async_std::prelude::*;

use crate::message::{Message, State};

const GENEIKOBURIMIN: Font = Font::External {
    name: "GenEiKoburiMin",
    bytes: include_bytes!("font/GenEiKoburiMin6-R.ttf"),
};

const CHUNK: usize = 20;

pub struct BlinkReader {
    display: String,
    state: State,
    full_text: String,
    position: usize,
    interval:Duration,
}

impl BlinkReader{
    fn reset(&mut self){
        self.position = 0;
        self.display = String::new();
    }
}

impl Application for BlinkReader {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (BlinkReader, Command<Message>) {
        let full_text_path = Path::new("D:\\Document\\Rust\\blinktextreader\\src\\text\\第1節　実体経済の動向.txt");
        let initial_text = "Loading...".to_string();

        (
            BlinkReader {
                display: initial_text.clone(),
                state: State::Idle,
                full_text: initial_text,
                position: 0,
                interval: Duration::from_secs(1),
            },
            Command::perform(
                async move {
                    
                    let file = fs::File::open(&full_text_path).await.map_err(|e| e.to_string())?;
                    let mut reader = BufReader::new(file);
                    let mut content = String::new();
                    reader.read_to_string(&mut content).await.map_err(|e| e.to_string())?;
                    //let content = content.replace("\n","").replace("\r","");
                    Ok(content)
                },
                |result| Message::FileLoaded(result)
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Blink Reader")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if self.position < self.full_text.len() {
                
                let end = self.full_text[self.position..]
                    .char_indices()
                    .nth(CHUNK)
                    .map_or(self.full_text.len(),|(idx, _ )| self.position + idx);
                
                    self.display = self.full_text[self.position..end].to_string();
                    self.position =end;
                }
                Command::none()
            },
            Message::Reset => {
                self.reset();
                Command::none()
            }
            Message::Toggle => {
                self.state = match self.state {
                    State::Idle => State::Reading,
                    State::Reading => State::Idle,
                };
                Command::none()
            },
            Message::FileLoaded(Ok(content)) => {
                self.full_text = content;
                self.reset();
                Command::none()
            },
            Message::FileLoaded(Err(err)) => {
                self.full_text = format!("Error loading file: {}", err);
                self.reset();
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let text = Text::new(&self.display).size(30).font(GENEIKOBURIMIN);
        let toggle_button = button(Text::new("Toggle")).on_press(Message::Toggle);
        let reset_button = button(Text::new("Reset")).on_press(Message::Reset);

        let content = column![text, toggle_button, reset_button]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.state {
            State::Reading => time::every(self.interval).map(|_| Message::Tick),
            State::Idle => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}
