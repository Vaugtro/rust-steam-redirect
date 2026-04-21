/// Configuration file handling.
use crate::error::Result;
use crate::error::WrapperError;
use crate::path::{parse_command_line, resolve_path};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_SUBDIR: &str = "redirect";
const CONFIG_FILENAME: &str = "config.cfg";

/// Find the config file by searching in the current directory and parent directories.
///
/// # Arguments
/// * `start_path` - The directory to start searching from
///
/// # Returns
/// The path to the config file if found, None otherwise
pub fn find_config_file(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path.to_path_buf();

    loop {
        let config_path = current.join(CONFIG_SUBDIR).join(CONFIG_FILENAME);
        if config_path.exists() {
            return Some(config_path);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

/// Parse the program entry from config file content.
///
/// # Arguments
/// * `config_content` - The content of the config file
/// * `base_dir` - The directory containing the config file (for path resolution)
///
/// # Returns
/// A vector of program and arguments, or an error if not found
///
/// # Config Format
/// Lines starting with `#` or `;` are comments.
/// The first line starting with `program=` is parsed.
/// Quoted arguments are handled correctly.
pub fn parse_program_entry(config_content: &str, base_dir: &Path) -> Result<Vec<String>> {
    for line in config_content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if let Some(value) = line.strip_prefix("program=") {
            // Parse the command line, handling quoted strings
            let args = parse_command_line(value);

            // Resolve the first argument (the program path)
            if let Some(program) = args.first() {
                let mut program_args = vec![resolve_path(program, base_dir)];

                // Add remaining arguments
                program_args.extend(args.iter().skip(1).cloned());

                return Ok(program_args);
            }

            return Err(WrapperError::ProgramNotFound);
        }
    }

    Err(WrapperError::ProgramNotFound)
}

/// Configuration loaded from the config file.
#[derive(Debug)]
pub struct WrapperConfig {
    pub program_args: Vec<String>,
    pub fallback_args: Option<Vec<String>>,
}

/// Load and parse the configuration from the config file.
///
/// # Arguments
/// * `exe_dir` - The directory containing the wrapper executable
///
/// # Returns
/// A tuple of (config_directory, wrapper_config) or an error
pub fn load_config(exe_dir: &Path) -> Result<(PathBuf, WrapperConfig)> {
    let config_file = find_config_file(exe_dir).ok_or(WrapperError::ConfigNotFound)?;

    let config_dir = config_file.parent().unwrap_or_else(|| Path::new("."));

    let config_content = fs::read_to_string(&config_file)?;
    let program_args = parse_program_entry(&config_content, config_dir)?;
    let fallback_args = parse_fallback_entry(&config_content, config_dir)?;

    Ok((
        config_dir.to_path_buf(),
        WrapperConfig {
            program_args,
            fallback_args,
        },
    ))
}

fn parse_fallback_entry(config_content: &str, base_dir: &Path) -> Result<Option<Vec<String>>> {
    for line in config_content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if let Some(value) = line.strip_prefix("fallback=") {
            let args = parse_command_line(value);

            if let Some(program) = args.first() {
                let mut fallback_args = vec![resolve_path(program, base_dir)];
                fallback_args.extend(args.iter().skip(1).cloned());
                return Ok(Some(fallback_args));
            }

            return Err(WrapperError::ProgramNotFound);
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program_entry_simple() {
        let config = "program=./game.exe";
        let result = parse_program_entry(config, Path::new("."));
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 1);
    }

    #[test]
    fn test_parse_program_entry_with_args() {
        let config = "program=./game.exe arg1 arg2";
        let result = parse_program_entry(config, Path::new("."));
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 3);
    }

    #[test]
    fn test_parse_program_entry_with_quotes() {
        let config = "program=\"./my game/game.exe\" arg1";
        let result = parse_program_entry(config, Path::new("."));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_program_entry_skips_comments() {
        let config = r#"# This is a comment
; This is also a comment
program=./game.exe"#;
        let result = parse_program_entry(config, Path::new("."));
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 1);
    }

    #[test]
    fn test_parse_program_entry_not_found() {
        let config = "# Just a comment";
        let result = parse_program_entry(config, Path::new("."));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fallback_entry_simple() {
        let config = "fallback=./_SkyrimSELauncher.exe";
        let result = parse_fallback_entry(config, Path::new("."));
        assert!(result.is_ok());
        let fallback = result.unwrap();
        assert_eq!(fallback.unwrap(), vec!["./_SkyrimSELauncher.exe"]);
    }

    #[test]
    fn test_parse_fallback_entry_with_args() {
        let config = "fallback=./_SkyrimSELauncher.exe --skipintro";
        let result = parse_fallback_entry(config, Path::new("."));
        assert!(result.is_ok());
        let fallback = result.unwrap();
        assert_eq!(fallback.unwrap(), vec!["./_SkyrimSELauncher.exe", "--skipintro"]);
    }

    #[test]
    fn test_parse_fallback_entry_not_present() {
        let config = "program=./game.exe";
        let result = parse_fallback_entry(config, Path::new("."));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
