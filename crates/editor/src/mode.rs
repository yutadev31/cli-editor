pub enum EditorMode {
    Normal,
    Command,
    Insert,
    Visual,
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::Normal
    }
}

impl ToString for EditorMode {
    fn to_string(&self) -> String {
        match self {
            EditorMode::Normal => "Normal".to_string(),
            EditorMode::Command => "Command".to_string(),
            EditorMode::Insert => "Insert".to_string(),
            EditorMode::Visual => "Visual".to_string(),
        }
    }
}
