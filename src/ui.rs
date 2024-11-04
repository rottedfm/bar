use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Paragraph},
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

    // Split the middle chunk horizontally to display both widgets side by side
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(vertical_chunks[1]);

    // Time display widget in the left half of the middle chunk
    frame.render_widget(
        Paragraph::new(app.current_time.clone()).alignment(Alignment::Center),
        horizontal_chunks[2],
    );

    // CPU usage widget in the right half of the middle chunk
    let cpu_usage_text = format!(" {:.2}%", app.cpu_usage);
    frame.render_widget(
        Paragraph::new(cpu_usage_text).alignment(Alignment::Center),
        horizontal_chunks[0],
    );

    // RAM usage widget
    let ram_usage_text = format!("RAM Usage: {:.2}%", app.ram_usage);
    frame.render_widget(
        Paragraph::new(ram_usage_text).alignment(Alignment::Center),
        horizontal_chunks[1],
    );
}
