use crate::gap_buffer::GapBuffer;
use crate::cursor::Cursor;
use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent},
    cursor as crossterm_cursor,
    terminal,
};

pub fn run(buffer: &mut GapBuffer, cursor: &mut Cursor) -> std::io::Result<()> {
    let mut stdout = stdout();

    stdout.execute(EnterAlternateScreen)?;    
    terminal::enable_raw_mode()?;

    loop {
        stdout.execute(Clear(ClearType::All))?;

        render_text(&mut stdout, buffer, cursor)?;

        if let Event::Key(KeyEvent { code, .. }) = read()? {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Left => cursor.move_left(buffer),
                KeyCode::Right => cursor.move_right(buffer),
                KeyCode::Up => cursor.move_up(buffer),
                KeyCode::Down => cursor.move_down(buffer),
                KeyCode::Char(c) => buffer.insert(c),
                KeyCode::Backspace => buffer.delete(),
                _ => {}
            }
        }

        stdout.flush()?;  // Ensure the terminal is updated
    }

    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn render_text<W: Write>(stdout: &mut W, buffer: &GapBuffer, cursor: &Cursor) -> std::io::Result<()> {
    // Write the text from the buffer to stdout
    for (i, &c) in buffer.text.iter().enumerate() {
        if i == buffer.gap_start {
            // If we're at the start of the gap, skip rendering the gap
            for _ in 0..(buffer.gap_end - buffer.gap_start) {
                write!(stdout, " ")?; // Render spaces for the gap
            }
            continue;
        }
        if c == '\n' {
            write!(stdout, "\n")?;
        } else {
            write!(stdout, "{}", c)?;
        }
    }

    // Move the cursor to its position
    stdout.execute(crossterm_cursor::MoveTo(cursor.column as u16, cursor.line as u16))?;
    stdout.flush()?;  // Ensure the rendering is flushed to the terminal
    Ok(())
}
