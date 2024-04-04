use std::{
    fs,
    io::{Write, ErrorKind},
    process,
};

use ludv_note::config::Config;

pub fn present(config: &Config, note_id: &String) -> Result<(), String> {
    let file_name = format!("{}.md", note_id);
    let note_path = config.notes_path.join(file_name);

    let note_contents = fs::read_to_string(&note_path)
        .map_err(|err| {
            match err.kind() {
                ErrorKind::NotFound => format!("note `{}` not found ({})", note_id, note_path.display()),
                _ => panic!("failed to read note contents: {:?}", err)
            }
        })?;

    let formatted = termimad::term_text(note_contents.as_str())
        .to_string();

    let mut less = process::Command::new("less")
        .args(["-RF"])
        .stdin(process::Stdio::piped())
        .spawn()
        .expect("failed to run command less");

    let mut less_stdin = less.stdin.take().unwrap();
    less_stdin.write_all(formatted.as_bytes()).unwrap();
    drop(less_stdin);

    less.wait().unwrap();

    Ok(())
}

