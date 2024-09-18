mod cursor;
mod gap_buffer;
mod ui;

fn main() {
    let mut gap_buff = gap_buffer::GapBuffer::new();
    let mut cursor = cursor::Cursor::new();

    if let Err(e) = ui::run(&mut gap_buff, &mut cursor) {
        eprintln!("Error: {}", e);
    }
}
