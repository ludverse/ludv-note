use std::path::{self, Path};

use crate::config::Config;

pub fn id_from_note_path(config: &Config, path: &Path) -> String
{
    let notes_dir_depth = config.notes_path.components().count();

    let mut components = path.components();
    for _ in 0..notes_dir_depth {
        components.next();
    }

    let mut id_path = components
        .as_path()
        .to_path_buf();
    id_path.set_extension("");

    id_path.to_str().unwrap().to_string()
}

pub fn note_path_from_id(config: &Config, id: &str) -> Option<path::PathBuf> {
    if id.starts_with(path::MAIN_SEPARATOR) || id.ends_with(path::MAIN_SEPARATOR) {
        return None;
    }

    let file_name = format!("{}.md", id);
    let note_path = config.notes_path.join(file_name);

    Some(note_path)
}
