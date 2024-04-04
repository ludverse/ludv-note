use std::path::PathBuf;

use crate::Action;

#[derive(Default)]
struct ConfigBuilder {
    pub action: Option<Action>,
    pub find_contents: bool
}

pub struct Config {
    pub notes_path: PathBuf,
    pub action: Action,
    pub find_contents: bool
}

impl Config {
    pub fn build(args: &mut impl Iterator<Item = String>, notes_path: PathBuf) -> Result<Self, ()> {
        let mut builder = ConfigBuilder::default();

        let mut next_arg = args.next().ok_or(())?;
        loop {
            match &next_arg[..] {
                "-c" | "--find-contents" => builder.find_contents = true,
                _ => break
            }

            next_arg = args.next().ok_or(())?;
        }

        match &next_arg[..] {
            "-e" | "--edit" => {
                let next_arg = args.next().ok_or(())?;
                builder.action = Some(Action::Edit(next_arg));
            },
            "-d" | "--delete" => {
                let next_arg = args.next().ok_or(())?;
                builder.action = Some(Action::Delete(next_arg));
            },
            "-l" | "--list" => builder.action = Some(Action::List),
            "-f" | "--find" => {
                let next_arg = args.next().ok_or(())?;
                builder.action = Some(Action::Find(next_arg));
            },
            "-h" | "--help" => builder.action = Some(Action::Help),
            _ => builder.action = Some(Action::Present(next_arg))
        }

        Ok(Self {
            notes_path,
            action: builder.action.unwrap(),
            find_contents: builder.find_contents
        })
    }
}
