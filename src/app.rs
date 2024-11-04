use chrono::Local;
use std::{
    error,
    time::{Duration, Instant},
};
use sysinfo::System;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// The current time to display
    pub current_time: String,

    /// Last tick time to control update
    last_tick: Instant,

    /// Current CPU usage as a percentage
    pub cpu_usage: f32,

    /// Current RAM usage in MB
    pub ram_usage: f32,

    system: System,
}

impl Default for App {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            running: true,
            current_time: format!("{}", Local::now().format("%H:%M:%S %d/%m/%Y")),
            last_tick: Instant::now(),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            system,
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
            self.current_time = format!("{}", Local::now().format("%H:%M:%S %d/%m/%Y"));
            self.last_tick = Instant::now();
            self.cpu_usage = Self::fetch_cpu_usage(&mut self.system);
            self.ram_usage = Self::fetch_ram_usage(&mut self.system);
        }
    }

    /// Fetches the current CPU usage as a percentage
    fn fetch_cpu_usage(system: &mut System) -> f32 {
        system.refresh_cpu_usage();
        system.global_cpu_usage()
    }

    /// Fetches the current RAM usage in MB
    fn fetch_ram_usage(system: &mut System) -> f32 {
        system.refresh_memory();
        let used_memory_kb = system.used_memory() as f32;
        let total_memory_kb = system.total_memory() as f32;

        // Calculate the percentage
        (used_memory_kb / total_memory_kb) * 100.0
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
