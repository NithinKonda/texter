pub struct GapBuffer {
    pub text: Vec<char>,
    pub gap_start : usize,
    pub gap_end : usize,
    pub cursor_line : usize,
    pub cursor_col : usize,
    pub selection_start : Option<(usize,usize)>,
    pub selection_end : Option<(usize,usize)>,
}

impl GapBuffer {
    const capacity : usize = 1024;
    pub fn new() -> Self {
        GapBuffer {
        text :  vec!['\0';GapBuffer::capacity],
        gap_start : 0,
        gap_end : 0,
        cursor_line : 0,
        cursor_col : 0,
        selection_start : None,
        selection_end : None,
        }
    }

    pub fn insert(&mut self, c: char) {
        if self.gap_start < self.gap_end {
            self.text[self.gap_start] = c;
            self.gap_start += 1;
            self.cursor_col += 1;
        } else {
            println!("Gap buffer is full");
        }
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.text[self.gap_start] = '\0';
            if self.cursor_col > 0 {
                self.cursor_col -= 1;
            } else if self.cursor_line > 0 {
                self.cursor_line -= 1;
                self.cursor_col = self.get_line_length(self.cursor_line);
            }
        }
    }

    pub fn get_line_length(&self, line: usize) -> usize {
        let mut col = 0;
        let mut current_line = 0;
        
        for (i, &ch) in self.text.iter().enumerate() {
            if i >= self.gap_start && i < self.gap_end {
                continue;
            }
            if ch == '\n' {
                current_line += 1;
            }

            if current_line == line {
                if ch == '\n' {
                    break;
                }
                col += 1;
            } else if current_line > line {
                break;
            }
        }

        col
    }




    pub fn get_total_lines(&self) -> usize {
        let mut lines = 1;
        for (i,&ch) in self.text.iter().enumerate() {
            if i >= self.gap_start && i < self.gap_end {
                continue;
            }

            if ch == '\n' {
                lines += 1;
            }
        }
        lines
    }


}


