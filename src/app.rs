use anyhow::{Context, Result};
use chrono::Local;
use std::{
    process::Command,
    str,
    time::{Duration, Instant},
};
use sysinfo::System;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    /// Current RAM usage as a percentage
    pub ram_usage: f32,

    /// ID of the active workspace
    pub active_workspace_id: Option<u32>,

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
            active_workspace_id: None,
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
            self.active_workspace_id = Self::fetch_active_workspace_id().ok();
        }
    }

    /// Fetches the current CPU usage as a percentage
    fn fetch_cpu_usage(system: &mut System) -> f32 {
        system.refresh_cpu_usage();
        system.global_cpu_usage()
    }

    /// Fetches the current RAM usage as a percentage
    fn fetch_ram_usage(system: &mut System) -> f32 {
        system.refresh_memory();
        let used_memory_kb = system.used_memory() as f32;
        let total_memory_kb = system.total_memory() as f32;

        // Calculate the percentage
        (used_memory_kb / total_memory_kb) * 100.0
    }

    /// Fetches the ID of the active workspace
    fn fetch_active_workspace_id() -> Result<u32> {
        let output = Command::new("hyprctl")
            .arg("activeworkspace")
            .output()
            .context("Failed to execute hyprctl")?;

        let output_str =
            str::from_utf8(&output.stdout).context("Failed to parse hyprctl output as UTF-8")?;

        // Parse the output to extract the workspace ID
        // Assuming the output contains a line like: "workspace ID 2 (2) on monitor HDMI-A-1:"
        if let Some(line) = output_str.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(id) = parts[2].parse::<u32>() {
                    return Ok(id);
                }
            }
        }
        Err(anyhow::anyhow!("Failed to parse active workspace ID"))
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
