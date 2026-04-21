/// Path resolution utilities for handling relative and absolute paths.
use std::path::{Path, PathBuf};

/// Parse a command line string, respecting quoted arguments.
///
/// # Arguments
/// * `input` - The command line string to parse
///
/// # Returns
/// A vector of parsed arguments with quotes removed
///
/// # Examples
/// ```
/// use steam_redirect::path::parse_command_line;
///
/// let result = parse_command_line("./game.exe arg1 arg2");
/// assert_eq!(result, vec!["./game.exe", "arg1", "arg2"]);
///
/// let quoted = parse_command_line("\"C:\\Program Files\\Game\\game.exe\" arg1");
/// assert_eq!(quoted, vec!["C:\\Program Files\\Game\\game.exe", "arg1"]);
/// ```
pub fn parse_command_line(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' | '\t' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

/// Resolve a path, handling relative paths and making them absolute.
///
/// # Arguments
/// * `path` - The path string to resolve
/// * `base_dir` - The base directory for resolving relative paths
///
/// # Returns
/// The resolved path as a String
///
/// # Behavior
/// - Absolute paths are returned as-is
/// - Windows-style absolute paths are preserved on all platforms
/// - Relative paths starting with `./` or `../` are resolved relative to base_dir
/// - Other relative paths are resolved relative to base_dir
pub fn resolve_path(path: &str, base_dir: &Path) -> String {
    let path_buf = PathBuf::from(path);

    if path_buf.is_absolute() || is_windows_absolute_path(path) {
        return path.to_string();
    }

    if path.starts_with("./") {
        let trimmed = &path[2..];
        let resolved = base_dir.join(trimmed);
        if let Ok(canonical) = resolved.canonicalize() {
            return canonical.to_string_lossy().into_owned();
        }
        return resolved.to_string_lossy().into_owned();
    }

    let resolved = base_dir.join(&path_buf);
    if path.starts_with("../") {
        if let Ok(canonical) = resolved.canonicalize() {
            return canonical.to_string_lossy().into_owned();
        }
    }

    resolved.to_string_lossy().into_owned()
}

fn is_windows_absolute_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    if bytes.len() < 3 {
        return false;
    }

    let drive_letter = bytes[0];
    let colon = bytes[1];
    let sep = bytes[2];

    drive_letter.is_ascii_alphabetic() && colon == b':' && (sep == b'\\' || sep == b'/')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_command_line_simple() {
        let result = parse_command_line("./game.exe");
        assert_eq!(result, vec!["./game.exe"]);
    }

    #[test]
    fn test_parse_command_line_with_args() {
        let result = parse_command_line("./game.exe arg1 arg2");
        assert_eq!(result, vec!["./game.exe", "arg1", "arg2"]);
    }

    #[test]
    fn test_parse_command_line_quoted_path_with_spaces() {
        let result = parse_command_line("\"C:\\Program Files\\Game\\game.exe\" arg1");
        assert_eq!(result, vec!["C:\\Program Files\\Game\\game.exe", "arg1"]);
    }

    #[test]
    fn test_parse_command_line_quoted_with_multiple_spaces() {
        let result = parse_command_line(
            "\"C:\\Program Files\\My Mod Manager\\ModOrganizer.exe\" -profile MyProfile",
        );
        assert_eq!(
            result,
            vec![
                "C:\\Program Files\\My Mod Manager\\ModOrganizer.exe",
                "-profile",
                "MyProfile"
            ]
        );
    }

    #[test]
    fn test_resolve_absolute_unix_path() {
        let result = resolve_path("/usr/bin/game.exe", Path::new("."));
        assert_eq!(result, "/usr/bin/game.exe");
    }

    #[test]
    fn test_resolve_absolute_windows_path() {
        let result = resolve_path("C:\\Program Files\\Game\\game.exe", Path::new("."));
        assert_eq!(result, "C:\\Program Files\\Game\\game.exe");
    }
}
