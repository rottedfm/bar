use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(48),
            Constraint::Percentage(1),
            Constraint::Percentage(48),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(app.current_time.clone()).alignment(Alignment::Center),
        chunks[1],
    );
}
