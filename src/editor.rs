use crate::buffer::Buffer;
use crate::commands::{parse_command, Command, CommandParseError};
use crate::fs_io;
use crate::render;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style,
    terminal,
};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
    Command,
}

struct Viewport {
    top_line: usize,
    left_cells: usize,
}

struct App {
    mode: Mode,
    buffer: Buffer,
    viewport: Viewport,
    status: Option<String>,
    cmdline: String,
    should_quit: bool,
    pending_g: bool,
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

pub fn run(start_path: Option<&Path>) -> Result<(), String> {
    let _guard = TerminalGuard::enter().map_err(|e| e.to_string())?;

    let (buffer, initial_status) = if let Some(path) = start_path {
        if path.exists() {
            match fs_io::read_text(path) {
                Ok(text) => (Buffer::from_text(Some(path.to_path_buf()), text), None),
                Err(err) => (
                    Buffer::new_empty(Some(path.to_path_buf())),
                    Some(format!("E: {err}")),
                ),
            }
        } else {
            (Buffer::new_empty(Some(path.to_path_buf())), Some("New file".to_string()))
        }
    } else {
        (Buffer::new_empty(None), None)
    };

    let mut app = App {
        mode: Mode::Normal,
        buffer,
        viewport: Viewport {
            top_line: 0,
            left_cells: 0,
        },
        status: initial_status,
        cmdline: String::new(),
        should_quit: false,
        pending_g: false,
    };

    loop {
        app.redraw().map_err(|e| e.to_string())?;
        if app.should_quit {
            break;
        }
        if event::poll(Duration::from_millis(250)).map_err(|e| e.to_string())? {
            match event::read().map_err(|e| e.to_string())? {
                Event::Key(key) => app.handle_key(key),
                Event::Resize(_, _) => {
                    // Redraw on next loop.
                }
                _ => {}
            }
        }
    }

    Ok(())
}

impl App {
    fn set_status(&mut self, msg: impl Into<String>) {
        self.status = Some(msg.into());
    }

    fn clear_status(&mut self) {
        self.status = None;
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
            Mode::Command => self.handle_key_command(key),
        }
        self.ensure_cursor_visible();
    }

    fn handle_key_normal(&mut self, key: KeyEvent) {
        self.clear_status();
        match key {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_quit = true;
            }

            KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.mode = Mode::Insert;
                self.pending_g = false;
            }
            KeyEvent {
                code: KeyCode::Char(':'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.mode = Mode::Command;
                self.cmdline.clear();
                self.pending_g = false;
            }

            KeyEvent {
                code: KeyCode::Char('g'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if self.pending_g {
                    self.buffer.move_top();
                    self.pending_g = false;
                } else {
                    self.pending_g = true;
                }
            }
            KeyEvent {
                code: KeyCode::Char('G'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.buffer.move_bottom();
                self.pending_g = false;
            }

            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Left,
                ..
            } => {
                self.buffer.move_left();
                self.pending_g = false;
            }
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Down,
                ..
            } => {
                self.buffer.move_down();
                self.pending_g = false;
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Up,
                ..
            } => {
                self.buffer.move_up();
                self.pending_g = false;
            }
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Right,
                ..
            } => {
                self.buffer.move_right();
                self.pending_g = false;
            }

