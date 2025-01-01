use std::collections::HashMap;

use crate::state::EditorState;

pub type Command = Box<dyn Fn(&mut EditorState)>;
type CommandMap = HashMap<String, Command>;

pub struct EditorCommand {
    cmds: CommandMap,
}

impl EditorCommand {
    pub fn new() -> Self {
        Self::register_default_commands()
    }

    pub fn register(&mut self, cmd: &str, f: Command) {
        self.cmds.insert(cmd.to_string(), f);
    }

    pub fn register_default_commands() -> Self {
        let mut cmds = Self {
            cmds: HashMap::new(),
        };
        cmds.register("q", Box::new(|editor| editor.quit()));
        cmds.register("w", Box::new(|editor| editor.write()));
        cmds.register(
            "wq",
            Box::new(|editor| {
                editor.write();
                editor.quit();
            }),
        );
        cmds.register(
            "x",
            Box::new(|editor| {
                editor.write();
                editor.quit();
            }),
        );
        cmds
    }

    pub fn run(&mut self, cmd: &str, editor: &mut EditorState) {
        if let Some(cmd) = self.cmds.get(cmd) {
            cmd(editor);
        }
    }
}
