pub struct Cursor {
    pub line,
    pub column,
}


impl Cursor {
    pub fn new() -> Self {
        Cursor { line : 0, column : 0 }
    }

    pub fn move_up(&mut self, buffer: &Buffer) {

    }

    pub fn move_down(&mut self, buffer: &Buffer) {
        
    }
}