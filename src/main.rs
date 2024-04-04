use std::{
    env,
    process,
    fmt::Display, fs
};

use expanduser::expanduser;
use ludv_note::{
    config::Config,
    Action
};

mod actions;

#[cfg(feature = "grep")]
pub mod grep;

#[cfg(not(feature = "grep"))]
pub mod grep {
    use ludv_note::config::Config;

    pub fn find_contents(config: &Config, pattern: &str) {
        panic!("tried to use grep without grep enabled");
    }
}

const DEFAULT_NOTE_DIR: &str = "~/.local/share/ludv-notes/";
const HELP_USAGE: &str = "LUDV'S NOTE TAKER

Usage: ludv-note [OPTIONS] NOTE_ID
       ludv-note [OPTIONS] -e NOTE_ID
       ludv-note [OPTIONS] -d NOTE_ID
       ludv-note [OPTIONS] -f PATTERN

ACTIONS
  default       Present note
  -e, --edit    Edit or create a note
  -d, --delete  Delete the specified note
  -l, --list    List all notes
  -f, --find    Regex search notes by id

OPTIONS
  -c, --find-contents  Search by contents instead of id (requires grep feature flag)
  -h, --help           Display this very message and exit

ENV
  EDITOR          Command name to use for editing. Default: \"nano\"
  LUDV_NOTES_DIR  Where notes are stored. Default: \"~/.local/share/ludv-notes/\"

EXAMPLES
  EDITOR=nvim ludv-note -e grep_cmd
  ludv-note -e new-sub-dir/holy_cmd
  ludv-note -f _cmd
  ludv-note -c -f '[0-9]+'
  LUDV_NOTES_DIR=~/.some-notes/ ludv-note -e pacman";

fn main() {
    let notes_path = env::var("LUDV_NOTES_DIR")
        .unwrap_or_else(|_| String::from(DEFAULT_NOTE_DIR));
    let notes_path = expanduser(notes_path)
        .unwrap_or_else(|err| exit_with_error(format!("failed to expand home dir tilda: {}", err.to_string())));

    if !notes_path.exists() {
        fs::create_dir_all(&notes_path)
            .expect("notes dir does not exist, and failed to create it");
    }

    let mut args = env::args();
    args.next();

    let config = Config::build(&mut args, notes_path)
        .unwrap_or_else(|_| exit_with_help());

    let res = match config.action {
        Action::Present(ref note_id) => actions::present(&config, note_id),
        Action::Edit(ref note_id) => actions::edit(&config, note_id),
        Action::Delete(ref note_id) => actions::delete(&config, note_id),
        Action::List => actions::list(&config),
        Action::Find(ref pattern) => actions::find(&config, pattern),
        Action::Help => exit_with_help()
    };

    res.unwrap_or_else(|err| exit_with_error(err));
}

fn exit_with_help() -> ! {
    println!("{}", HELP_USAGE);
    process::exit(0)
}

fn exit_with_error(err_msg: impl Display) -> ! {
    eprintln!("{}", err_msg);
    process::exit(1)
}
