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
            Box::new(|editor| {
                editor
                    .cursor
                    .move_by(&editor.buf, &mut editor.offset, -1, 0)
            }),
        );
        cmds.register(
            "right",
            Box::new(|editor| editor.cursor.move_by(&editor.buf, &mut editor.offset, 1, 0)),
        );
        cmds.register(
            "up",
            Box::new(|editor| {
                editor
                    .cursor
                    .move_by(&editor.buf, &mut editor.offset, 0, -1)
            }),
        );
        cmds.register(
            "down",
            Box::new(|editor| editor.cursor.move_by(&editor.buf, &mut editor.offset, 0, 1)),
        );
        cmds.register(
            "top",
            Box::new(|editor| editor.cursor.move_y_to(&editor.buf, 0)),
        );
        cmds.register(
            "bottom",
            Box::new(|editor| {
                editor
                    .cursor
                    .move_y_to(&editor.buf, editor.buf.line_count() - 1)
            }),
        );
        cmds.register(
            "line_start",
            Box::new(|editor| editor.cursor.move_x_to(&editor.buf, 0)),
        );
        cmds.register(
            "line_end",
            Box::new(|editor| {
                editor.cursor.move_x_to(
                    &editor.buf,
                    editor
                        .buf
                        .line_length(editor.cursor.get_display(&editor.buf).1),
                )
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
