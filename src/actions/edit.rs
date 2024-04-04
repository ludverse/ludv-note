use crate::{
    fs,
    env,
    process
};

use ludv_note::{config::Config, ids::note_path_from_id};

pub fn edit(config: &Config, note_id: &String) -> Result<(), String> {
    let note_path = note_path_from_id(config, &note_id)
        .ok_or_else(|| String::from("invalid note id string"))?;

    let editor = env::var("EDITOR")
        .unwrap_or_else(|_| String::from("nano"));

    let mut subdirs_path = note_path.to_path_buf();
    subdirs_path.set_file_name("");
    fs::create_dir_all(&subdirs_path)
        .expect("error while creating the specified subdirectories since they don't exist");

    process::Command::new(&editor)
        .arg(note_path)
        .status()
        .expect(&format!("failed to launch editor ({})", editor));

    Ok(())
}
