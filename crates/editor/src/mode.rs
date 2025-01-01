use std::fmt::{self, Display, Formatter};

#[derive(Default, Clone)]
pub enum EditorMode {
    #[default]
    Normal,
    Command,
    Insert,
    Visual,
}

impl Display for EditorMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EditorMode::Normal => "Normal".to_string(),
            EditorMode::Command => "Command".to_string(),
            EditorMode::Insert => "Insert".to_string(),
            EditorMode::Visual => "Visual".to_string(),
        }) 
    }
}

