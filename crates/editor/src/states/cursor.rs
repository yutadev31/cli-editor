use utils::{cli::terminal_size, types::Vec2};

use crate::{cmd::EditorCommand, states::buf::CodeBuffer};

#[derive(Default, Clone)]
pub struct EditorCursor {
    x: usize,
    y: usize,
}

impl EditorCursor {
    pub fn move_by(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>, x: isize, y: isize) {
        let (_, term_h) = terminal_size();
        let term_h = term_h - 1;
        let buf_len = buf.line_count();
        let line_len = buf.line_length(self.y);

        match x.cmp(&0) {
            std::cmp::Ordering::Less => {
                if self.x < -x as usize {
                    self.x = 0;
                } else {
                    self.x -= -x as usize;
                }
            }
            std::cmp::Ordering::Greater => {
                if self.x + x as usize > line_len {
                    self.x = line_len;
                } else {
                    self.x += x as usize;
                }
            }
            std::cmp::Ordering::Equal => {}
        }

        if y < 0 {
            if self.y < -y as usize {
                self.y = 0;
            } else {
                self.y -= -y as usize;
            }

            if self.y < offset.y {
                offset.y = self.y;
            }
        } else {
            if self.y + y as usize > buf_len - 1 {
                self.y = buf_len - 1;
            } else {
                self.y += y as usize;
            }

            if self.y >= offset.y + term_h {
                offset.y = self.y - term_h + 1;
            }
        }
    }

    pub fn move_x_to(&mut self, buf: &CodeBuffer, x: usize) {
        let line_len = buf.line_length(self.y);

        if x > line_len {
            self.x = line_len;
        } else {
            self.x = x;
        }
    }

    pub fn move_y_to(&mut self, buf: &CodeBuffer, y: usize) {
        let buf_len = buf.line_count();

        if y > buf_len {
            self.y = buf_len;
        } else {
            self.y = y;
        }
    }

    pub fn get_display(&self, buf: &CodeBuffer) -> (usize, usize) {
        let line_len = buf.line_length(self.y);
        let x = if self.x > line_len { line_len } else { self.x };

        (x, self.y)
    }

    pub fn cmd_left(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_by(buf, offset, -1, 0);
    }

    pub fn cmd_right(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_by(buf, offset, 1, 0);
    }

    pub fn cmd_up(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_by(buf, offset, 0, -1);
    }

    pub fn cmd_down(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_by(buf, offset, 0, 1);
    }

    pub fn cmd_top(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_y_to(buf, 0);
        offset.y = 0;
    }

    pub fn cmd_bottom(&mut self, buf: &CodeBuffer, offset: &mut Vec2<usize>) {
        self.move_y_to(buf, buf.line_count() - 1);
        offset.y = buf.line_count() - 1;
    }

    pub fn cmd_line_start(&mut self, buf: &CodeBuffer) {
        self.move_x_to(buf, 0);
    }

    pub fn cmd_line_end(&mut self, buf: &CodeBuffer) {
        self.move_x_to(buf, buf.line_length(self.y));
    }

    pub fn register_cmds(cmds: &mut EditorCommand) {
        cmds.register(
            "left",
            Box::new(|editor| editor.cursor.cmd_left(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "right",
            Box::new(|editor| editor.cursor.cmd_right(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "up",
            Box::new(|editor| editor.cursor.cmd_up(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "down",
            Box::new(|editor| editor.cursor.cmd_down(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "top",
            Box::new(|editor| editor.cursor.cmd_top(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "bottom",
            Box::new(|editor| editor.cursor.cmd_bottom(&editor.buf, &mut editor.offset)),
        );
        cmds.register(
            "line_start",
            Box::new(|editor| editor.cursor.cmd_line_start(&editor.buf)),
        );
        cmds.register(
            "line_end",
            Box::new(|editor| editor.cursor.cmd_line_end(&editor.buf)),
        );
    }
}
