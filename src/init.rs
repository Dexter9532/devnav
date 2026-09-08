use std::fs;
use std::io::{self, Write};

use anyhow::{Result, bail};

use crate::config::{Config, Location, display_path, resolve_path, validate_name};

pub fn run() -> Result<()> {
    let mut config = Config::load()?;

    if config.locations.is_empty() {
        println!("Set up DevNav");
        add_location(&mut config, true)?;
        let path = config.save()?;
        println!("Configuration saved to {}", path.display());
        return Ok(());
    }

    manage_locations(&mut config)
}

fn manage_locations(config: &mut Config) -> Result<()> {
    loop {
        print_locations(config);
        println!();
        println!("[a] Add  [r] Rename  [p] Change path  [d] Delete  [q] Done");

        match prompt("Choose an action", Some("q"))?.to_lowercase().as_str() {
            "a" | "add" => add_location(config, false)?,
            "r" | "rename" => rename_location(config)?,
            "p" | "path" => change_path(config)?,
            "d" | "delete" => delete_location(config)?,
            "q" | "done" => {
                let path = config.save()?;
                println!("Configuration saved to {}", path.display());
                return Ok(());
            }
            _ => println!("Unknown action"),
        }
    }
}

fn add_location(config: &mut Config, first: bool) -> Result<()> {
    let default_name = first.then_some("projects");
    let name = prompt("Location name", default_name)?;
    validate_name(&name)?;

    if config.locations.iter().any(|location| location.name == name) {
        bail!("a location named '{name}' already exists");
    }

    let default_path = first.then_some("~/dev/projects");
    let path = resolve_path(&prompt("Projects directory", default_path)?)?;
    create_directory(&path)?;

    config.locations.push(Location { name, path });
    Ok(())
}

fn rename_location(config: &mut Config) -> Result<()> {
    let Some(index) = choose_location(config)? else {
        return Ok(());
    };

    let name = prompt("New name", Some(&config.locations[index].name))?;
    validate_name(&name)?;

    if config
        .locations
        .iter()
        .enumerate()
        .any(|(other, location)| other != index && location.name == name)
    {
        bail!("a location named '{name}' already exists");
    }

    config.locations[index].name = name;
    Ok(())
}

fn change_path(config: &mut Config) -> Result<()> {
    let Some(index) = choose_location(config)? else {
        return Ok(());
    };

    let current = display_path(&config.locations[index].path);
    let path = resolve_path(&prompt("New path", Some(&current))?)?;
    create_directory(&path)?;
    config.locations[index].path = path;
    Ok(())
}

fn delete_location(config: &mut Config) -> Result<()> {
    let Some(index) = choose_location(config)? else {
        return Ok(());
    };

    let location = &config.locations[index];
    let question = format!(
        "Remove '{}' from DevNav? Files will not be deleted",
        location.name
    );

    if confirm(&question, false)? {
        config.locations.remove(index);
    }

    Ok(())
}

fn create_directory(path: &std::path::Path) -> Result<()> {
    if path.exists() {
        if !path.is_dir() {
            bail!("{} is not a directory", path.display());
        }
        return Ok(());
    }

    let question = format!("Create {}?", display_path(path));
    if confirm(&question, true)? {
        fs::create_dir_all(path)?;
        Ok(())
    } else {
        bail!("directory does not exist");
    }
}

fn choose_location(config: &Config) -> Result<Option<usize>> {
    print_locations(config);
    let answer = prompt("Location number (blank to cancel)", None)?;
    if answer.is_empty() {
        return Ok(None);
    }

    let number: usize = answer.parse()?;
    if number == 0 || number > config.locations.len() {
        bail!("invalid location number");
    }

    Ok(Some(number - 1))
}

fn print_locations(config: &Config) {
    println!();
    println!("Configured locations");
    for (index, location) in config.locations.iter().enumerate() {
        println!(
            "{}. {}  {}",
            index + 1,
            location.name,
            display_path(&location.path)
        );
    }
}

fn prompt(label: &str, default: Option<&str>) -> Result<String> {
    match default {
        Some(value) => print!("{label} [{value}]: "),
        None => print!("{label}: "),
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        Ok(default.unwrap_or_default().to_owned())
    } else {
        Ok(input.to_owned())
    }
}

fn confirm(question: &str, default: bool) -> Result<bool> {
    let hint = if default { "Y/n" } else { "y/N" };
    let answer = prompt(question, Some(hint))?;

    if answer.eq_ignore_ascii_case(hint) {
        return Ok(default);
    }

    match answer.to_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => bail!("please answer yes or no"),
    }
}
