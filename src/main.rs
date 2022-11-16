use serde::Deserialize;
use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};
use std::{
    borrow::Cow,
    error::Error,
    fs::{read_dir, read_to_string},
    iter::once,
    path::Path,
    sync::Arc,
};

const APP_NAME: &str = "commando";

#[derive(Deserialize, Debug)]
struct Config {
    commands: Vec<Command>,
}

#[derive(Deserialize, Debug, Clone)]
struct Command {
    name: String,
    command: String,
}

impl SkimItem for Command {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.command)
    }
}

fn load_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    let data = read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}

fn get_config_files() -> Result<Vec<Box<Path>>, Box<dyn Error>> {
    let home = xdg::BaseDirectories::with_prefix(APP_NAME)
        .or(Err("Config directory not found in config home"))?
        .get_config_home();
    let dirs = xdg::BaseDirectories::with_prefix(APP_NAME)
        .or(Err("Config directory not found in config dirs"))?
        .get_config_dirs();

    Ok(dirs
        .into_iter()
        .chain(once(home))
        .filter_map(|dir| {
            let command_dir = dir.join("commandos");
            if command_dir.exists() {
                Some(command_dir)
            } else {
                None
            }
        })
        .filter_map(|command_dir| {
            let files = read_dir(command_dir).ok()?;
            Some(files.filter_map(|object| Some(object.ok()?.path().into_boxed_path())))
        })
        .flatten()
        .collect())
}

fn select_item(commands: &[Command]) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .build()
        .expect("Could not build skim");

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for command in commands {
        tx.send(Arc::new(command.clone()))
            .expect("Could not add command");
    }
    drop(tx);

    let result = Skim::run_with(&options, Some(rx))?;

    if result.is_abort {
        None
    } else {
        Some((*result.selected_items[0].output()).to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_files = get_config_files().expect("Could not find config files");
    let commands: Vec<Command> = config_files
        .iter()
        .filter_map(|path| Some(load_config(path).ok()?.commands))
        .flatten()
        .collect();
    let output = select_item(&commands);

    let config_command = output.unwrap_or_else(|| panic!("No action selected"));
    let split_command = shlex::split(&config_command).unwrap_or_else(|| panic!("No valid command"));
    let mut split_iter = split_command.into_iter();
    let command_string = split_iter
        .next()
        .unwrap_or_else(|| panic!("No valid command"));

    let mut child = std::process::Command::new(command_string)
        .args(split_iter)
        .spawn()?;

    child.wait()?;

    Ok(())
}
