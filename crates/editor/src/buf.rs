use std::fmt::{self, Display, Formatter};

use utils::text::lines;

pub struct CodeBuffer {
    lines: Vec<String>,
}

impl CodeBuffer {
    pub fn new(buf: String) -> Self {
        Self { lines: lines(buf) }
    }

    pub fn insert(&mut self, c: char, x: usize, y: usize) {
        self.lines[y].insert(x, c);
    }

    pub fn insert_str(&mut self, s: &str, x: usize, y: usize) {
        self.lines[y].insert_str(x, s);
    }

    pub fn insert_line(&mut self) {
        self.lines.push("".to_string());
    }

    pub fn delete(&mut self, x: usize, y: usize) {
        self.lines[y].remove(x);
    }

    // ２つの行を繋ぐ (間の改行を削除) 関数
    // yには上の行を指定する
    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.line_count() {
            let combined = self.lines[y].clone() + &self.lines[y + 1];
            self.lines[y] = combined;
            self.lines.remove(y + 1);
        }
    }

    pub fn split_line(&mut self, x: usize, y: usize) {
        let original = self.lines[y].clone();
        let (p0, p1) = original.split_at(x);
        self.lines[y] = p0.to_string();
        self.lines.insert(y + 1, p1.to_string());
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line_length(&self, line: usize) -> usize {
        self.lines[line].len()
    }
}

impl Display for  CodeBuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}
