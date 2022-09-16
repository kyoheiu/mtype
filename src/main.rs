use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

fn main() {
    run();
}

#[derive(Deserialize, Debug)]
struct Settings {
    app: String,
    src: PathBuf,
    key: String,
    txt: String,
    number: Option<usize>,
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
        change_one_line(s);
    }
}

fn change_one_line(s: Settings) {
    if let Ok(config) = read_to_string(&s.src) {
        let mut new = String::new();
        match s.number {
            None => {
                for line in config.lines() {
                    if line.starts_with(&s.key) {
                        new.push_str(&s.txt);
                        new.push('\n');
                    } else {
                        new.push_str(line);
                        new.push('\n');
                    }
                }
            }
            Some(i) => {
                let mut count = 0;
                for line in config.lines() {
                    if count > 0 {
                        count -= 1;
                        continue;
                    }
                    if line.starts_with(&s.key) {
                        new.push_str(&s.txt);
                        new.push('\n');
                        count = i;
                    } else {
                        new.push_str(line);
                        new.push('\n');
                    }
                }
            }
        }
        if std::fs::write(s.src, new).is_ok() {
            println!("New config created for {}.", s.app);
        };
    } else {
    }
}
