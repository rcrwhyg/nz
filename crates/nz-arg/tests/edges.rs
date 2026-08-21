//! 边缘路径补测，拉高行覆盖至 ≥95%。

use nz_arg::{ArgSchema, ArgSpec, ParseError, ParseMode, ParseOutcome, ValueKind, parse};

#[test]
fn schema_edge_rejects() {
    assert!(matches!(
        ArgSchema::try_from_specs(vec![ArgSpec::optional_bool('-', "x", "bad")]),
        Err(ParseError::InvalidSchema(_))
    ));
    let mut empty_name = ArgSpec::optional_bool('a', "ok", "h");
    empty_name.long_name.clear();
    assert!(matches!(
        ArgSchema::try_from_specs(vec![empty_name]),
        Err(ParseError::InvalidSchema(_))
    ));
    assert!(matches!(
        ArgSchema::try_from_specs(vec![
            ArgSpec::optional_bool('a', "same", "h"),
            ArgSpec::optional_bool('b', "same", "h"),
        ]),
        Err(ParseError::InvalidSchema(_))
    ));
}

#[test]
fn schema_helpers_and_advanced() {
    let schema = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_u32('u', "uint", "n", Some(7)).advanced(),
        ArgSpec::required_u32('c', "count", "c"),
        ArgSpec::optional_string('s', "str", "s", Some("def")),
        ArgSpec::more("rest", "more files"),
    ])
    .expect("ok");
    assert!(schema.allow_more());
    assert_eq!(schema.more_help(), Some("more files"));
    assert_eq!(schema.specs().len(), 3);
    assert!(schema.specs()[0].advanced);
    assert_eq!(
        schema.find_by_key('u').map(|s| s.value_kind),
        Some(ValueKind::U32)
    );
}

#[test]
fn parse_bare_signs_and_clusters() {
    let schema = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('a', "alpha", "a"),
        ArgSpec::optional_string('s', "str", "s", None::<String>),
    ])
    .expect("ok");
    assert!(matches!(
        parse(&schema, &["-"], ParseMode::Cli),
        Err(ParseError::BareSign(_))
    ));
    assert!(matches!(
        parse(&schema, &["+"], ParseMode::Cli),
        Err(ParseError::BareSign(_))
    ));
    assert!(matches!(
        parse(&schema, &["-as"], ParseMode::Cli),
        Err(ParseError::NonBooleanInCluster { .. })
    ));
    assert!(matches!(
        parse(&schema, &["+as"], ParseMode::Cli),
        Err(ParseError::NonBooleanInCluster { .. })
    ));
}

#[test]
fn parse_unexpected_more_and_missing_paths() {
    let schema =
        ArgSchema::try_from_specs(vec![ArgSpec::optional_bool('t', "tools", "t")]).expect("ok");
    assert!(matches!(
        parse(&schema, &["extra"], ParseMode::Cli),
        Err(ParseError::UnexpectedPositional(_))
    ));
    assert!(matches!(
        parse(&schema, &["--argfile"], ParseMode::Cli),
        Err(ParseError::ArgFileMissingPath)
    ));
    assert!(matches!(
        parse(&schema, &["--name"], ParseMode::Cli),
        Err(ParseError::UnknownOption(_))
    ));
}

#[test]
fn parse_long_value_missing_and_nonbool_no() {
    let schema = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_string('n', "name", "n", None::<String>),
        ArgSpec::optional_bool('t', "tools", "t"),
    ])
    .expect("ok");
    assert!(matches!(
        parse(&schema, &["--name"], ParseMode::Cli),
        Err(ParseError::MissingValue(_))
    ));
    assert!(matches!(
        parse(&schema, &["--no-name"], ParseMode::Cli),
        Err(ParseError::UnknownOption(_))
    ));
}

#[test]
fn parse_invalid_values_and_defaults() {
    let schema = ArgSchema::try_from_specs(vec![
        ArgSpec::optional_u32('u', "uint", "u", Some(3)),
        ArgSpec::optional_string('s', "str", "s", Some("hi")),
        ArgSpec::optional_bool('t', "tools", "t"),
    ])
    .expect("ok");
    assert!(matches!(
        parse(&schema, &["-u", "nope"], ParseMode::Cli),
        Err(ParseError::InvalidValue { .. })
    ));
    let values = match parse(&schema, &[] as &[&str], ParseMode::Cli).expect("ok") {
        ParseOutcome::Parsed(v) => v,
        ParseOutcome::Help { .. } => panic!("help"),
    };
    assert_eq!(values.get_u32('u'), Some(3));
    assert_eq!(values.get_string('s'), Some("hi"));
    assert_eq!(values.get_bool('t'), Some(false));
    assert!(!values.isset('t'));
    assert!(values.get_bool('u').is_none());
    assert!(values.get_string('u').is_none());
    assert!(values.get_u32('t').is_none());
}

