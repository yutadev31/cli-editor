pub mod buf;

use std::{fs::read_to_string, io::Write, path::PathBuf};

use anyhow::Result;
use termion::{
    clear, cursor,
    event::{Event, Key},
};
use utils::{cli::terminal_size, vec::Vec2};

use crate::buf::CodeBuffer;

pub struct Editor {
    buf: CodeBuffer,
    cursor: Vec2<usize>,
    offset: Vec2<usize>,
    mode: EditorMode,
}

pub enum EditorMode {
    Normal,
    Command,
    Insert,
    Visual,
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

        write!(
            stdout,
            "{}",
            cursor::Goto(
                (self.cursor.x + 1 - self.offset.x) as u16,
                (self.cursor.y + 1 - self.offset.y) as u16
            )
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn scroll_by(&mut self, y: isize) {
        let y = self.offset.y as isize + y;
        if y < 0 {
            return;
        }
        self.offset.y = y as usize;
    }

    pub fn on_event(&mut self, evt: Event) -> u8 {
        let (term_w, term_h) = terminal_size();

        match self.mode {
            EditorMode::Normal => match evt {
                Event::Key(Key::Char('i')) => {
                    self.mode = EditorMode::Insert;
                }
                Event::Key(Key::Char('q')) => return 1,
                Event::Key(Key::Char('h')) => {
                    if self.cursor.x > 0 {
                        self.cursor.x -= 1;
                    }
                }
                Event::Key(Key::Char('j')) => {
                    let len_count = self.buf.line_count();
                    if self.cursor.y < len_count {
                        self.cursor.y += 1;

                        if len_count <= self.cursor.y {
                            self.cursor.y -= 1;
                            return 0;
                        }

                        let line_len = self.buf.line_length(self.cursor.y);
                        if self.cursor.x > line_len {
                            self.cursor.x = line_len;
                        }

                        if self.offset.y + term_h <= self.cursor.y {
                            self.scroll_by(1);
                        }
                    }
                }
                Event::Key(Key::Char('k')) => {
                    if self.cursor.y > 0 {
                        self.cursor.y -= 1;

                        let line_len = self.buf.line_length(self.cursor.y);
                        if self.cursor.x > line_len {
                            self.cursor.x = line_len;
                        }

                        if self.offset.y > self.cursor.y {
                            self.scroll_by(-1);
                        }
                    }
                }
                Event::Key(Key::Char('l')) => {
                    if self.cursor.x < self.buf.line_length(self.cursor.y) {
                        self.cursor.x += 1;
                    }
                }
                _ => {}
            },
            EditorMode::Insert => match evt {
                Event::Key(Key::Char('\n')) => {
                    self.buf.split_line(self.cursor.x, self.cursor.y);
                    self.cursor.y += 1;
                    self.cursor.x = 0;
                }
                Event::Key(Key::Char('\t')) => {
                    self.buf.insert(' ', self.cursor.x, self.cursor.y);
                    self.buf.insert(' ', self.cursor.x, self.cursor.y);
                    self.cursor.x += 2;
                }
                Event::Key(Key::Char(c)) => {
                    self.buf.insert(c, self.cursor.x, self.cursor.y);
                    self.cursor.x += 1;
                }
                Event::Key(Key::Backspace) => {
                    if self.cursor.x > 0 {
                        self.buf.delete(self.cursor.x - 1, self.cursor.y);
                        self.cursor.x -= 1;
                    } else if self.cursor.y > 0 {
                        self.buf.join_lines(self.cursor.y - 1);
                        self.cursor.y -= 1;

                        let line_len = self.buf.line_length(self.cursor.y);
                        self.cursor.x = line_len;
                    }
                }
                Event::Key(Key::Delete) => {
                    let line_len = self.buf.line_length(self.cursor.y);
                    let len_count = self.buf.line_count();

                    if self.cursor.x < line_len {
                        self.buf.delete(self.cursor.x, self.cursor.y);
                    } else if self.cursor.y < len_count {
                        self.buf.join_lines(self.cursor.y);
                    }
                }
                Event::Key(Key::Ctrl('c')) => {
                    self.mode = EditorMode::Normal;
                }
                _ => {}
            },
            EditorMode::Visual => {}
            EditorMode::Command => {}
        }

        0
    }
}

impl From<String> for Editor {
    fn from(value: String) -> Self {
        Self {
            buf: CodeBuffer::new(value),
            mode: EditorMode::Normal,
            cursor: Vec2 { x: 0, y: 0 },
            offset: Vec2 { x: 0, y: 0 },
        }
    }
}
