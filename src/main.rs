pub(crate) use async_std::fs;

use iced::theme::Theme;
use iced::widget::{button, column, container, Text};
use iced::{Command, Element, executor, Settings, Alignment, Length, Application};



pub fn main() -> iced::Result {
    BlinkReader::run(Settings::default())
}

struct BlinkReader {
    display: String,
    count: i32,
    state: State,
    file_path:String,
}

enum State {
    Idle,
    Reading,
}

#[derive(Debug, Clone)]
enum Message {
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
        let p = "C:\\Users\\kyoch\\Rust\\blinktextreader\\src\\令和3章1節_生産性の動向と課題.txt".to_string();
            (
                BlinkReader {
                display:String::new(),
                count:0,
                state: State::Idle,
                file_path:p.clone(),
                },
                Command::perform(async move{
                    fs::read_to_string(p).await.map_err(|e| e.to_string())
                    },Message::FileLoaded
                )
            )
    }

    fn title(&self) -> String {
        String::from("Blink Reader")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => self.state = State::Reading,
                State::Reading { .. } => self.state = State::Idle,
            },
            Message::Reset => self.count = 0,
            Message::FileLoaded(Ok(content)) => self.display = content,
            Message::FileLoaded(Err(_)) => {
                self.display = format!("Failed to load file: {:?}", self.file_path.to_string());
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {

        let text = Text::new(&self.display).size(30);
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
