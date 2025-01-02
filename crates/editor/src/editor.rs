mod cmd;
mod key;
mod states;

use std::{
    fs::{read_to_string, write},
    io::Write,
    path::PathBuf,
};

use cmd::EditorCommand;
use states::mode::EditorMode;
use states::EditorState;
use termion::{
    clear, color, cursor,
    event::{Event, Key},
};
use utils::{cli::terminal_size, types::Vec2};

pub struct Editor {
    cmds: EditorCommand,
    state: EditorState,
}

impl Editor {
    pub fn new(buf: String, path: Option<PathBuf>) -> Self {
        Self {
            cmds: EditorCommand::new(),
            state: EditorState::new(buf, path),
        }
    }

    pub fn open(path: PathBuf) -> Self {
        let mut buf = read_to_string(path.clone()).unwrap_or_default();

        if buf.is_empty() {
            buf = String::from("\n");
        }

        Self::new(buf, Some(path))
    }

    pub fn draw<T: Write>(&self, stdout: &mut T) {
        let mode = self.state.get_mode();

        let (term_w, term_h) = terminal_size();
        let (cursor_x, cursor_y) = self.state.cursor.get_display(&self.state.buf);

        let str_buf = self.state.buf.to_string();
        let lines = str_buf.lines();

        write!(stdout, "{}", clear::All).unwrap();

        // Draw line numbers
        let len_count = self.state.buf.line_count();
        let line_num_w = len_count.to_string().len();
        let line_numbers: Vec<String> = (1..=len_count)
            .skip(self.state.offset.y)
            .take(term_h - 1)
            .map(|x| x.to_string())
            .collect();
        write!(stdout, "{}", cursor::Goto(1, 2)).unwrap();
        write!(stdout, "{}", line_numbers.join("\r\n")).unwrap();

        // Draw code
        lines
            .skip(self.state.offset.y)
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

        // Draw info bar
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(stdout, "{}", color::Bg(color::White)).unwrap();
        write!(stdout, "{}", color::Fg(color::Black)).unwrap();
        write!(stdout, "{}", " ".repeat(term_w)).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(
            stdout,
            " {} {},{} {},{}",
            mode, cursor_x, cursor_y, self.state.offset.x, self.state.offset.y
        )
        .unwrap();
        write!(stdout, "{}", color::Bg(color::Reset)).unwrap();
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();

        write!(
            stdout,
            "{}",
            cursor::Goto(
                (cursor_x + 2 - self.state.offset.x + line_num_w) as u16,
                (cursor_y + 2 - self.state.offset.y) as u16
            )
        )
        .unwrap();

        if let EditorMode::Command = mode {
            write!(stdout, "{}", cursor::Goto(1, term_h as u16 - 1)).unwrap();
            write!(stdout, ":{}", self.state.cmd_buf).unwrap();
        }

        match mode {
            EditorMode::Insert => write!(stdout, "{}", cursor::SteadyBar).unwrap(),
            _ => write!(stdout, "{}", cursor::SteadyBlock).unwrap(),
        }

        stdout.flush().unwrap();
    }

