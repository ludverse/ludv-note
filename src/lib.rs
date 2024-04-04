use ids::id_from_note_path;
use walkdir::{DirEntry, WalkDir};

pub mod config;
pub mod ids;

pub enum Action {
    Present(String),
    Edit(String),
    Delete(String),
    List,
    Find(String),
    Help
}

pub fn iterate_notes<F>(config: &config::Config, callback: F)
where
    F: Fn(DirEntry, String)
{
    let entries = WalkDir::new(&config.notes_path)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in entries {
        let entry_path = entry.path();
        if !entry_path.is_file() { continue; };

        let id = id_from_note_path(config, entry_path);

        callback(entry, id);
    }
}
