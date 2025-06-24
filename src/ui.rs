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

    // 显示光标
    if app.current_area == area_enum && (area_enum == AreaEnum::Sql || area_enum == AreaEnum::Value)
    {
        let cursor_pos = app.cursor_position(area_enum);
        let content = app.content(area_enum);

        // 计算光标在文本中的位置
        let mut x = area.x + 1; // 左边框
        let mut y = area.y + 1; // 上边框

        for (i, ch) in content.chars().enumerate() {
            if i == cursor_pos {
                break;
            }

            if ch == '\n' {
                x = area.x + 1;
                y += 1;
            } else {
                x += 1;
                // 处理宽字符（如中文）
                if ch.len_utf8() > 1 {
                    x += 1;
                }
            }
        }

        // 确保光标在可视区域内
        if x >= area.x + area.width {
            x = area.x + area.width - 1;
        }
        if y >= area.y + area.height {
            y = area.y + area.height - 1;
        }

        frame.set_cursor_position((x, y));
    }
}

fn render_footer(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Tab/Mouse: Switch | Ctrl + l: Clear | q: Exit").block(
            Block::new()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        area,
    );
}
