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
    pub fn new() -> Self {

    }

    pub fn insert(&mut self, c: char) -> Self {
        
    }

    pub fn delete(&mut self) -> Self {
        
    }
}