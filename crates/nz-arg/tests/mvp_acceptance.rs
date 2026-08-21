//! `nz-arg` 验收测试（对照 `spec/nz-arg/mvp.md`）。

use nz_arg::{ArgSchema, ArgSpec, ParseError, ParseMode, ParseOutcome, parse};
use std::io::Write;

fn sample_schema() -> ArgSchema {
    ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('t', "tools", "list tools"),
        ArgSpec::optional_bool('a', "alpha", "alpha flag"),
        ArgSpec::optional_bool('b', "beta", "beta flag"),
        ArgSpec::optional_string('d', "device", "network device", None::<String>),
        ArgSpec::optional_string('n', "name", "a name", None::<String>),
        ArgSpec::optional_u32('u', "uint", "an integer", None),
        ArgSpec::required_string('s', "server", "server name"),
        ArgSpec::more("files", "extra files"),
    ])
    .expect("schema")
}

fn parsed(args: &[&str]) -> nz_arg::ParsedArgs {
    match parse(&sample_schema(), args, ParseMode::Cli).expect("parse") {
        ParseOutcome::Parsed(values) => values,
        ParseOutcome::Help { .. } => panic!("expected parsed, got help"),
    }
}

#[test]
fn schema_rejects_duplicate_keys() {
    let error = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('t', "tools", "a"),
        ArgSpec::optional_bool('t', "other", "b"),
    ])
    .expect_err("dup");
    assert!(matches!(error, ParseError::InvalidSchema(_)));
}

#[test]
fn schema_rejects_reserved_long_names() {
    let error = ArgSchema::try_from_specs(vec![ArgSpec::optional_bool('h', "help", "no")])
        .expect_err("reserved");
    assert!(matches!(error, ParseError::InvalidSchema(_)));
}

#[test]
fn parse_long_string_and_u32() {
    let values = parsed(&["--name", "alice", "-u", "42", "-s", "srv"]);
    assert_eq!(values.get_string('n'), Some("alice"));
    assert_eq!(values.get_u32('u'), Some(42));
    assert!(values.isset('n'));
}

#[test]
fn bool_triple_short_long() {
    let on = parsed(&["-t", "-s", "x"]);
    assert_eq!(on.get_bool('t'), Some(true));
    assert!(on.isset('t'));

    let off_plus = parsed(&["+t", "-s", "x"]);
    assert_eq!(off_plus.get_bool('t'), Some(false));
    assert!(off_plus.isset('t'));

    let on_long = parsed(&["--tools", "-s", "x"]);
    assert_eq!(on_long.get_bool('t'), Some(true));

    let off_long = parsed(&["--no-tools", "-s", "x"]);
    assert_eq!(off_long.get_bool('t'), Some(false));
    assert!(off_long.isset('t'));
}

#[test]
fn bool_cluster_short() {
    let values = parsed(&["-ab", "-s", "x"]);
    assert_eq!(values.get_bool('a'), Some(true));
    assert_eq!(values.get_bool('b'), Some(true));

    let off = parsed(&["+ab", "-s", "x"]);
    assert_eq!(off.get_bool('a'), Some(false));
    assert_eq!(off.get_bool('b'), Some(false));
}

#[test]
fn long_name_unique_prefix() {
    let values = parsed(&["--dev", "eth0", "-s", "x"]);
    assert_eq!(values.get_string('d'), Some("eth0"));
}

#[test]
fn long_name_ambiguous_prefix_errors() {
    let schema = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_string('d', "device", "dev", None::<String>),
        ArgSpec::optional_string('e', "devil", "evil", None::<String>),
        ArgSpec::optional_bool('x', "extra", "x"),
    ])
    .expect("schema");
    let error = parse(&schema, &["--de"], ParseMode::Cli).expect_err("ambig");
    assert!(matches!(error, ParseError::AmbiguousPrefix { .. }));
}

