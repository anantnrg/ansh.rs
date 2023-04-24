use colored::Colorize;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use toml;

pub fn parse_config() {
    let config_path = get_user_config_path();
    let conf_exists = ensure_user_config_exists(&config_path);

    #[derive(Deserialize)]
    struct Config {
        prompt: Prompt,
    }

    #[derive(Deserialize)]
    struct Prompt {
        prompt_1: String,
        prompt_2: String,
    }

    if conf_exists == true {
        let contents = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", config_path);
                exit(1);
            }
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", config_path);
                exit(1);
            }
        };

        // Print out the values to `stdout`.
        println!("{}", config.prompt.prompt_1);
        println!("{}", config.prompt.prompt_2);
    }
}

fn get_user_config_path() -> String {
    let config_name = "/.config/ansh/config.toml".to_string();
    let mut config = "".to_string();

    let home_dir = env::var("HOME")
        .expect("Cannot get HOME directory.")
        .to_string();
    config = format!("{}{}", home_dir, config_name);
    println!("{}", config);
    return config;
}

fn ensure_user_config_exists(config_path: &String) -> bool {
    let _conf_does_exist: bool = Path::new(&config_path).exists();
    if _conf_does_exist == true {
        println!("Using user config which exists at '{}'", config_path);
        println!("{}", "User config found at ~/.config/ansh/config.toml\nTo see all the wonderful things you can do with ANsh, visit 'https://github.com/anantnrg/ansh.rs'. To disable this message, change the value of 'show_welcome_message' to 'false' in the config. ");
        return true;
    } else {
        println!("{}", "ERROR: User config does not exist. User config not found in '~/.config/ansh/config.toml'.".bold().red());
        println!("{}", "Creating new config with default settings at ~/.config/ansh/config.toml\n\nConfig will be loaded when ANSH is started next time.".bold().green());
        let _ = File::create(config_path);
        return false;
    }
}
