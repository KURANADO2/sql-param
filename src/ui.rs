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

    render_cursor(app, frame, area, area_enum);
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

fn render_cursor(app: &mut App, frame: &mut Frame, area: Rect, area_enum: AreaEnum) {
    // whether the cursor needs to be displayed
    if app.show_cursor(area_enum) {
        let cursor_pos = app.get_cursor_position();
        let content = app.content(area_enum);
        let cursor_coords = calculate_cursor_position(&content, cursor_pos, area);
        frame.set_cursor_position(cursor_coords);
    }
}

fn calculate_cursor_position(content: &str, cursor_pos: usize, area: Rect) -> (u16, u16) {
    // left border
    let mut x = area.x + 1;
    // right border
    let mut y = area.y + 1;
    // subtract the left and right borders
    let max_width = area.width.saturating_sub(2);

    let mut line_count = 0;

    for ch in content.chars() {
        if line_count >= cursor_pos {
            break;
        }

        if ch == '\n' {
            x = area.x + 1;
            y += 1;
            line_count += 1;
        } else {
            let char_width = if ch.len_utf8() > 1 { 2 } else { 1 };
            let current_line_width = x - (area.x + 1);
            if current_line_width + char_width > max_width {
                // Auto wrap
                x = area.x + 1;
                y += 1;
            }

            x += char_width;
            line_count += 1;
        }
    }

    // make sure the cursor is within the visible area
    if x >= area.x + area.width {
        x = area.x + area.width - 1;
    }
    if y >= area.y + area.height {
        y = area.y + area.height - 1;
    }

    // make sure the cursor does not go beyond the left boundary
    if x < area.x + 1 {
        x = area.x + 1;
    }

    (x, y)
}
