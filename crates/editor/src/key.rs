use std::collections::HashMap;

use termion::event::Key;

use crate::states::mode::EditorMode;

type KeyMap = HashMap<(EditorMode, Vec<Key>), String>;

pub struct EditorKeys {
    pub keys: KeyMap,
}

impl EditorKeys {
    pub fn new() -> Self {
        Self::register_default_keys()
    }

    pub fn register(&mut self, mode: EditorMode, key: Vec<Key>, cmd: &str) {
        self.keys.insert((mode, key), cmd.to_string());
    }

    pub fn n_register(&mut self, key: Vec<Key>, cmd: &str) {
        self.register(EditorMode::Normal, key, cmd);
    }

    pub fn v_register(&mut self, key: Vec<Key>, cmd: &str) {
        self.register(EditorMode::Visual, key, cmd);
    }

    pub fn nv_register(&mut self, key: Vec<Key>, cmd: &str) {
        self.n_register(key.clone(), cmd);
        self.v_register(key, cmd);
    }

    pub fn register_default_keys() -> Self {
        let mut keys = Self {
            keys: HashMap::new(),
        };

        // Normal mode -> Command mode
        keys.n_register(vec![Key::Char(':')], "command");

        // Insert mode -> Normal mode
        keys.register(EditorMode::Insert, vec![Key::Ctrl('c')], "normal");
        keys.register(EditorMode::Insert, vec![Key::Esc], "normal");

        // Visual mode -> Normal mode
        keys.register(EditorMode::Visual, vec![Key::Ctrl('c')], "normal");
        keys.register(EditorMode::Visual, vec![Key::Esc], "normal");

        // Command mode -> Normal mode
        keys.register(EditorMode::Command, vec![Key::Ctrl('c')], "normal");
        keys.register(EditorMode::Command, vec![Key::Esc], "normal");

        // Movement (Cursor keys)
        keys.nv_register(vec![Key::Char('h')], "left");
        keys.nv_register(vec![Key::Char('j')], "down");
        keys.nv_register(vec![Key::Char('k')], "up");
        keys.nv_register(vec![Key::Char('l')], "right");
        keys.nv_register(vec![Key::Char('g'), Key::Char('g')], "top");
        keys.nv_register(vec![Key::Char('G')], "bottom");
        keys.nv_register(vec![Key::Char('b')], "back_word_left");
        keys.nv_register(vec![Key::Char('g'), Key::Char('e')], "back_word_right");
        keys.nv_register(vec![Key::Char('w')], "next_word_left");
        keys.nv_register(vec![Key::Char('e')], "next_word_right");
        keys.nv_register(vec![Key::Char('0')], "line_start");
        keys.nv_register(vec![Key::Char('^')], "first_char");
        keys.nv_register(vec![Key::Char('$')], "line_end");
        keys.nv_register(vec![Key::Char('H')], "window_top");
        keys.nv_register(vec![Key::Char('M')], "window_middle");
        keys.nv_register(vec![Key::Char('L')], "window_bottom");
        keys.nv_register(vec![Key::Char('%')], "match_paren");

        // Insert
        keys.n_register(vec![Key::Char('i')], "insert_before");
        keys.n_register(vec![Key::Char('I')], "insert_line_start");
        keys.n_register(vec![Key::Char('a')], "insert_after");
        keys.n_register(vec![Key::Char('A')], "insert_line_end");
        keys.n_register(vec![Key::Char('o')], "insert_below");
        keys.n_register(vec![Key::Char('O')], "insert_above");

        // Visual
        keys.n_register(vec![Key::Char('v')], "visual");
        keys.n_register(vec![Key::Char('V')], "visual_line");
        keys.n_register(vec![Key::Ctrl('v')], "visual_block");

        // Cut, Copy, Paste
        keys.n_register(vec![Key::Char('d'), Key::Char('d')], "delete_line");
        keys.n_register(vec![Key::Char('y'), Key::Char('y')], "copy_line");
        keys.n_register(vec![Key::Char('p')], "paste_after");
        keys.n_register(vec![Key::Char('P')], "paste_before");
        keys.v_register(vec![Key::Char('y')], "copy");
        keys.v_register(vec![Key::Char('d')], "delete");

        // Undo, Redo
        keys.n_register(vec![Key::Char('u')], "undo");
        keys.n_register(vec![Key::Ctrl('r')], "redo");

        // Indent
        keys.n_register(vec![Key::Char('>'), Key::Char('>')], "indent");
        keys.n_register(vec![Key::Char('<'), Key::Char('<')], "de-indent");
        keys.n_register(vec![Key::Char('>'), Key::Char('%')], "block_indent");
        keys.n_register(vec![Key::Char('<'), Key::Char('%')], "block_de-indent");
        keys.v_register(vec![Key::Char('>')], "indent");
        keys.v_register(vec![Key::Char('<')], "indent");

        keys
    }

    pub fn get(&self, mode: EditorMode, key: Vec<Key>) -> Option<&String> {
        self.keys.get(&(mode, key))
    }
}
