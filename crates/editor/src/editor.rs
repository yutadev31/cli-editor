mod buf;
mod e_cursor;
mod mode;

use std::{
    fs::{read_to_string, write},
    io::Write,
    path::PathBuf,
};

use e_cursor::EditorCursor;
use mode::EditorMode;
use termion::{
    clear, color, cursor,
    event::{Event, Key},
};
use utils::{cli::terminal_size, types::Vec2};

use crate::buf::CodeBuffer;

pub struct Editor {
    buf: CodeBuffer,
    mode: EditorMode,
    cursor: EditorCursor,
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
        let (cursor_x, cursor_y) = self.cursor.get_display(&self.buf);

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
            cursor_x,
            cursor_y,
        )
        .unwrap();
        write!(stdout, "{}", color::Bg(color::Reset)).unwrap();
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();

        write!(
            stdout,
            "{}",
            cursor::Goto(
                (cursor_x + 2 - self.offset.x + line_num_w) as u16,
                (cursor_y + 2 - self.offset.y) as u16
            )
        )
        .unwrap();

        match self.mode {
            EditorMode::Insert => write!(stdout, "{}", cursor::SteadyBar).unwrap(),
            _ => write!(stdout, "{}", cursor::SteadyBlock).unwrap(),
        }

        stdout.flush().unwrap();
    }

    pub fn scroll_by(&mut self, y: isize) {
        self.offset.y = (self.offset.y as isize + y) as usize;
    }

    pub fn on_event(&mut self, evt: Event, path: PathBuf) -> u8 {
        let (cursor_x, cursor_y) = self.cursor.get_display(&self.buf);

        match self.mode {
            EditorMode::Normal => match evt {
                Event::Key(Key::Char('i')) => {
                    self.mode = EditorMode::Insert;
                }
                Event::Key(Key::Char('q')) => return 1,
                Event::Key(Key::Char('h')) => {
                    self.cursor.move_by(&self.buf, -1, 0);
                }
                Event::Key(Key::Char('j')) => {
                    self.cursor.move_by(&self.buf, 0, 1);
                }
                Event::Key(Key::Char('k')) => {
                    self.cursor.move_by(&self.buf, 0, -1);
                }
                Event::Key(Key::Char('l')) => {
                    self.cursor.move_by(&self.buf, 1, 0);
                }
                Event::Key(Key::Ctrl('w')) => write(path, self.buf.to_string()).unwrap(),
                Event::Key(Key::Char(':')) => {
                    self.mode = EditorMode::Command;
                }
                _ => {}
            },
            EditorMode::Insert => match evt {
                Event::Key(Key::Char('\n')) => {
                    if cursor_x < self.buf.line_length(cursor_y) {
                        self.buf.split_line(cursor_x, cursor_y);
                        self.cursor.move_by(&self.buf, 0, 1);
                        self.cursor.move_x_to(&self.buf, 0)
                    } else {
                        self.buf.insert_line();
                        self.cursor.move_by(&self.buf, 0, 1);
                        self.cursor.move_x_to(&self.buf, 0);
                    }
                }
                Event::Key(Key::Char('\t')) => {
                    self.buf.insert_str("  ", cursor_x, cursor_y);
                    self.cursor.move_by(&self.buf, 2, 0);
                }
                Event::Key(Key::Char(c)) => {
                    self.buf.insert(c, cursor_x, cursor_y);
                    self.cursor.move_by(&self.buf, 1, 0);
                }
                Event::Key(Key::Backspace) => {
                    if cursor_x > 0 {
                        self.buf.delete(cursor_x - 1, cursor_y);
                        self.cursor.move_by(&self.buf, -1, 0);
                    } else if cursor_y > 0 {
                        let line_len = self.buf.line_length(cursor_y - 1);

                        self.buf.join_lines(cursor_y - 1);
                        self.cursor.move_by(&self.buf, 0, -1);
                        self.cursor.move_x_to(&self.buf, line_len);
                    }
                }
                Event::Key(Key::Delete) => {
                    let line_len = self.buf.line_length(cursor_y);
                    let len_count = self.buf.line_count();

                    if cursor_x < line_len {
                        self.buf.delete(cursor_x, cursor_y);
                    } else if cursor_y < len_count {
                        self.buf.join_lines(cursor_y);
                    }
                }
                Event::Key(Key::Ctrl('c')) => {
                    self.mode = EditorMode::Normal;
                }
                _ => {}
            },
            EditorMode::Visual => {}
            EditorMode::Command => match evt {
                Event::Key(Key::Ctrl('c')) => {
                    self.mode = EditorMode::Normal;
                }
                _ => {}
            },
        }

        0
    }
}

impl From<String> for Editor {
    fn from(value: String) -> Self {
        Self {
            buf: CodeBuffer::new(value),
            mode: EditorMode::default(),
            cursor: EditorCursor::default(),
            offset: Vec2::default(),
        }
    }
}
