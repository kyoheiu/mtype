# mtype
Simple app to manage font setting by replacing config files.

## Usage

1. Create `setting.yaml` in `~/.config/mtype`.

```yaml
# example

- app: vs code # Name to display in the log.
  src: /home/kyohei/.config/Code/User/settings.json # Path to the config file.
  key: '  "editor.fontFamily": ' # Words to detect the font-setting part (should start with these words).
  txt: '  "editor.fontFamily": "Berkeley Mono",' # Whole text for replacement. Multiple lines can also be used.
```

2. Update `txt` lines in `setting.yaml`.
3. `git clone` this repo and `cargo run`. Or, `cargo install --path .` and `mtype`.  

At this moment, this program covers only apps whose config file is stored locally (i.e. `~/.config/app`).