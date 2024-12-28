pub struct CodeBuffer {
    lines: Vec<String>,
}

impl CodeBuffer {
    pub fn new(buf: String) -> Self {
        Self {
            lines: buf.lines().map(|line| line.to_string()).collect(),
        }
    }

    pub fn insert(&mut self, c: char, x: usize, y: usize) {
        self.lines[y].insert(x, c);
    }

    pub fn delete(&mut self, x: usize, y: usize) {
        self.lines[y].remove(x);
    }

    // ２つの行を繋ぐ (間の改行を削除) 関数
    // yには上の行を指定する
    pub fn join_lines(&mut self, y: usize) {
        if y + 1 < self.lines.len() {
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

impl ToString for CodeBuffer {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
