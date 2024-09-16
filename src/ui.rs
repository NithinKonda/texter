use crate::gap_buffer::GapBuffer;
use crate::cursor::Cursor;
use crossterm : {

    ExecutableCommand, terminal ::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    event :: {read, Event, KeyCode, KeyEvent},
    cursor, terminal, Result,   

};

pub fn run(buffer: &mut GapBuffer, cursor: &mut Cursor) {
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen);
    terminal :: enable_raw_mode();

    loop {
        stdout.execute(Clear(ClearType::All))?;

        render_text(&mut stdout, buffer, cursor)?;

        if let Event::Key(KeyEvent {code, ..}) = read()? {
            match code {
                KeyCode::Char('q'),
                KeyCode::Left => cursor.move_left(buffer),
                KeyCode::Right => cursor.move_right(buffer),
                KeyCode::Up => cursor.up(buffer),
                KeyCode::Down => cursor.down(buffer),
                KeyCode::Char(c) => buffer.insert(c),
                KeyCode::Backspace => buffer.delete(),
                _ => {}
            }
        }

        stdout.flush()?;
    }

    stdout.execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    Ok(())


}



pub fn render_text<W : Write>(stdout: &mut W , buffer :  &GapBuffer, cursor : &Cursor) -> Result<()> {
    let mut line = 0;
    let mut col =0;

    for (i ,&c ) in buffer.text.item().enumerate() {
        if i == buffer.gap_start {
            col += buffer.gap_start;
                continue;
            }


            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                print!("{}",c);
                col += 1;
            }
        }
        stdout.execute(cursor::MoveTo(cursor.cursor_col as u16, cursor.cursor_line as u16))?;

        Ok(())

}


