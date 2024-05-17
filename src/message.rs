#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Reading,
}


#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Toggle,
    Reset,
    FileLoaded(Result<String,String>),
}