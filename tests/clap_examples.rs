use clap::{error::ErrorKind, Parser};
use rust_study::cli::Cli;

#[test]
fn accepts_composed_commands() {
    for args in [
        vec![
            "demo", "export", "--tag", "rust", "--tag", "cli", "-ddd", "--format", "json",
        ],
        vec!["demo", "ex", "--format", "text"],
        vec!["demo", "config", "show", "--format", "json"],
        vec!["demo", "config", "set", "theme", "dark"],
        vec!["demo", "connect", "--anonymous", "--port", "9000"],
        vec![
            "demo",
            "connect",
            "--user",
            "alice",
            "--password",
            "example",
        ],
    ] {
        assert!(Cli::try_parse_from(&args).is_ok(), "{args:?}");
    }
}

#[test]
fn rejects_invalid_enum_and_port_values() {
    for args in [
        vec!["demo", "export", "--format", "xml"],
        vec!["demo", "connect", "--anonymous", "--port", "0"],
        vec!["demo", "connect", "--anonymous", "--port", "65536"],
        vec!["demo", "connect", "--anonymous", "--port", "abc"],
    ] {
        assert!(Cli::try_parse_from(&args).is_err(), "{args:?}");
    }
}

#[test]
fn requires_authentication_choice_and_password() {
    for args in [
        vec!["demo", "connect"],
        vec!["demo", "connect", "--user", "alice"],
        vec!["demo", "connect", "--password", "example"],
    ] {
        assert_eq!(
            Cli::try_parse_from(args).unwrap_err().kind(),
            ErrorKind::MissingRequiredArgument
        );
    }
}

#[test]
fn rejects_conflicting_authentication_options() {
    for args in [
        vec![
            "demo",
            "connect",
            "--anonymous",
            "--user",
            "alice",
            "--password",
            "example",
        ],
        vec!["demo", "connect", "--anonymous", "--password", "example"],
    ] {
        assert_eq!(
            Cli::try_parse_from(args).unwrap_err().kind(),
            ErrorKind::ArgumentConflict
        );
    }
}