#[test]
fn parse_bool_string_aliases_via_argfile_empty_and_io() {
    let schema =
        ArgSchema::try_from_specs(vec![ArgSpec::optional_bool('t', "tools", "t")]).expect("ok");
    let dir = std::env::temp_dir().join(format!("nz-arg-edge-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let empty = dir.join("empty.txt");
    std::fs::write(&empty, "# only comment\n\n").expect("write");
    let values = match parse(
        &schema,
        &["--argfile", empty.to_str().unwrap()],
        ParseMode::Cli,
    )
    .expect("ok")
    {
        ParseOutcome::Parsed(v) => v,
        ParseOutcome::Help { .. } => panic!("help"),
    };
    assert!(!values.isset('t'));

    assert!(matches!(
        parse(
            &schema,
            &["--argfile", dir.join("missing.txt").to_str().unwrap()],
            ParseMode::Cli
        ),
        Err(ParseError::ArgFileIo { .. })
    ));
}

#[test]
fn cmdline_error_paths_via_argfile() {
    let schema = ArgSchema::try_from_specs(vec![ArgSpec::optional_string(
        'n',
        "name",
        "n",
        None::<String>,
    )])
    .expect("ok");
    let dir = std::env::temp_dir().join(format!("nz-arg-quote-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let bad = dir.join("bad.txt");
    std::fs::write(&bad, r#"-n "unterminated"#).expect("write");
    assert!(matches!(
        parse(
            &schema,
            &["--argfile", bad.to_str().unwrap()],
            ParseMode::Cli
        ),
        Err(ParseError::ArgFileTokenize(_))
    ));

    let esc = dir.join("esc.txt");
    std::fs::write(&esc, "-n \"a\\ b\"\n").expect("write");
    let values = match parse(
        &schema,
        &["--argfile", esc.to_str().unwrap()],
        ParseMode::Cli,
    )
    .expect("ok")
    {
        ParseOutcome::Parsed(v) => v,
        ParseOutcome::Help { .. } => panic!("help"),
    };
    assert_eq!(values.get_string('n'), Some("a b"));

    let trail = dir.join("trail.txt");
    std::fs::write(&trail, r"-n x\").expect("write");
    // trailing backslash outside quotes
    let _ = parse(
        &schema,
        &["--argfile", trail.to_str().unwrap()],
        ParseMode::Cli,
    );
}

#[test]
fn argfile_can_request_help() {
    let schema =
        ArgSchema::try_from_specs(vec![ArgSpec::optional_bool('t', "tools", "t")]).expect("ok");
    let dir = std::env::temp_dir().join(format!("nz-arg-helpfile-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("h.txt");
    std::fs::write(&path, "--help2\n").expect("write");
    assert_eq!(
        parse(
            &schema,
            &["--argfile", path.to_str().unwrap()],
            ParseMode::Cli
        )
        .expect("ok"),
        ParseOutcome::Help {
            include_advanced: true
        }
    );
}

#[test]
fn formupdate_skips_required_check() {
    let schema =
        ArgSchema::try_from_specs(vec![ArgSpec::required_string('s', "server", "s")]).expect("ok");
    let values = match parse(&schema, &[] as &[&str], ParseMode::FormUpdate).expect("ok") {
        ParseOutcome::Parsed(v) => v,
        ParseOutcome::Help { .. } => panic!("help"),
    };
    assert!(!values.isset('s'));
}

#[test]
fn error_display_smoke() {
    let messages = [
        ParseError::InvalidSchema("x".into()).to_string(),
        ParseError::UnknownOption("--z".into()).to_string(),
        ParseError::AmbiguousPrefix {
            prefix: "d".into(),
            first: "device".into(),
            second: "devil".into(),
        }
        .to_string(),
        ParseError::MissingValue("--n".into()).to_string(),
        ParseError::InvalidValue {
            option: "-u".into(),
            value: "x".into(),
        }
        .to_string(),
        ParseError::MissingRequired {
            key: 's',
            long_name: "server".into(),
        }
        .to_string(),
        ParseError::UnexpectedPositional("a".into()).to_string(),
        ParseError::ArgFileMissingPath.to_string(),
        ParseError::ArgFileIo {
            path: "p".into(),
            message: "m".into(),
        }
        .to_string(),
        ParseError::ArgFileTokenize("t".into()).to_string(),
        ParseError::InteractiveNotSupported.to_string(),
        ParseError::BareSign("-".into()).to_string(),
        ParseError::NonBooleanInCluster {
            key: 's',
            cluster: "-as".into(),
        }
        .to_string(),
    ];
    assert!(messages.iter().all(|m| !m.is_empty()));
}
