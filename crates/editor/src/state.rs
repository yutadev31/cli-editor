use std::{fs::write, path::PathBuf};

use utils::types::Vec2;

use crate::{buf::CodeBuffer, cmd::EditorCommand, e_cursor::EditorCursor, mode::EditorMode};

#[derive(Clone)]
pub struct EditorState {
    pub buf: CodeBuffer,
    mode: EditorMode,
    pub cursor: EditorCursor,
    pub offset: Vec2<usize>,
    key_buf: Option<char>,
    pub visual_start: Vec2<usize>,
    pub cmd_buf: String,
    path: Option<PathBuf>,
    pub is_quit: bool,
}

impl EditorState {
    pub fn new(buf: String) -> Self {
        Self {
            buf: CodeBuffer::new(buf),
            mode: EditorMode::default(),
            cursor: EditorCursor::default(),
            offset: Vec2::default(),
            key_buf: None,
            visual_start: Vec2::default(),
            cmd_buf: String::new(),
            path: None,
            is_quit: false,
        }
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> EditorMode {
        self.mode.clone()
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
    }

    pub fn set_key_buf(&mut self, key: Option<char>) {
        self.key_buf = key;
    }

    pub fn get_key_buf(&self) -> Option<char> {
        self.key_buf
    }

    pub fn write(&self) {
        if let Some(path) = &self.get_path() {
            write(path, self.buf.to_string()).unwrap();
        }
    }

    pub fn quit(&mut self) {
        self.is_quit = true;
    }
}