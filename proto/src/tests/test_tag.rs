use crate::message::{Tag, message_tags, parse_tags, unescape};

fn t(key: &str, value: Option<&str>) -> Tag {
    Tag {
        key: key.to_string(),
        value: value.map(str::to_string),
    }
}

#[test]
fn key_with_value() {
    assert_eq!(parse_tags("id=123"), Ok(("", vec![t("id", Some("123"))])));
}

#[test]
fn key_without_value() {
    assert_eq!(parse_tags("flag"), Ok(("", vec![t("flag", None)])));
}

#[test]
fn empty_value_is_none() {
    assert_eq!(parse_tags("key="), Ok(("", vec![t("key", None)])));
}

#[test]
fn multiple_tags() {
    assert_eq!(
        parse_tags("a=1;b;c=3"),
        Ok(("", vec![t("a", Some("1")), t("b", None), t("c", Some("3"))]))
    );
}

#[test]
fn client_prefix_and_vendor() {
    assert_eq!(
        parse_tags("+draft/reply=abc"),
        Ok(("", vec![t("+draft/reply", Some("abc"))]))
    );
    assert_eq!(
        parse_tags("example.com/foo=bar"),
        Ok(("", vec![t("example.com/foo", Some("bar"))]))
    );
}

#[test]
fn stops_at_space_and_returns_remainder() {
    assert_eq!(
        parse_tags("a=b;c=d :nick PRIVMSG #chan :hi"),
        Ok((
            " :nick PRIVMSG #chan :hi",
            vec![t("a", Some("b")), t("c", Some("d"))]
        ))
    );
}

#[test]
fn value_may_contain_equals_sign() {
    assert_eq!(parse_tags("a=b=c"), Ok(("", vec![t("a", Some("b=c"))])));
}

#[test]
fn trailing_semicolon_is_left_unconsumed() {
    assert_eq!(parse_tags("a=b;"), Ok((";", vec![t("a", Some("b"))])));
}

#[test]
fn rejects_empty_input() {
    assert!(parse_tags("").is_err());
}

#[test]
fn rejects_missing_key() {
    assert!(parse_tags("=value").is_err());
    assert!(parse_tags(";a=b").is_err());
}

#[test]
fn unescapes_values() {
    assert_eq!(
        parse_tags(r"msg=hello\sworld\:\\\r\n"),
        Ok(("", vec![t("msg", Some("hello world;\\\r\n"))]))
    );
}

#[test]
fn unescape_unknown_sequence_drops_backslash() {
    assert_eq!(unescape(r"\b"), "b");
}

#[test]
fn unescape_trailing_backslash_is_dropped() {
    assert_eq!(unescape(r"abc\"), "abc");
}

#[test]
fn unescape_plain_text_is_unchanged() {
    assert_eq!(unescape("plain"), "plain");
}

#[test]
fn message_tags_strips_at_and_space() {
    assert_eq!(
        message_tags("@a=b;c :nick PRIVMSG"),
        Ok((":nick PRIVMSG", vec![t("a", Some("b")), t("c", None)]))
    );
}

#[test]
fn message_tags_requires_at_sign() {
    assert!(message_tags("a=b :nick").is_err());
}

#[test]
fn message_tags_requires_trailing_space() {
    assert!(message_tags("@a=b").is_err());
}