            KeyEvent {
                code: KeyCode::Char('0'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.buffer.move_line_start();
                self.pending_g = false;
            }
            KeyEvent {
                code: KeyCode::Char('$'),
                modifiers: KeyModifiers::SHIFT,
                ..
            }
            | KeyEvent {
                code: KeyCode::End,
                ..
            } => {
                self.buffer.move_line_end();
                self.pending_g = false;
            }

            _ => {
                self.pending_g = false;
            }
        }
    }

    fn handle_key_insert(&mut self, key: KeyEvent) {
        self.pending_g = false;
        match key {
            KeyEvent {
                code: KeyCode::Esc,
                ..
            } => {
                self.mode = Mode::Normal;
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                self.buffer.insert_newline();
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.buffer.backspace();
            }
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => {
                self.buffer.delete();
            }
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => self.buffer.move_left(),
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => self.buffer.move_right(),
            KeyEvent {
                code: KeyCode::Up,
                ..
            } => self.buffer.move_up(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => self.buffer.move_down(),
            KeyEvent {
                code: KeyCode::Home,
                ..
            } => self.buffer.move_line_start(),
            KeyEvent {
                code: KeyCode::End,
                ..
            } => self.buffer.move_line_end(),
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => {
                self.buffer.insert_char(ch);
            }
            _ => {}
        }
    }

    fn handle_key_command(&mut self, key: KeyEvent) {
        self.pending_g = false;
        match key {
            KeyEvent {
                code: KeyCode::Esc,
                ..
            } => {
                self.mode = Mode::Normal;
                self.cmdline.clear();
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                let line = self.cmdline.clone();
                self.mode = Mode::Normal;
                self.cmdline.clear();
                self.exec_command(&line);
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.cmdline.pop();
            }
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => {
                self.cmdline.push(ch);
            }
            _ => {}
        }
    }

    fn exec_command(&mut self, line: &str) {
        match parse_command(line) {
            Ok(cmd) => self.apply_command(cmd),
            Err(CommandParseError::Empty) => {}
            Err(CommandParseError::MissingPath) => self.set_status("E: missing path"),
            Err(CommandParseError::Unknown) => self.set_status("E: unknown command"),
        }
    }

    fn apply_command(&mut self, cmd: Command) {
        match cmd {
            Command::Quit { force } => {
                if self.buffer.dirty && !force {
                    self.set_status("E: No write since last change (add ! to override)");
                } else {
                    self.should_quit = true;
                }
            }
            Command::Write { path } => {
                let target = match (path, self.buffer.file_path.clone()) {
                    (Some(p), _) => Some(p),
                    (None, Some(p)) => Some(p),
                    (None, None) => None,
                };
                let Some(path) = target else {
                    self.set_status("E: No file name");
                    return;
                };
                let contents = self.buffer.contents();
                match fs_io::write_text(&path, &contents) {
                    Ok(()) => {
                        self.buffer.file_path = Some(path);
                        self.buffer.mark_saved();
                        self.set_status("written");
                    }
                    Err(err) => self.set_status(format!("E: {err}")),
                }
            }
            Command::WriteQuit => {
                let path = self.buffer.file_path.clone();
                if path.is_none() {
                    self.set_status("E: No file name");
                    return;
                }
                let path = path.unwrap();
                let contents = self.buffer.contents();
                match fs_io::write_text(&path, &contents) {
                    Ok(()) => {
                        self.buffer.mark_saved();
                        self.should_quit = true;
                    }
                    Err(err) => self.set_status(format!("E: {err}")),
                }
            }
            Command::Edit { path, force } => {
                if self.buffer.dirty && !force {
                    self.set_status("E: Unsaved changes (use :e! to discard)");
                    return;
                }
                if path.exists() {
                    match fs_io::read_text(&path) {
                        Ok(text) => {
                            self.buffer.set_contents(text);
                            self.buffer.file_path = Some(path);
                            self.set_status("opened");
                        }
                        Err(err) => self.set_status(format!("E: {err}")),
                    }
                } else {
                    self.buffer.set_contents(String::new());
                    self.buffer.file_path = Some(path);
                    self.set_status("New file");
                }
            }
        }
    }

    fn ensure_cursor_visible(&mut self) {
        let (cols, rows) = match terminal::size() {
            Ok(s) => s,
            Err(_) => return,
        };
        if rows == 0 {
            return;
        }
        let content_height = rows.saturating_sub(1) as usize;
        if content_height == 0 {
            return;
        }

        // Vertical scrolling.
        if self.buffer.cursor.line < self.viewport.top_line {
            self.viewport.top_line = self.buffer.cursor.line;
        } else if self.buffer.cursor.line >= self.viewport.top_line + content_height {
            self.viewport.top_line = self.buffer.cursor.line + 1 - content_height;
        }

        // Horizontal scrolling (cells).
        let line = self.buffer.line_string(self.buffer.cursor.line);
        let cursor_cells = render::prefix_cells(&line, self.buffer.cursor.col);
        let width = cols as usize;
        if width > 0 {
            if cursor_cells < self.viewport.left_cells {
                self.viewport.left_cells = cursor_cells;
            } else if cursor_cells >= self.viewport.left_cells + width {
                self.viewport.left_cells = cursor_cells + 1 - width;
            }
        }
    }

    fn redraw(&mut self) -> std::io::Result<()> {
        let mut out = stdout();
        let (cols, rows) = terminal::size()?;
        let width = cols as usize;
        let height = rows as usize;

        execute!(out, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0))?;

        let content_height = height.saturating_sub(1);
        for row in 0..content_height {
            let line_idx = self.viewport.top_line + row;
            let line = self.buffer.line_string(line_idx);
            let visible = render::slice_by_cells(&line, self.viewport.left_cells, width);
            let mut s = visible;
            // Fill remainder so old content doesn't leak.
            let fill = width.saturating_sub(s.width());
            if fill > 0 {
                s.push_str(&" ".repeat(fill));
            }
            out.write_all(s.as_bytes())?;
            if row + 1 < content_height {
                out.write_all(b"\r\n")?;
            }
        }

        // Footer line.
        execute!(out, cursor::MoveTo(0, (height.saturating_sub(1)) as u16))?;
        match self.mode {
            Mode::Command => {
                let prompt = format!(":{}", self.cmdline);
                let mut s = prompt;
                let fill = width.saturating_sub(s.width());
                if fill > 0 {
                    s.push_str(&" ".repeat(fill));
                }
                execute!(out, style::Print(s))?;
            }
            Mode::Normal | Mode::Insert => {
                let mode = match self.mode {
                    Mode::Normal => "NORMAL",
                    Mode::Insert => "INSERT",
                    Mode::Command => "COMMAND",
                };
                let status = render::status_line(&self.buffer, mode, self.status.as_deref(), width);
                execute!(out, style::Print(status))?;
            }
        }

        // Cursor
        let cursor_row = self.buffer.cursor.line.saturating_sub(self.viewport.top_line);
        if cursor_row < content_height {
            let line = self.buffer.line_string(self.buffer.cursor.line);
            let cursor_cells = render::prefix_cells(&line, self.buffer.cursor.col);
            let x = cursor_cells.saturating_sub(self.viewport.left_cells);
            let x = x.min(width.saturating_sub(1));
            execute!(out, cursor::MoveTo(x as u16, cursor_row as u16))?;
        }

        out.flush()?;
        Ok(())
    }
}
