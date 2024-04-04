use ludv_note::{config::Config, iterate_notes};

pub fn list(config: &Config) -> Result<(), String> {
    iterate_notes(config, |_entry, id| {
        println!("{}", id);
    });

    Ok(())
}

