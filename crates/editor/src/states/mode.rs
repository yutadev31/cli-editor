use std::fmt::{self, Display, Formatter};

use crate::cmd::EditorCommand;

#[derive(Default, Clone, Hash, Eq, PartialEq)]
pub enum EditorMode {
    #[default]
    Normal,
    Command,
    Insert,
    Visual,
}

impl EditorMode {
    pub fn register_cmds(cmds: &mut EditorCommand) {
        cmds.register(
            "normal",
            Box::new(|editor| {
                editor.cmd_buf.clear();
                editor.set_mode(EditorMode::Normal);
            }),
        );
        cmds.register(
            "command",
            Box::new(|editor| editor.set_mode(EditorMode::Command)),
        );
        cmds.register(
            "insert_before",
            Box::new(|editor| editor.set_mode(EditorMode::Insert)),
        );
        cmds.register(
            "visual",
            Box::new(|editor| editor.set_mode(EditorMode::Visual)),
        );
    }
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EditorMode::Normal => "Normal".to_string(),
                EditorMode::Command => "Command".to_string(),
                EditorMode::Insert => "Insert".to_string(),
                EditorMode::Visual => "Visual".to_string(),
            }
        )
    }
}
