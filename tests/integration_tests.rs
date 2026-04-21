use steam_redirect::config::parse_program_entry;
use steam_redirect::path::parse_command_line;
use std::path::Path;

#[test]
fn test_parse_program_entry_integration() {
    let config = r#"# Configuration for my game
program=./launcher.exe --config game.ini --verbose"#;

    let result = parse_program_entry(config, Path::new("."));
    assert!(result.is_ok());

    let args = result.unwrap();
    assert!(args.len() >= 3);
}

#[test]
fn test_parse_with_quoted_mod_organizer() {
    let config = r#"program="C:\Program Files\ModOrganizer2\ModOrganizer.exe" -profile Skyrim"#;

    let result = parse_program_entry(config, Path::new("."));
    assert!(result.is_ok());

    let args = result.unwrap();
    assert!(args[1] == "-profile");
    assert!(args[2] == "Skyrim");
}

#[test]
fn test_parse_command_line_edge_cases() {
    // Multiple spaces
    let result = parse_command_line("program   arg1   arg2");
    assert_eq!(result.len(), 3);

    // Trailing spaces
    let result = parse_command_line("program arg1   ");
    assert_eq!(result.len(), 2);

    // Empty quotes (quotes are removed but empty string is not added)
    let result = parse_command_line("program \"\" arg2");
    assert_eq!(result.len(), 2);
}