    fn on_event_cursor(&mut self, evt: Event, _cursor_x: usize, cursor_y: usize) -> bool {
        let key_buf = self.state.get_key_buf();

        if let Some(c) = key_buf {
            self.state.set_key_buf(None);
            match c {
                'g' => match evt {
                    Event::Key(Key::Char('g')) => {
                        self.state.cursor.move_y_to(&self.state.buf, 0);
                    }
                    _ => return false,
                },
                _ => return false,
            }
        } else if let Event::Key(Key::Char(c)) = evt {
            match c {
                'h' => {
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, -1, 0);
                }
                'j' => {
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, 0, 1);
                }
                'k' => {
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, 0, -1);
                }
                'l' => {
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, 1, 0);
                }
                '0' => {
                    self.state.cursor.move_x_to(&self.state.buf, 0);
                }
                '$' => {
                    self.state
                        .cursor
                        .move_x_to(&self.state.buf, self.state.buf.line_length(cursor_y));
                }
                'G' => {
                    self.state
                        .cursor
                        .move_y_to(&self.state.buf, self.state.buf.line_count() - 1);
                }
                _ => return false,
            }
        }

        true
    }

    pub fn on_event(&mut self, evt: Event, path: PathBuf) -> bool {
        let (cursor_x, cursor_y) = self.state.cursor.get_display(&self.state.buf);

        match self.state.get_mode() {
            EditorMode::Normal => {
                if self.on_event_cursor(evt.clone(), cursor_x, cursor_y) {
                    return self.state.is_quit;
                }

                match evt {
                    Event::Key(Key::Char('i')) => {
                        self.state.set_mode(EditorMode::Insert);
                    }
                    Event::Key(Key::Char('v')) => {
                        self.state.set_mode(EditorMode::Visual);
                        self.state.visual_start = Vec2::new(cursor_x, cursor_y);
                    }
                    Event::Key(Key::Char(':')) => {
                        self.state.set_mode(EditorMode::Command);
                        self.state.cmd_buf = String::new();
                    }
                    Event::Key(Key::Char(c)) => {
                        self.state.set_key_buf(Some(c));
                    }
                    _ => {}
                }
            }
            EditorMode::Insert => match evt {
                Event::Key(Key::Char('\n')) => {
                    if cursor_x < self.state.buf.line_length(cursor_y) {
                        self.state.buf.split_line(cursor_x, cursor_y);
                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, 0, 1);
                        self.state.cursor.move_x_to(&self.state.buf, 0)
                    } else {
                        self.state.buf.insert_line();
                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, 0, 1);
                        self.state.cursor.move_x_to(&self.state.buf, 0);
                    }
                }
                Event::Key(Key::Char('\t')) => {
                    self.state.buf.insert_str("  ", cursor_x, cursor_y);
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, 2, 0);
                }
                Event::Key(Key::Char(c)) => {
                    self.state.buf.insert(c, cursor_x, cursor_y);
                    self.state
                        .cursor
                        .move_by(&self.state.buf, &mut self.state.offset, 1, 0);
                }
                Event::Key(Key::Backspace) => {
                    if cursor_x > 0 {
                        self.state.buf.delete(cursor_x - 1, cursor_y);
                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, -1, 0);
                    } else if cursor_y > 0 {
                        let line_len = self.state.buf.line_length(cursor_y - 1);

                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, 0, -1);
                        self.state.cursor.move_x_to(&self.state.buf, line_len);
                        self.state.buf.join_lines(cursor_y - 1);
                    }
                }
                Event::Key(Key::Delete) => {
                    let line_len = self.state.buf.line_length(cursor_y);
                    let len_count = self.state.buf.line_count();

                    if cursor_x < line_len {
                        self.state.buf.delete(cursor_x, cursor_y);
                    } else if cursor_y < len_count {
                        self.state.buf.join_lines(cursor_y);
                    }
                }
                Event::Key(Key::Ctrl('c')) => {
                    self.state.set_mode(EditorMode::Normal);
                }
                _ => {}
            },
            EditorMode::Visual => {
                if self.on_event_cursor(evt.clone(), cursor_x, cursor_y) {
                    return self.state.is_quit;
                }

                if let Event::Key(Key::Ctrl('c')) = evt {
                    self.state.set_mode(EditorMode::Normal);
                }
            }
            EditorMode::Command => match evt {
                Event::Key(Key::Char('\n')) => {
                    self.cmds
                        .run(self.state.cmd_buf.clone().as_str(), &mut self.state);
                    self.state.set_mode(EditorMode::Normal);
                }
                Event::Key(Key::Backspace) => {
                    self.state.cmd_buf.pop();
                }
                Event::Key(Key::Char(c)) => {
                    self.state.cmd_buf.push(c);
                }
                Event::Key(Key::Ctrl('c')) => {
                    self.state.set_mode(EditorMode::Normal);
                }
                _ => {}
            },
        }

        self.state.is_quit
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new(String::new(), None)
    }
}

impl From<String> for Editor {
    fn from(buf: String) -> Self {
        Self::new(buf, None)
    }
}
