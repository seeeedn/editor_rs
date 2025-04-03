use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{layout::Position, style::{Style, Stylize}, text::Text, DefaultTerminal};

mod gapbuffer;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut style = Style::new().white();
    let mut gb = gapbuffer::GapBuffer::init_buffer();
    let _ = gb.read_from_file("src/main.rs");

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let text = Text::raw(gb.data_to_string()).style(style);
            f.set_cursor_position(Position::new(0, 0));
            f.render_widget(text, size);
        })?;

        if let Event::Key(key ) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        //let _ = gb.write_to_file("src/main.rs");
                        break;
                    }
                    KeyCode::Enter => gb.insert_char('\n'),
                    KeyCode::Backspace => gb.delete_char(),
                    KeyCode::Left => gb.move_left(1),
                    KeyCode::Char(ch) => gb.insert_char(ch),
                    _ => style = style.green(),
                }
            }
        }
    }
    Ok(())
}