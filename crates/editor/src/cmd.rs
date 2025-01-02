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

        // Movement
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

        cmds
    }

    pub fn run(&mut self, cmd: &str, editor: &mut EditorState) {
        if let Some(cmd) = self.cmds.get(cmd) {
            cmd(editor);
        }
    }
}
