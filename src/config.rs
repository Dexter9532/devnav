use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub path: PathBuf,
}

fn config_path() -> Result<PathBuf> {
    let directory = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or(home_dir()?.join(".config"));

    Ok(directory.join("devnav/config.toml"))
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        toml::from_str(&contents)
            .with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn save(&self) -> Result<PathBuf> {
        let path = config_path()?;
        let parent = path.parent().context("configuration path has no parent")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;

        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)
            .with_context(|| format!("failed to write {}", path.display()))?;
        Ok(path)
    }
}

pub fn home_dir() -> Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set")
}

pub fn resolve_path(value: &str) -> Result<PathBuf> {
    let path = if value == "~" {
        home_dir()?
    } else if let Some(rest) = value.strip_prefix("~/") {
        home_dir()?.join(rest)
    } else {
        PathBuf::from(value)
    };

    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

pub fn display_path(path: &Path) -> String {
    if let Ok(home) = home_dir()
        && let Ok(relative) = path.strip_prefix(home)
    {
        if relative.as_os_str().is_empty() {
            return "~".to_owned();
        }
        return format!("~/{}", relative.display());
    }

    path.display().to_string()
}

pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("location name cannot be empty");
    }

    if !name
        .chars()
        .all(|character| character.is_alphanumeric() || matches!(character, '-' | '_'))
    {
        bail!("location name may only contain letters, numbers, '-' and '_'");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_name;

    #[test]
    fn accepts_valid_location_names() {
        assert!(validate_name("projects").is_ok());
        assert!(validate_name("client-work_2").is_ok());
    }

    #[test]
    fn rejects_empty_or_unsafe_location_names() {
        assert!(validate_name("").is_err());
        assert!(validate_name("client work").is_err());
        assert!(validate_name("../projects").is_err());
    }
}
