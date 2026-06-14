use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

const STAFF_AREA_ROWS: u16 = 27;
const CORE_STAFF_TOP: u16 = 6;
const CORE_STAFF_BOTTOM: u16 = 21;
const NOTE_COLUMN: u16 = 36;
const STAFF_START: usize = 12;
const STAFF_WIDTH_CHARS: usize = 62;

fn clef_for_row(idx: i16) -> &'static str {
    match idx {
        0 => "      ⣀⣀   ",
        1 => "    ⢠⣾⠿⣿⣇  ",
        2 => "    ⣾⠇⠀⢸⣿  ",
        3 => "    ⡿⠀⣠⣿⡏  ",
        4 => "    ⣠⣾⡿⠋   ",
        5 => "  ⣠⣾⡿⠋⠀⠀⠀  ",
        6 => " ⣴⡿⠋⢠⡆     ",
        7 => "⣼⡿⠁⢠⣾⣿⠿⠿⣦⡀ ",
        8 => "⣿⠁⠀⣿⠁⢿⡄⠀⠘⣷ ",
        9 => "⢹⣧⡀⠙⠆⠸⣇⠀⣴⡟ ",
        10 => " ⠙⠿⣶⣤⠀⣿⠀⠋ ",
        11 => "⠀⠀⠀⠀⠀⢸⡇   ",
        12 => "⢠⣶⣿⣷⣦⢸⣧   ",
        13 => "⠸⣿⣿⣿⣿⣿⠏   ",
        14 => "⠀⠈⠉⠉⠉     ",
        _ => "",
    }
}

fn is_ledger_row(area_row: i16, note_row: i16, sp: i16) -> bool {
    area_row % 2 == 1 && {
        if sp > 8 {
            let first = if sp % 2 == 0 { note_row } else { note_row - 1 };
            first <= 7 && area_row >= first && area_row <= 7
        } else if sp < 0 {
            let last = if sp % 2 == 0 { note_row } else { note_row + 1 };
            last >= 19 && area_row >= 19 && area_row <= last
        } else {
            false
        }
    }
}

struct RowContext {
    is_core_staff: bool,
    is_line: bool,
    is_ledger: bool,
    note_offset: i16,
}

fn build_row_chars(area_row: i16, ctx: &RowContext) -> Vec<char> {
    let mut chars = vec![' '; STAFF_WIDTH_CHARS];

    if ctx.is_line {
        chars.fill('─');
    }

    if ctx.is_core_staff {
        let staff_line_idx = area_row - CORE_STAFF_TOP as i16;
        let clef_str = clef_for_row(staff_line_idx);
        for (i, c) in clef_str.chars().enumerate() {
            if i + 1 < STAFF_WIDTH_CHARS {
                chars[i + 1] = c;
            }
        }
    }

    if ctx.is_ledger {
        let start = NOTE_COLUMN.saturating_sub(4) as usize;
        let end = (NOTE_COLUMN + 4).min(STAFF_WIDTH_CHARS as u16 - 1) as usize;
        chars[start..=end].fill('─');
    }

    if (-1..=1).contains(&ctx.note_offset) {
        let notehead_str = match ctx.note_offset {
            0 => "⢾⡷",
            _ => "",
        };
        let start = NOTE_COLUMN as usize - 1;
        for (i, c) in notehead_str.chars().enumerate() {
            if start + i < STAFF_WIDTH_CHARS {
                chars[start + i] = c;
            }
        }
    }

    chars
}

fn style_row_spans<'a>(
    line_str: &'a str,
    note_offset: i16,
    ctx: &'a RowContext,
) -> Vec<Span<'a>> {
    let note_style = Style::default().fg(Color::DarkGray);
    let line_style = Style::default().fg(Color::DarkGray);
    let clef_style = Style::default();
    let default_style = Style::default();

    let mut spans: Vec<Span> = Vec::new();

    for (idx, ch) in line_str.char_indices() {
        let is_notehead = (-1..=1).contains(&note_offset)
            && idx >= NOTE_COLUMN as usize - 1
            && idx <= NOTE_COLUMN as usize;
        let s = if is_notehead {
            note_style
        } else if ctx.is_core_staff && idx < STAFF_START && ch != ' ' {
            clef_style
        } else if (ctx.is_line || ctx.is_ledger) && idx >= STAFF_START && ch == '─' {
            line_style
        } else {
            default_style
        };
        spans.push(Span::styled(ch.to_string(), s));
    }

    spans
}

fn render_staff_area(frame: &mut Frame, area: Rect, app: &App) {
    let staff_y = area.y + area.height.saturating_sub(STAFF_AREA_ROWS) / 2;
    let staff_x = area.x + area.width.saturating_sub(STAFF_WIDTH_CHARS as u16) / 2;

    let sp = app.current_note.staff_position;
    let note_row = CORE_STAFF_BOTTOM as i16 - sp - 4;

    for local_row in 0..STAFF_AREA_ROWS {
        let y = staff_y + local_row;
        if y >= area.y + area.height {
            break;
        }

        let area_row = local_row as i16;
        let is_core_staff =
            (CORE_STAFF_TOP as i16..=CORE_STAFF_BOTTOM as i16).contains(&area_row);
        let staff_line_idx = area_row - CORE_STAFF_TOP as i16;
        let is_line =
            is_core_staff && (3..=11).contains(&staff_line_idx) && staff_line_idx % 2 == 1;
        let is_ledger = is_ledger_row(area_row, note_row, sp);
        let note_offset = area_row - note_row;

        let ctx = RowContext { is_core_staff, is_line, is_ledger, note_offset };
        let chars = build_row_chars(area_row, &ctx);
        let line_str: String = chars.into_iter().collect();
        let spans = style_row_spans(&line_str, note_offset, &ctx);

        frame.render_widget(
            Paragraph::new(Line::from(spans)),
            Rect { x: staff_x, y, width: STAFF_WIDTH_CHARS as u16, height: 1 },
        );
    }
}

fn make_layout(area: Rect) -> [Rect; 4] {
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);
    [main[0], main[1], main[2], main[3]]
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Music Note Trainer  (q/Esc/Ctrl-C to quit)")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default());
    frame.render_widget(title, area);
}

fn render_input(frame: &mut Frame, area: Rect) {
    let widget = Paragraph::new("Press A-G to answer")
        .block(Block::default().borders(Borders::ALL).title(" Input "))
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(widget, area);
}

fn render_feedback(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![Span::styled(
        format!("Score: {}  |  Streak: {}", app.score, app.streak),
        Style::default().fg(Color::White),
    )];
    if !app.feedback_text.is_empty() {
        let fb_style = if app.feedback_text.starts_with("Correct") {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };
        spans.push(Span::raw("  |  "));
        spans.push(Span::styled(&app.feedback_text, fb_style));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

pub fn render(frame: &mut Frame, app: &App) {
    let [title_area, staff_area, input_area, feedback_area] = make_layout(frame.area());
    render_title(frame, title_area);
    render_staff_area(frame, staff_area, app);
    render_input(frame, input_area);
    render_feedback(frame, feedback_area, app);
}
