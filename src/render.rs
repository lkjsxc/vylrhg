use crate::buffer::Buffer;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn slice_by_cells(s: &str, left_cells: usize, width_cells: usize) -> String {
    if width_cells == 0 {
        return String::new();
    }

    let mut out = String::new();
    let mut cell_pos = 0usize;

    for ch in s.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(0).max(1);
        if cell_pos + w <= left_cells {
            cell_pos += w;
            continue;
        }
        if cell_pos >= left_cells + width_cells {
            break;
        }
        out.push(ch);
        cell_pos += w;
        if cell_pos >= left_cells + width_cells {
            break;
        }
    }

    out
}

pub fn prefix_cells(s: &str, chars: usize) -> usize {
    let mut cells = 0usize;
    for (i, ch) in s.chars().enumerate() {
        if i >= chars {
            break;
        }
        cells += UnicodeWidthChar::width(ch).unwrap_or(0).max(1);
    }
    cells
}

pub fn status_line(buffer: &Buffer, mode: &str, msg: Option<&str>, width: usize) -> String {
    let name = buffer
        .file_path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "[No Name]".to_string());
    let dirty = if buffer.dirty { " [+]" } else { "" };
    let pos = format!("{}:{}", buffer.cursor.line + 1, buffer.cursor.col + 1);
    let left = format!(" {}  {}{} ", mode, name, dirty);
    let right = if let Some(m) = msg {
        format!(" {} ", m)
    } else {
        format!(" {} ", pos)
    };

    if width == 0 {
        return String::new();
    }

    let left_cells = left.width();
    let right_cells = right.width();
    if left_cells + right_cells <= width {
        let spaces = width - left_cells - right_cells;
        return format!("{}{}{}", left, " ".repeat(spaces), right);
    }

    if right_cells >= width {
        return slice_by_cells(&right, 0, width);
    }

    let left_allowed = width - right_cells;
    let left_trunc = slice_by_cells(&left, 0, left_allowed);
    let spaces = width.saturating_sub(left_trunc.width() + right_cells);
    format!("{}{}{}", left_trunc, " ".repeat(spaces), right)
}
