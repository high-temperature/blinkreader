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
    IntervalChanged(f64),
    FileLoaded(Result<String,String>),
}