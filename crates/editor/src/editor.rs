mod cmd;
mod key;
mod states;

use std::{fs::read_to_string, io::Write, path::PathBuf};

use anyhow::Result;
use cmd::EditorCommand;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use key::EditorKeys;
use states::mode::EditorMode;
use states::EditorState;
use utils::cli::terminal_size;

pub struct Editor {
    cmds: EditorCommand,
    keys: EditorKeys,
    state: EditorState,
}

impl Editor {
    pub fn new(buf: String, path: Option<PathBuf>) -> Self {
        Self {
            cmds: EditorCommand::new(),
            keys: EditorKeys::new(),
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

    pub fn draw<T: Write>(&self, stdout: &mut T) -> Result<()> {
        let mode = self.state.get_mode();

        let (term_w, term_h) = terminal_size()?;
        let (cursor_x, cursor_y) = self.state.cursor.get_display(&self.state.buf);

        let str_buf = self.state.buf.to_string();
        let lines = str_buf.lines();

        execute!(stdout, Clear(ClearType::All))?;

        // Draw line numbers
        let len_count = self.state.buf.line_count();
        let line_num_w = len_count.to_string().len();
        let line_numbers: Vec<String> = (1..=len_count)
            .skip(self.state.offset.y)
            .take(term_h - 1)
            .map(|x| x.to_string())
            .collect();
        execute!(stdout, cursor::MoveTo(0, 1))?;
        execute!(stdout, Print(line_numbers.join("\r\n")))?;

        // Draw code
        lines
            .skip(self.state.offset.y)
            .take(term_h - 1)
            .enumerate()
            .for_each(|(index, line)| {
                execute!(
                    stdout,
                    cursor::MoveTo(1 + line_num_w as u16, 1 + index as u16)
                )
                .unwrap();
                execute!(stdout, Print(line)).unwrap();
            });

        // Draw info bar
        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(stdout, SetBackgroundColor(Color::White))?;
        execute!(stdout, SetForegroundColor(Color::Black))?;
        execute!(stdout, Print(" ".repeat(term_w)))?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        write!(
            stdout,
            " {} {},{} {},{}",
            mode, cursor_x, cursor_y, self.state.offset.x, self.state.offset.y
        )
        .unwrap();
        execute!(stdout, ResetColor)?;

        execute!(
            stdout,
            cursor::MoveTo(
                (cursor_x + 1 - self.state.offset.x + line_num_w) as u16,
                (cursor_y + 1 - self.state.offset.y) as u16
            )
        )?;

        if let EditorMode::Command = mode {
            execute!(stdout, cursor::MoveTo(1, term_h as u16 - 1))?;
            write!(stdout, ":{}", self.state.cmd_buf).unwrap();
        }

        match mode {
            EditorMode::Insert => write!(stdout, "{}", cursor::SetCursorStyle::SteadyBar).unwrap(),
            _ => write!(stdout, "{}", cursor::SetCursorStyle::SteadyBlock).unwrap(),
        }

        stdout.flush()?;

        Ok(())
    }

    fn on_insert_or_command_mode_event(&mut self, evt: Event) -> bool {
        let (cursor_x, cursor_y) = self.state.cursor.get_display(&self.state.buf);

        match evt {
            Event::Key(key) => match self.state.get_mode() {
                EditorMode::Insert => match (key.code, key.modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        if cursor_x < self.state.buf.line_length(cursor_y) {
                            self.state.buf.split_line(cursor_x, cursor_y);
                            self.state
                                .cursor
                                .move_by(&self.state.buf, &mut self.state.offset, 0, 1)
                                .unwrap();
                            self.state.cursor.move_x_to(&self.state.buf, 0)
                        } else {
                            self.state.buf.insert_line();
                            self.state
                                .cursor
                                .move_by(&self.state.buf, &mut self.state.offset, 0, 1)
                                .unwrap();
                            self.state.cursor.move_x_to(&self.state.buf, 0);
                        }
                    }
                    (KeyCode::Tab, KeyModifiers::NONE) => {
                        self.state.buf.insert_str("  ", cursor_x, cursor_y);
                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, 2, 0)
                            .unwrap();
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if cursor_x > 0 {
                            self.state.buf.delete(cursor_x - 1, cursor_y);
                            self.state
                                .cursor
                                .move_by(&self.state.buf, &mut self.state.offset, -1, 0)
                                .unwrap();
                        } else if cursor_y > 0 {
                            let line_len = self.state.buf.line_length(cursor_y - 1);

                            self.state
                                .cursor
                                .move_by(&self.state.buf, &mut self.state.offset, 0, -1)
                                .unwrap();
                            self.state.cursor.move_x_to(&self.state.buf, line_len);
                            self.state.buf.join_lines(cursor_y - 1);
                        }
                    }
                    (KeyCode::Delete, KeyModifiers::NONE) => {
                        let line_len = self.state.buf.line_length(cursor_y);
                        let len_count = self.state.buf.line_count();

                        if cursor_x < line_len {
                            self.state.buf.delete(cursor_x, cursor_y);
                        } else if cursor_y < len_count {
                            self.state.buf.join_lines(cursor_y);
                        }
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                        self.state.buf.insert(c, cursor_x, cursor_y);
                        self.state
                            .cursor
                            .move_by(&self.state.buf, &mut self.state.offset, 1, 0)
                            .unwrap();
                    }
                    _ => return false,
                },
                EditorMode::Command => match (key.code, key.modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        self.cmds
                            .run(self.state.cmd_buf.clone().as_str(), &mut self.state);
                        self.cmds.run("normal", &mut self.state);
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if self.state.cmd_buf.is_empty() {
                            self.cmds.run("normal", &mut self.state);
                        }
                        self.state.cmd_buf.pop();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                        self.state.cmd_buf.push(c);
                    }
                    _ => return false,
                },
                _ => return false,
            },
            _ => return false,
        }

        true
    }

    pub fn on_event(&mut self, evt: Event) -> bool {
        if self.on_insert_or_command_mode_event(evt.clone()) {
            return self.state.is_quit;
        };
        
        if let Event::Key(key) = evt {
            let mut keys = self.state.get_keys();
            keys.push((key.code, key.modifiers));

            let cmd = self.keys.get(self.state.get_mode(), keys);
            if let Some(cmd) = cmd {
                self.state.clear_keys();
                self.cmds.run(cmd, &mut self.state);
            } else {
                self.state.push_key((key.code, key.modifiers));
            }
         
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