#[test]
fn help_flags_basic() {
    for flag in ["--help", "--?"] {
        let outcome = parse(&sample_schema(), &[flag], ParseMode::Cli).expect("help");
        assert_eq!(
            outcome,
            ParseOutcome::Help {
                include_advanced: false
            }
        );
    }
}

#[test]
fn help_flags_advanced() {
    for flag in ["--help2", "--??"] {
        let outcome = parse(&sample_schema(), &[flag], ParseMode::Cli).expect("help2");
        assert_eq!(
            outcome,
            ParseOutcome::Help {
                include_advanced: true
            }
        );
    }
}

#[test]
fn formupdate_ignores_help() {
    let error = parse(&sample_schema(), &["--help"], ParseMode::FormUpdate).expect_err("no help");
    assert!(matches!(error, ParseError::UnknownOption(_)));
}

#[test]
fn formupdate_ignores_argfile() {
    let error =
        parse(&sample_schema(), &["--argfile", "x"], ParseMode::FormUpdate).expect_err("no file");
    assert!(matches!(error, ParseError::UnknownOption(_)));
}

#[test]
fn required_missing_errors() {
    let error = parse(&sample_schema(), &["-t"], ParseMode::Cli).expect_err("req");
    assert!(matches!(
        error,
        ParseError::MissingRequired { key: 's', .. }
    ));
}

#[test]
fn unknown_option_errors() {
    assert!(matches!(
        parse(&sample_schema(), &["-z", "-s", "x"], ParseMode::Cli).expect_err("z"),
        ParseError::UnknownOption(_)
    ));
    assert!(matches!(
        parse(&sample_schema(), &["--zzz", "-s", "x"], ParseMode::Cli).expect_err("zzz"),
        ParseError::UnknownOption(_)
    ));
}

#[test]
fn value_option_missing_value_errors() {
    let error = parse(&sample_schema(), &["-u"], ParseMode::Cli).expect_err("missing");
    assert!(matches!(error, ParseError::MissingValue(_)));
}

#[test]
fn more_after_double_dash() {
    let values = parsed(&["-s", "srv", "--", "a", "b"]);
    assert_eq!(values.more(), &["a".to_owned(), "b".to_owned()]);
}

#[test]
fn more_after_required_satisfied() {
    let values = parsed(&["-s", "srv", "file1", "file2"]);
    assert_eq!(values.more(), &["file1".to_owned(), "file2".to_owned()]);
}

#[test]
fn positional_fills_required() {
    let values = parsed(&["myserver"]);
    assert_eq!(values.get_string('s'), Some("myserver"));
    assert!(values.isset('s'));
}

#[test]
fn argfile_loads_and_merges() {
    let directory = tempfile_dir();
    let path = directory.join("args.txt");
    let mut file = std::fs::File::create(&path).expect("create");
    writeln!(file, "# comment").expect("write");
    writeln!(file).expect("blank");
    writeln!(file, "-t --name alice").expect("write");
    writeln!(file, "-s srv").expect("write");
    drop(file);

    let path_str = path.to_str().expect("utf8");
    let values = parsed(&["--argfile", path_str]);
    assert_eq!(values.get_bool('t'), Some(true));
    assert_eq!(values.get_string('n'), Some("alice"));
    assert_eq!(values.get_string('s'), Some("srv"));
}

#[test]
fn argfile_quoted_tokens() {
    let directory = tempfile_dir();
    let path = directory.join("q.txt");
    std::fs::write(&path, r#"-n "hello world" -s srv"#).expect("write");
    let values = parsed(&["--argfile", path.to_str().expect("utf8")]);
    assert_eq!(values.get_string('n'), Some("hello world"));
}

#[test]
fn kbd_not_supported_errors() {
    let error = parse(&sample_schema(), &["--kbd"], ParseMode::Cli).expect_err("kbd");
    assert_eq!(error, ParseError::InteractiveNotSupported);
}

fn tempfile_dir() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "nz-arg-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    std::fs::create_dir_all(&path).expect("mkdir");
    path
}
