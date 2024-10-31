use chrono::{offset::LocalResult, Local};
use std::{error, time::Duration, time::Instant};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// The current time to display
    pub current_time: String,

    /// Last tick time to control updates
    last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_time: format!("[{}]", Local::now().format("%H:%M:%S %d/%m/%Y")),
            last_tick: Instant::now(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Update the time every second
        if self.last_tick.elapsed() >= Duration::from_secs(1) {
            self.current_time = format!("[{}]", Local::now().format("%H:%M:%S %d/%m/%Y"));
            self.last_tick = Instant::now();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
