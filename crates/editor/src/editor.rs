mod buf;
mod mode;

use std::{
    fs::{read_to_string, write},
    io::Write,
    path::PathBuf,
};

use mode::EditorMode;
use termion::{
    clear, color, cursor,
    event::{Event, Key},
};
use utils::{cli::terminal_size, types::Direction, types::Vec2};

use crate::buf::CodeBuffer;

pub struct Editor {
    buf: CodeBuffer,
    mode: EditorMode,
    cursor: Vec2<usize>,
    offset: Vec2<usize>,
}

impl Editor {
    pub fn new() -> Self {
        Self::from(String::new())
    }

    pub fn open(path: PathBuf) -> Self {
        let mut buf = match read_to_string(path) {
            Ok(buf) => buf,
            Err(_) => String::new(),
        };

        if buf.is_empty() {
            buf = String::from("\n");
        }

        Self::from(buf)
    }

    pub fn draw<T: Write>(&self, stdout: &mut T) {
        let (term_w, term_h) = terminal_size();

        let buf = self.buf.to_string();
        let lines = buf.lines();

        write!(stdout, "{}", clear::All).unwrap();

        // 行番号を表示
        let len_count = self.buf.line_count();
        let line_num_w = len_count.to_string().len();
        let line_numbers: Vec<String> = (1..=len_count)
            .skip(self.offset.y)
            .take(term_h - 1)
            .map(|x| x.to_string())
            .collect();
        write!(stdout, "{}", cursor::Goto(1, 2)).unwrap();
        write!(stdout, "{}", line_numbers.join("\r\n")).unwrap();

        // コードを表示
        lines
            .skip(self.offset.y)
            .take(term_h - 1)
            .enumerate()
            .for_each(|(index, line)| {
                write!(
                    stdout,
                    "{}",
                    cursor::Goto(2 + line_num_w as u16, 2 + index as u16)
                )
                .unwrap();
                write!(stdout, "{}", line).unwrap();
            });

        // 情報バーを表示
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}", color::Bg(color::White)).unwrap();
        write!(stdout, "{}", color::Fg(color::Black)).unwrap();
        write!(stdout, "{}", " ".repeat(term_w as usize)).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(
            stdout,
            " {} {},{}",
            self.mode.to_string(),
            self.cursor.x,
            self.cursor.y,
        )
        .unwrap();
        write!(stdout, "{}", color::Bg(color::Reset)).unwrap();
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();

        write!(
            stdout,
            "{}",
            cursor::Goto(
                (self.cursor.x + 2 - self.offset.x + line_num_w) as u16,
                (self.cursor.y + 2 - self.offset.y) as u16
            )
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    pub fn scroll_by(&mut self, y: isize) {
        self.offset.y = (self.offset.y as isize + y) as usize;
    }

    pub fn cursor_by(&mut self, dir: Direction) {
        let (_, term_h) = terminal_size();
        let len_count = self.buf.line_count();

        match dir {
            Direction::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                }
            }
            Direction::Down => {
                if self.cursor.y < len_count {
                    self.cursor.y += 1;

                    if len_count <= self.cursor.y {
                        self.cursor.y -= 1;
                        return;
                    }

                    let line_len = self.buf.line_length(self.cursor.y);
                    if self.cursor.x > line_len {
                        self.cursor.x = line_len;
                    }

                    if self.cursor.y > self.offset.y + term_h - 2 {
                        self.scroll_by(1);
                    }
                }
            }
            Direction::Up => {
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
            Direction::Right => {
                if self.cursor.x < self.buf.line_length(self.cursor.y) {
                    self.cursor.x += 1;
                }
            }
        }
    }

    pub fn on_event(&mut self, evt: Event, path: PathBuf) -> u8 {
        match self.mode {
            EditorMode::Normal => match evt {
                Event::Key(Key::Char('i')) => {
                    self.mode = EditorMode::Insert;
                }
                Event::Key(Key::Char('q')) => return 1,
                Event::Key(Key::Char('h')) => {
                    self.cursor_by(Direction::Left);
                }
                Event::Key(Key::Char('j')) => {
                    self.cursor_by(Direction::Down);
                }
                Event::Key(Key::Char('k')) => {
                    self.cursor_by(Direction::Up);
                }
                Event::Key(Key::Char('l')) => {
                    self.cursor_by(Direction::Right);
                }
                Event::Key(Key::Ctrl('w')) => write(path, self.buf.to_string()).unwrap(),
                Event::Key(Key::Char(':')) => {
                    self.mode = EditorMode::Command;
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
                    self.buf.insert_str("  ", self.cursor.x, self.cursor.y);
                    self.cursor.x += 2;
                }
                Event::Key(Key::Char(c)) => {
                    self.buf.insert(c, self.cursor.x, self.cursor.y);
                    self.cursor_by(Direction::Right);
                }
                Event::Key(Key::Backspace) => {
                    if self.cursor.x > 0 {
                        self.buf.delete(self.cursor.x - 1, self.cursor.y);
                        self.cursor_by(Direction::Left);
                    } else if self.cursor.y > 0 {
                        self.buf.join_lines(self.cursor.y - 1);
                        self.cursor_by(Direction::Up);

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
            EditorMode::Command => {
                match evt {
                    Event::Key(Key::Ctrl('c')) => {
                        self.mode = EditorMode::Normal;
                    }
                    _ => {}
                }
            }
        }

        0
    }
}

impl From<String> for Editor {
    fn from(value: String) -> Self {
        Self {
            buf: CodeBuffer::new(value),
            mode: EditorMode::default(),
            cursor: Vec2::default(),
            offset: Vec2::default(),
        }
    }
}
