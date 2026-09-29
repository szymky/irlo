use crate::command::Command;

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
