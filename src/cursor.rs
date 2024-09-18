use crate::gap_buffer::GapBuffer;


pub struct Cursor {
    pub line : usize,
    pub column : usize,
}


impl Cursor {
    pub fn new() -> Self {
        Cursor { line : 0, column : 0 }
    }

    pub fn move_left( &mut self, buffer: &GapBuffer ) {
        if self.column  > 0 {
            self.column -=1;
        } else if self.line > 0 {
            self.line -= 1;
            self.column = buffer.get_line_length(self.line);
        }
    }

    pub fn move_right( &mut self, buffer: &GapBuffer ) {
        let line_length = buffer.get_line_length(self.line);
        if self.column < line_length {
            self.column += 1;
        } else if self.line < buffer.get_total_lines() -1 {
            self.line += 1;
            self.column = 0;
        }
    }

    pub fn move_up(&mut self, buffer: &GapBuffer) {
        if self.line > 0 {
            self.line -= 1;
            let line_length = buffer.get_line_length(self.line);
            self.column = std::cmp::min(self.column, line_length);
    }
}

    pub fn move_down(&mut self, buffer: &GapBuffer) {
        if self.line < buffer.get_total_lines() - 1 {
            self.line += 1;
            let line_length = buffer.get_line_length(self.line);
            // Ensure the cursor stays within the length of the line
            self.column = std::cmp::min(self.column, line_length);
        }
        
    }
}