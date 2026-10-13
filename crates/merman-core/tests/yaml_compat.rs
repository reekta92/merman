//! Compatibility contract captured against the 0.6.0 serde_yaml backend.
//! Keep this alongside the semantic snapshots when replacing the YAML parser.

use merman_core::{Engine, ParseOptions};
use serde::Deserialize;
use serde_json::json;
use serde_yaml::{Deserializer, Value};

#[test]
fn frontmatter_keeps_aliases_quotes_multiline_and_crlf() {
    let source = "---\r\ntitle: |\r\n  Native diagram\r\n  Second line\r\ndefaults: &defaults {nodeSpacing: 72, curve: 'stepAfter'}\r\nconfig:\r\n  flowchart: *defaults\r\n  theme: dark\r\n  themeVariables: {fontSize: '002', enabled: true}\r\n---\r\nflowchart LR\r\n  A --> B\r\n";
    let parsed = Engine::new()
        .parse_metadata_sync(source, ParseOptions::strict())
        .unwrap()
        .unwrap();
    assert_eq!(parsed.diagram_type, "flowchart-v2");
    assert_eq!(
        parsed.title.as_deref(),
        Some("Native diagram\nSecond line\n")
    );
    let config = parsed.config.as_value();
    assert_eq!(
        config["flowchart"],
        json!({"nodeSpacing":72,"curve":"stepAfter"})
    );
    assert_eq!(config["theme"], "dark");
    assert_eq!(
        config["themeVariables"],
        json!({"fontSize":"002","enabled":true})
    );
}

#[test]
fn value_roundtrip_keeps_tags_complex_keys_numbers_dates_and_null() {
    let source = "tagged: !plugin {quoted: 'true', sequence: [null, false, 3.5]}\n? [a, b]\n: complex key\nunsigned: 18446744073709551615\nhex: 0x10\nexponent: 1e3\ndate: 2026-10-13\nleading: '002'\nmultiline: |\n  First\n  Second\n";
    let value: Value = serde_yaml::from_str(source).unwrap();
    assert_eq!(value["unsigned"].as_u64(), Some(u64::MAX));
    assert_eq!(value["hex"].as_u64(), Some(16));
    assert_eq!(value["exponent"].as_f64(), Some(1000.0));
    assert_eq!(value["date"].as_str(), Some("2026-10-13"));
    assert_eq!(value["leading"].as_str(), Some("002"));
    assert_eq!(value["multiline"].as_str(), Some("First\nSecond\n"));
    let Value::Tagged(tagged) = &value["tagged"] else {
        panic!("opaque tag was lost");
    };
    assert_eq!(tagged.tag.to_string(), "!plugin");
    assert_eq!(tagged.value["quoted"].as_str(), Some("true"));
    let key = Value::Sequence(vec![Value::String("a".into()), Value::String("b".into())]);
    assert_eq!(
        value.as_mapping().unwrap().get(&key).unwrap().as_str(),
        Some("complex key")
    );
    let encoded = serde_yaml::to_string(&value).unwrap();
    assert_eq!(serde_yaml::from_str::<Value>(&encoded).unwrap(), value);
}

#[test]
fn malformed_aliases_recursive_aliases_and_multiple_documents_reject() {
    for source in [
        "value: [unterminated",
        "value: *absent",
        "value: &cycle [*cycle]",
        "one: 1\n---\ntwo: 2\n",
    ] {
        assert!(
            serde_yaml::from_str::<Value>(source).is_err(),
            "accepted {source:?}"
        );
    }
    for yaml in [
        "config: [unterminated",
        "config: *absent",
        "config: &cycle [*cycle]",
    ] {
        let source = format!("---\n{yaml}\n---\nflowchart LR\nA --> B\n");
        assert!(
            Engine::new()
                .parse_metadata_sync(&source, ParseOptions::strict())
                .is_err()
        );
    }
    let documents = Deserializer::from_str("first: 1\n---\nsecond: 2\n")
        .map(Value::deserialize)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(documents.len(), 2);
    assert_eq!(documents[0]["first"].as_u64(), Some(1));
    assert_eq!(documents[1]["second"].as_u64(), Some(2));
}
