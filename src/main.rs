use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

const FONT_WITHOUT_SIZE: &str = "jetbrains mono";
const FONT_WITH_SIZE: &str = "jetbrainsmono 9";

fn main() {
    run();
}

#[derive(Deserialize, Debug)]
struct Settings {
    app: String,
    src: PathBuf,
    key: String,
    with_size: bool,
}

fn read_setting() -> Vec<Settings> {
    if let Ok(y) = read_to_string("setting.yaml") {
        serde_yaml::from_str(&y).unwrap()
    } else {
        vec![]
    }
}

fn run() {
    let setting = read_setting();
    for s in setting {
        println!("{}", change_one_line(s));
    }
}

fn change_one_line(s: Settings) -> String {
    if let Ok(config) = read_to_string(s.src) {
        let mut new = String::new();
        for line in config.lines() {
            if line.starts_with(&s.key) {
                let mut new_line = s.key.clone();
                if s.with_size {
                    new_line.push_str(FONT_WITH_SIZE);
                } else {
                    new_line.push_str(FONT_WITHOUT_SIZE);
                }
                new.push_str(&new_line);
                new.push('\n');
            } else {
                new.push_str(line);
                new.push('\n');
            }
        }
        new
    } else {
        String::new()
    }
}
