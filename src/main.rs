use std::time::Duration;

use async_std::path::Path;
use iced::theme::Theme;
use iced::widget::{button, column, container, Text};
use iced::{executor, time, Alignment, Application, Command, Element, Font, Length, Settings, Subscription};


use async_std::{self, fs};
fn main() -> iced::Result {
    BlinkReader::run(Settings::default())
}
const GENEIKOBURIMIN: Font = Font::External {
    name: "GenEiKoburiMin",
    bytes: include_bytes!("font/GenEiKoburiMin6-R.ttf"),
};

const CHUNK: usize = 20;

struct BlinkReader {
    display: String,
    state: State,
    full_text:String,
    position: usize,
}

enum State {
    Idle,
    Reading,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Toggle,
    Reset,
    FileLoaded(Result<String, String>),
}

impl Application for BlinkReader {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (BlinkReader, Command<Message>) {
        let full_text_path = Path::new("C:\\Users\\kyoch\\Downloads\\R5_03_01_生産性の動向と課題 (1).txt");
        let initial_text = "Loading...".to_string();
            (
                BlinkReader {
                display:String::new(),
                state: State::Idle,
                full_text: initial_text,
                position:0,
                },
                Command::perform(
                    async move {
                        fs::read_to_string(&full_text_path).await.map_err(|e| e.to_string())
                    },
                    Message::FileLoaded // Adjusted to accept Result<String, String>
                )
            )
    }

    fn title(&self) -> String {
        String::from("Blink Reader")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message{
            Message::Tick =>{
                if self.position<self.full_text.len(){
                    let pr_chars:Vec<char> =self.full_text.chars().collect();
                    let end = (self.position + CHUNK).min(self.full_text.len());
                    let sub_str = &pr_chars[self.position..end];
                    self.display = sub_str.into_iter().collect();
                    self.position += CHUNK;
                }
                Command::none()
            },
            Message::Reset =>{
                self.position = 0;
                Command::none()
            },
            Message::Toggle => {
                self.state = match self.state {
                    State::Idle => State::Reading,
                    State::Reading => State::Idle,
                };
                Command::none()
            },
            Message::FileLoaded(Ok(content)) => {
            self.full_text =  "Loading Successful!".to_string();
            self.full_text = content;
            self.position = 0;
            Command::none()
            },
            Message::FileLoaded(Err(e))=>{
                self.full_text = e.to_string();
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
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
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