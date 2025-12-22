use ropey::Rope;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cursor {
    pub line: usize,
    pub col: usize, // char index within line (excluding newline)
}

#[derive(Debug)]
pub struct Buffer {
    text: Rope,
    pub cursor: Cursor,
    preferred_col: usize,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
}

impl Buffer {
    pub fn new_empty(file_path: Option<PathBuf>) -> Self {
        Self {
            text: Rope::from_str(""),
            cursor: Cursor::default(),
            preferred_col: 0,
            file_path,
            dirty: false,
        }
    }

    pub fn from_text(file_path: Option<PathBuf>, text: String) -> Self {
        let mut buffer = Self::new_empty(file_path);
        buffer.text = Rope::from_str(&text);
        buffer.cursor = Cursor::default();
        buffer.preferred_col = 0;
        buffer.dirty = false;
        buffer.clamp_cursor();
        buffer
    }

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn line_string(&self, line: usize) -> String {
        if line >= self.len_lines() {
            return String::new();
        }
        let slice = self.text.line(line);
        let mut s = slice.to_string();
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
        s
    }

    pub fn line_len_chars(&self, line: usize) -> usize {
        if line >= self.len_lines() {
            return 0;
        }
        let slice = self.text.line(line);
        let mut len = slice.len_chars();
        if slice.chars().last() == Some('\n') {
            len = len.saturating_sub(1);
        }
        len
    }

    pub fn set_preferred_col(&mut self) {
        self.preferred_col = self.cursor.col;
    }

    pub fn move_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.col = self.line_len_chars(self.cursor.line);
        }
        self.set_preferred_col();
    }

    pub fn move_right(&mut self) {
        let len = self.line_len_chars(self.cursor.line);
        if self.cursor.col < len {
            self.cursor.col += 1;
        } else if self.cursor.line + 1 < self.len_lines() {
            self.cursor.line += 1;
            self.cursor.col = 0;
        }
        self.set_preferred_col();
    }

    pub fn move_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let len = self.line_len_chars(self.cursor.line);
            self.cursor.col = self.preferred_col.min(len);
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.len_lines() {
            self.cursor.line += 1;
            let len = self.line_len_chars(self.cursor.line);
            self.cursor.col = self.preferred_col.min(len);
        }
    }

    pub fn move_line_start(&mut self) {
        self.cursor.col = 0;
        self.set_preferred_col();
    }

    pub fn move_line_end(&mut self) {
        self.cursor.col = self.line_len_chars(self.cursor.line);
        self.set_preferred_col();
    }

    pub fn move_top(&mut self) {
        self.cursor.line = 0;
        self.cursor.col = 0;
        self.set_preferred_col();
    }

    pub fn move_bottom(&mut self) {
        self.cursor.line = self.len_lines().saturating_sub(1);
        self.cursor.col = self.line_len_chars(self.cursor.line);
        self.set_preferred_col();
    }

    pub fn insert_char(&mut self, ch: char) {
        let idx = self.char_index();
        self.text.insert_char(idx, ch);
        self.cursor.col += 1;
        self.set_preferred_col();
        self.dirty = true;
        self.clamp_cursor();
    }

    pub fn insert_newline(&mut self) {
        let idx = self.char_index();
        self.text.insert_char(idx, '\n');
        self.cursor.line += 1;
        self.cursor.col = 0;
        self.set_preferred_col();
        self.dirty = true;
        self.clamp_cursor();
    }

    pub fn backspace(&mut self) {
        if self.cursor.col > 0 {
            let idx = self.char_index();
            self.text.remove(idx - 1..idx);
            self.cursor.col -= 1;
            self.set_preferred_col();
            self.dirty = true;
            self.clamp_cursor();
            return;
        }
        if self.cursor.line == 0 {
            return;
        }
        let start = self.text.line_to_char(self.cursor.line);
        if start == 0 {
            return;
        }
        self.text.remove(start - 1..start);
        self.cursor.line -= 1;
        self.cursor.col = self.line_len_chars(self.cursor.line);
        self.set_preferred_col();
        self.dirty = true;
        self.clamp_cursor();
    }

    pub fn delete(&mut self) {
        let len = self.line_len_chars(self.cursor.line);
        let idx = self.char_index();
        if self.cursor.col < len {
            self.text.remove(idx..idx + 1);
            self.dirty = true;
            self.clamp_cursor();
            return;
        }
        // At end-of-line: join with next line (remove newline) if exists.
        if self.cursor.line + 1 < self.len_lines() {
            self.text.remove(idx..idx + 1);
            self.dirty = true;
            self.clamp_cursor();
        }
    }

    pub fn contents(&self) -> String {
        self.text.to_string()
    }

    pub fn set_contents(&mut self, text: String) {
        self.text = Rope::from_str(&text);
        self.cursor = Cursor::default();
        self.preferred_col = 0;
        self.dirty = false;
        self.clamp_cursor();
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn clamp_cursor(&mut self) {
        let lines = self.len_lines().max(1);
        if self.cursor.line >= lines {
            self.cursor.line = lines - 1;
        }
        let len = self.line_len_chars(self.cursor.line);
        if self.cursor.col > len {
            self.cursor.col = len;
        }
        if self.preferred_col > len {
            self.preferred_col = len;
        }
    }

    fn char_index(&self) -> usize {
        let line_start = self.text.line_to_char(self.cursor.line);
        line_start + self.cursor.col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_backspace() {
        let mut b = Buffer::new_empty(None);
        b.insert_char('a');
        b.insert_char('b');
        assert_eq!(b.contents(), "ab");
        b.backspace();
        assert_eq!(b.contents(), "a");
    }

    #[test]
    fn newline_and_join() {
        let mut b = Buffer::new_empty(None);
        b.insert_char('a');
        b.insert_newline();
        b.insert_char('b');
        assert_eq!(b.contents(), "a\nb");
        b.cursor.line = 1;
        b.cursor.col = 0;
        b.backspace();
        assert_eq!(b.contents(), "ab");
    }
}
