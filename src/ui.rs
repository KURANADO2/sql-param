use crate::app::{App, AreaEnum};
use lazy_static::lazy_static;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

lazy_static! {
    pub static ref FOCUSED_STYLE: Style = Style::default().fg(Color::Green).bold();
    pub static ref NORMAL_STYLE: Style = Style::default();
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Min(3),
        ])
        .split(frame.area());

    let input_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[0]);

    // recording area coordinate
    app.set_area_coordinate(AreaEnum::Sql, input_layout[0]);
    app.set_area_coordinate(AreaEnum::Value, input_layout[1]);
    app.set_area_coordinate(AreaEnum::Result, layout[1]);

    // render body
    // render sql
    render_body(app, frame, input_layout[0], AreaEnum::Sql);
    // render value
    render_body(app, frame, input_layout[1], AreaEnum::Value);
    // render result
    render_body(app, frame, layout[1], AreaEnum::Result);

    // render footer
    render_footer(frame, layout[2]);
}

fn render_body(app: &mut App, frame: &mut Frame, area: Rect, area_enum: AreaEnum) {
    let paragraph = Paragraph::new(app.content(area_enum))
        .wrap(Wrap { trim: true })
        .block(
            Block::new()
                .title(area_enum.title())
                .title_style(if app.current_area == area_enum {
                    *FOCUSED_STYLE
                } else {
                    *NORMAL_STYLE
                })
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Tab/Mouse: Switch | Ctrl+l: Clear | q: Exit").block(
            Block::new()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}
