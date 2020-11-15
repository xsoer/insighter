use clap::{load_yaml, App};
use std::collections::HashMap;

pub fn cli_options() -> HashMap<String, String> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let mut options = HashMap::new();

    options.insert(
        "dir".to_string(),
        matches.value_of("dir").unwrap_or("./").to_string(),
    );
    options.insert(
        "output".to_string(),
        matches
            .value_of("output")
            .unwrap_or("outlines.md")
            .to_string(),
    );
    options
}
