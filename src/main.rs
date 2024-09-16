mod cursor;
mod gap_buffer;
mod ui;

fn main() {
    let gap_buff = gap_buffer::GapBuffer::new();
    let mut cursor = cursor::Cursor::new();

    ui::run(&mut text_buffer, &mut cursor);
}
