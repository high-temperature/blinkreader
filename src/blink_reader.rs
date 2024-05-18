use std::time::Duration;
use std::vec;

use async_std::path::Path;
use iced::theme::Theme;
use iced::widget::{button, column, container, Text};
use iced::{executor, time, Alignment, Application, Command, Element, Font, Length, Subscription};
use async_std::{self, fs};
use async_std::io::BufReader;
use async_std::prelude::*;
use iced::widget::Slider;

use crate::message::{Message, State};

const GENEIKOBURIMIN: Font = Font::External {
    name: "GenEiKoburiMin",
    bytes: include_bytes!("font/GenEiKoburiMin6-R.ttf"),
};

fn split_into_chunks(text: String, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let text_len = text.len();

    while start < text_len {
        let mut end = start;
        let mut char_count = 0;

        for (idx, c) in text[start..].char_indices() {
            if c == '\n' {
                end = start + idx + 1;
                break;
            }
            if char_count >= chunk_size {
                end = start + idx;
                break;
            }
            char_count += 1;
            end = start + idx + 1;
        }

        if end > text_len {
            end = text_len;
        }

        let chunk = text[start..end].to_string();
        chunks.push(chunk);
        start = end;
    }

    chunks
}


const CHUNK: usize = 20;
const MIN_INTERVAL:f64 = 0.1;
const MAX_INTERVAL:f64 = 5.0;

pub struct BlinkReader {
    display: Vec<String>,
    state: State,
    full_text: Vec<String>,
    position: usize,
    interval:Duration,
    slider_value: f64,
}

impl BlinkReader{
    fn reset(&mut self){
        self.position = 0;
        self.display = vec!["".to_string(),"".to_string(),"".to_string(),"".to_string()];
    }
}

impl Application for BlinkReader {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (BlinkReader, Command<Message>) {
        let full_text_path = Path::new("D:\\Document\\Rust\\blinktextreader\\src\\text\\第1節　実体経済の動向.txt");
        let initial_text = vec!["Loading...".to_string();10];

        (
            BlinkReader {
                display: initial_text.clone(),
                state: State::Idle,
                full_text: initial_text.clone(),
                position: 0,
                interval: Duration::from_secs(1),
                slider_value:1.0,
            },
            Command::perform(
                async move {
                    
                    let file = fs::File::open(&full_text_path).await.map_err(|e| e.to_string())?;
                    let mut reader = BufReader::new(file);
                    let mut content = String::new();
                    reader.read_to_string(&mut content).await.map_err(|e| e.to_string())?;
                    let lines = split_into_chunks(content, CHUNK);
                    Ok(lines)
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
                
                    self.display[0] = self.display[1].clone();
                    self.display[1] = self.display[2].clone();
                    self.display[2] = self.display[3].clone();
                    self.display[3] = self.full_text[self.position].clone();
                    self.position += 1 ;
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
                self.full_text = vec![format!("Error loading file: {}", err)];
                self.reset();
                Command::none()
            },
            Message::IntervalChanged(new_interval)=>{
                self.interval = Duration::from_secs_f64(new_interval);
                self.slider_value = new_interval;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let text0 = Text::new(&self.display[0]).size(30).font(GENEIKOBURIMIN);
        let text1 = Text::new(&self.display[1]).size(30).font(GENEIKOBURIMIN);
        let text2 = Text::new(&self.display[2]).size(30).font(GENEIKOBURIMIN);
        let text3 = Text::new(&self.display[3]).size(30).font(GENEIKOBURIMIN);
        let toggle_button = button(Text::new("Toggle")).on_press(Message::Toggle);
        let reset_button = button(Text::new("Reset")).on_press(Message::Reset);

        let slider = Slider::new(
            MIN_INTERVAL..=MAX_INTERVAL,
            self.slider_value,
            Message::IntervalChanged,
        );

        let content = column![text0, text1, text2, text3, toggle_button, reset_button,slider]
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
