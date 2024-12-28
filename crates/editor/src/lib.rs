pub mod buf;

use crate::buf::CodeBuffer;

pub struct Editor {
    buf: CodeBuffer,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buf: CodeBuffer::new(String::new()),
        }
    }

    pub fn draw(self) {
        let buf = self.buf.to_string();
        let lines = buf.lines();

        lines.for_each(|line| println!("{}", line));
    }
}

impl From<String> for Editor {
    fn from(value: String) -> Self {
        Self {
            buf: CodeBuffer::new(value),
        }
    }
}

impl From<&str> for Editor {
    fn from(value: &str) -> Self {
        Self {
            buf: CodeBuffer::new(value.to_string()),
        }
    }
}
