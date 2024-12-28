pub fn terminal_size() -> (usize, usize) {
    let (cols, rows) = termion::terminal_size().unwrap();
    (cols as usize, rows as usize)
}
