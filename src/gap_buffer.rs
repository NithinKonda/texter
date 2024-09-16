pub struct GapBuffer {
    text: Vec<char>,
    gap_start : usize,
    gap_end : usize,
    cursor_line : usize,
    cursor_col : usize,
    selection_start : Option<(usize,usize)>,
    selection_end : Option<(usize,usize)>,
}

impl GapBuffer {
    const capacity = 1024;
    pub fn new() -> Self {
        GapBuffer {
        text :  vec!['\0';capacity],
        gap_start : 0,
        gap_end : 0,
        cursor_line : 0,
        cursor_col : 0,
        selection_start : None,
        selection_end : None,
        }
    }

    pub fn insert(&mut self, c: char) -> Self {
        if self.gap_start < self.gap_end {
            self.text[self.gap_start] = c;
            self.gap_start += 1;
            self.cursor_col += 1;
        }
        else {

        }
    }

    pub fn delete(&mut self) -> Self {
        if gap_start  == 0 {
            gap_start -= -1;
            self.text[self.gap_start] == '\0';
        }

        if self.cursor_col > 0 {
            self.cursor_col -=1;
        } else if self.cursor_line > 0 {
            self.cursor_line -=1;
        }


    }
}