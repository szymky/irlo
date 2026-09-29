use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while, take_while1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
};

use crate::{command::Command, error::ProtocolError};

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub tag: Option<Vec<Tag>>,
    pub source: Option<String>,
    pub command: Command,
    pub params: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

fn key(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '/' | '+'))
        .parse(input)
}

fn escaped_value(input: &str) -> IResult<&str, String> {
    map(
        take_while(|c: char| !matches!(c, '\0' | '\r' | '\n' | ';' | ' ')),
        unescape,
    )
    .parse(input)
}

fn parse_tag(input: &str) -> IResult<&str, Tag> {
    map(
        pair(key, opt(preceded(tag("="), escaped_value))),
        |(key, value)| Tag {
            key: key.to_string(),
            value: value.filter(|v| !v.is_empty()),
        },
    )
    .parse(input)
}

pub fn parse_tags(input: &str) -> IResult<&str, Vec<Tag>> {
    separated_list1(tag(";"), parse_tag).parse(input)
}

pub fn unescape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some(':') => out.push(';'),
                Some('s') => out.push(' '),
                Some('r') => out.push('\r'),
                Some('n') => out.push('\n'),
                Some(other) => out.push(other),
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn message_tags(input: &str) -> IResult<&str, Vec<Tag>> {
    delimited(tag("@"), parse_tags, tag(" ")).parse(input)
}

impl TryFrom<String> for Message {
    type Error = ProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let has_tag = value.starts_with('@');

        println!("Has_tag:{}", has_tag);

        Ok(Self {
            tag: None,
            source: None,
            command: Command::None,
            params: Vec::new(),
        })
    }
}
