# DevNav Agent Guide

## Project

DevNav is a Rust CLI for creating, organizing, and navigating development
projects. It is intentionally small and should remain simple to install,
understand, and remove.

DevNav is vibe-coded with AI assistance. Treat existing code as code that must
be reviewed and tested, not as an automatically correct specification.

## Current scope

The first implementation milestone is `devnav init`. It manages named project
locations and stores them in:

```text
~/.config/devnav/config.toml
```

The default location is named `projects` and points to `~/dev/projects`.
Running `devnav init` again must allow users to add, rename, repoint, or remove
configured locations.

Removing a location from DevNav must never remove its directory or files.

## Planned interface

```text
devnav init
devnav update
devnav <project>
devnav list
devnav create <name>
devnav create --github <repository>
```

Navigation will require minimal shell integration because a child process
cannot change its parent shell's working directory. The core behavior must stay
in the DevNav binary, with support planned for Bash, Fish, and Zsh.

## Code structure

- `src/main.rs`: command-line parsing and command dispatch.
- `src/config.rs`: configuration loading, saving, and path handling.
- `src/init.rs`: interactive location setup and management.
- `src/update.rs`: updates the installed executable through Cargo.

Keep modules focused and avoid adding abstractions until they remove concrete
duplication or complexity.

## Development

Use the stable Rust toolchain.

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Tests that read or write configuration must use a temporary `HOME` or
`XDG_CONFIG_HOME` and must not modify the developer's real configuration.

## Conventions

- Use `clap` for command-line parsing.
- Use TOML for persistent configuration.
- Return actionable errors instead of silently ignoring invalid state.
- Preserve `-h` and `--help` for command help.
- Do not delete user files or project directories.
- Keep output readable in terminals and usable without color.
- Keep platform-specific shell behavior outside the core project logic.
