use regex::Regex;
use ludv_note::{config::Config, iterate_notes};

use crate::grep;

pub fn find(config: &Config, pattern: &String) -> Result<(), String> {
    if config.find_contents && cfg!(not(feature = "grep")) {
        return Err("you have not enabled the feature flag `grep` but still you tried to use the --find-contents option. to use it, you need to install this binary using `cargo install ludv-note --features grep`.".to_string());
    }

    if config.find_contents {
        grep::find_contents(config, pattern);
    } else {
        let pattern = Regex::new(pattern).unwrap();

        iterate_notes(config, |_entry, id| {
            if pattern.is_match(&id) {
                println!("{}", id);
            }
        });
    }

    Ok(())
}

