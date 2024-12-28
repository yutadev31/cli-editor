pub mod buf;

use std::{fs::read_to_string, io::Write, path::PathBuf};

use anyhow::Result;
use termion::{clear, cursor};
use utils::{cli::terminal_size, vec::Vec2};

use crate::buf::CodeBuffer;

pub struct Editor {
    buf: CodeBuffer,
    cursor: Vec2<usize>,
    offset: Vec2<usize>,
}

impl Editor {
    pub fn new() -> Self {
        Self::from(String::new())
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        let buf = read_to_string(path)?;
        Ok(Self::from(buf))
    }

    pub fn draw<T: Write>(&self, stdout: &mut T) {
        let (term_w, term_h) = terminal_size();

        let buf = self.buf.to_string();
        let lines = buf.lines();

        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        let lines: Vec<&str> = lines.skip(self.offset.y).take(term_h).collect();
        write!(stdout, "{}", lines.join("\r\n")).unwrap();
    }

    pub fn scroll_by(&mut self, y: isize) {
        let y = self.offset.y as isize + y;
        if y < 0 {
            return;
        }
        self.offset.y = y as usize;
    }
}

impl From<String> for Editor {
    fn from(value: String) -> Self {
        Self {
            buf: CodeBuffer::new(value),
            cursor: Vec2 { x: 0, y: 0 },
            offset: Vec2 { x: 0, y: 0 },
        }
    }
}
