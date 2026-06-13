mod app;
mod music;
mod ui;

use std::io;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;
        app.tick();

        if app.quit {
            break;
        }

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                app.quit = true;
                continue;
            }
            match key.code {
                KeyCode::Char('q') => app.quit = true,
                KeyCode::Esc => app.quit = true,
                KeyCode::Enter => app.submit(),
                KeyCode::Char(c) => match c {
                    'a' | 'c' | 'd' | 'e' | 'f' | 'g' => {
                        app.type_char(c.to_ascii_uppercase());
                    }
                    'A'..='G' => app.type_char(c),
                    'b' => app.type_char('b'),
                    '#' => app.type_char('#'),
                    _ => {}
                },
                KeyCode::Backspace => app.backspace(),
                _ => {}
            }
        }
    }

    Ok(())
}

