use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::music::midi_to_staff_position;

const STAFF_AREA_ROWS: u16 = 27;
const CORE_STAFF_TOP: u16 = 6;
const CORE_STAFF_BOTTOM: u16 = 21;
const NOTE_COLUMN: u16 = 36;
const STAFF_START: usize = 12;
const STAFF_WIDTH_CHARS: usize = 62;

fn render_staff_area(frame: &mut Frame, area: Rect, app: &App) {
    let staff_y = area.y + area.height.saturating_sub(STAFF_AREA_ROWS) / 2;
    let staff_x = area.x + area.width.saturating_sub(STAFF_WIDTH_CHARS as u16) / 2;

    let sp = midi_to_staff_position(app.current_note.midi);
    let note_row = CORE_STAFF_BOTTOM as i16 - sp;

    for local_row in 0..STAFF_AREA_ROWS {
        let y = staff_y + local_row;
        if y >= area.y + area.height {
            break;
        }

        let area_row = local_row as i16;
        let mut chars = vec![' '; STAFF_WIDTH_CHARS];

        let is_core_staff = (CORE_STAFF_TOP as i16..=CORE_STAFF_BOTTOM as i16).contains(&area_row);
        let staff_line_idx = area_row - CORE_STAFF_TOP as i16;
        let is_line =
            is_core_staff && staff_line_idx >= 1 && staff_line_idx <= 9 && staff_line_idx % 2 == 1;

        if is_line {
            chars.fill('─');
        }

        if is_core_staff {
            let clef_str = match staff_line_idx + 2 {
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
            };
            for (i, c) in clef_str.chars().enumerate() {
                if i + 1 < STAFF_WIDTH_CHARS {
                    chars[i + 1] = c;
                }
            }
        }

        if area_row == note_row {
            chars[NOTE_COLUMN as usize] = '●';
        }

        let is_ledger_line = !is_core_staff && area_row == note_row && sp % 2 == 0;
        if is_ledger_line {
            let start = NOTE_COLUMN.saturating_sub(4) as usize;
            let end = (NOTE_COLUMN + 4).min(STAFF_WIDTH_CHARS as u16 - 1) as usize;
            chars[start..=end].fill('─');
            chars[NOTE_COLUMN as usize] = '●';
        }

        let line_str: String = chars.into_iter().collect();
        let note_style = Style::default().fg(Color::Yellow);
        let line_style = Style::default().fg(Color::DarkGray);
        let clef_style = Style::default().fg(Color::Cyan);
        let default_style = Style::default();

        let mut spans: Vec<Span> = Vec::new();
        let col_note = NOTE_COLUMN as usize;

        for (idx, ch) in line_str.char_indices() {
            let s = if idx == col_note && area_row == note_row {
                note_style
            } else if is_core_staff && idx < STAFF_START && ch != ' ' {
                clef_style
            } else if is_line && idx >= STAFF_START && ch == '─' {
                line_style
            } else {
                default_style
            };
            spans.push(Span::styled(ch.to_string(), s));
        }

        frame.render_widget(
            Paragraph::new(Line::from(spans)),
            Rect {
                x: staff_x,
                y,
                width: STAFF_WIDTH_CHARS as u16,
                height: 1,
            },
        );
    }
}

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(STAFF_AREA_ROWS),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Music Note Trainer  (q/Esc/Ctrl-C to quit)")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default());
    frame.render_widget(title, main[0]);

    render_staff_area(frame, main[1], app);

    let octave = (app.current_note.midi / 12) as i16 - 1;
    let info = Paragraph::new(format!(
        "Note on staff (octave {}) - MIDI: {}   |   Staff position: {}",
        octave,
        app.current_note.midi,
        midi_to_staff_position(app.current_note.midi),
    ))
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(info, main[2]);

    let input_widget = {
        let prompt = Span::styled(
            "Answer (Enter to confirm): ",
            Style::default().fg(Color::Gray),
        );
        let text = if app.user_input.is_empty() {
            Span::styled("_", Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(&app.user_input, Style::default().fg(Color::White))
        };
        Paragraph::new(Line::from(vec![prompt, text]))
            .block(Block::default().borders(Borders::ALL).title(" Input "))
    };
    frame.render_widget(input_widget, main[3]);

    let feedback = {
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
        Paragraph::new(Line::from(spans))
    };
    frame.render_widget(feedback, main[4]);
}
