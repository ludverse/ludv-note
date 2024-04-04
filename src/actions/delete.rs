use std::{
    fs,
    io::ErrorKind,
};

use ludv_note::config::Config;

pub fn delete(config: &Config, note_id: &String) -> Result<(), String> {
    let file_name = format!("{}.md", note_id);
    let note_path = config.notes_path.join(file_name);

    fs::remove_file(&note_path)
        .map_err(|err| {
            match err.kind() {
                ErrorKind::NotFound => format!("note `{}` not found ({})", note_id, note_path.display()),
                _ => panic!("failed to delete note: {:?}", err)
            }
        })?;

    println!("sucessfully deleted note `{}` ({})", note_id, note_path.display());

    Ok(())
}

