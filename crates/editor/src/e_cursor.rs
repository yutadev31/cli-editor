use crate::buf::CodeBuffer;

pub struct EditorCursor {
    x: usize,
    y: usize,
}

impl EditorCursor {
    pub fn move_by(&mut self, buf: &CodeBuffer, x: isize, y: isize) {
        let buf_len = buf.line_count();
        let line_len = buf.line_length(self.y);

        if x < 0 {
            if self.x < -x as usize {
                self.x = 0;
            } else {
                self.x -= -x as usize;
            }
        } else if x > 0 {
            if self.x + x as usize > line_len {
                self.x = line_len;
            } else {
                self.x += x as usize;
            }
        }

        if y < 0 {
            if self.y < -y as usize {
                self.y = 0;
            } else {
                self.y -= -y as usize;
            }
        } else {
            if self.y + y as usize > buf_len {
                self.y = buf_len;
            } else {
                self.y += y as usize;
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

    pub fn move_to(&mut self, buf: &CodeBuffer, x: usize, y: usize) {
        self.move_x_to(buf, x);
        self.move_y_to(buf, y);
    }

    pub fn get_display(&self, buf: &CodeBuffer) -> (usize, usize) {
        let line_len = buf.line_length(self.y);
        let x = if self.x > line_len { line_len } else { self.x };

        (x, self.y)
    }
}

impl Default for EditorCursor {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
