use grep::{
    searcher::{Searcher, sinks::UTF8},
    regex::RegexMatcher
};
use ludv_note::{config::Config, iterate_notes};

pub fn find_contents(config: &Config, pattern: &str) {
    let matcher = RegexMatcher::new(pattern).unwrap();

    iterate_notes(config, |entry, id| {
        let entry_path = entry.path();

        let mut results = vec![];
        Searcher::new().search_path(&matcher, entry_path, UTF8(|lnum, line| {
            results.push((lnum, line.trim_end().to_string()));
            Ok(true)
        })).unwrap();

        if !results.is_empty() {
            println!("  {}", id);

            for (lnum, line) in results {
                println!("{}:{}", lnum, line);
            }

            println!();
        }
    });
}

