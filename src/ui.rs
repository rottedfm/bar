use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(48),
            Constraint::Percentage(4),
            Constraint::Percentage(48),
        ])
        .split(frame.area());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10), // NixOS logo
            Constraint::Percentage(30), // Workspace tracker
            Constraint::Percentage(60), // Time, CPU, and RAM
        ])
        .split(vertical_chunks[1]);

    let logo = "ʕ•ᴥ•ʔ";
    frame.render_widget(
        Paragraph::new(logo).alignment(Alignment::Left),
        horizontal_chunks[0],
    );

    // Centered Workspace Tracker
    let icons = vec!['●', '○', '○', '○', '○', '○', '○', '○', '○', '○'];
    let mut workspace_text = String::new();

    for i in 0..10 {
        if Some(i) == app.active_workspace_id {
            workspace_text.push(icons[i as usize]);
        } else {
            workspace_text.push('●');
        }
        workspace_text.push(' ');
    }
    frame.render_widget(
        Paragraph::new(workspace_text).alignment(Alignment::Center),
        horizontal_chunks[1],
    );

    // Right-aligned Time, CPU, and RAM usage
    let time_cpu_ram_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ])
        .split(horizontal_chunks[2]);

    let current_time_text = format!("{}", app.current_time);
    frame.render_widget(
        Paragraph::new(current_time_text).alignment(Alignment::Right),
        time_cpu_ram_chunks[2],
    );

    let cpu_usage_text = format!("CPU Usage: {:.2}%", app.cpu_usage);
    frame.render_widget(
        Paragraph::new(cpu_usage_text).alignment(Alignment::Center),
        time_cpu_ram_chunks[0],
    );

    // RAM usage widget
    let ram_usage_text = format!("RAM Usage: {:.2}%", app.ram_usage);
    frame.render_widget(
        Paragraph::new(ram_usage_text).alignment(Alignment::Center),
        time_cpu_ram_chunks[1],
    );
}
