use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Node
{
    Root
    {
        children: Vec<Node>,
    },
    Text 
    {
        text: String,
    },
    Heading
    {
        level: u32,
        children: Vec<Node>,
    },
    Paragraph
    {
        children: Vec<Node>,
    },
    Italics
    {
        text: String
    },
    Bold 
    {
        text: String
    },
    BoldItalic 
    {
        text: String
    },
    Code 
    {
        text: String,
    },
    BlockQuote
    {
        level: u32,
        text: Vec<Node>,
    },
    ColoredText
    {
        color: String,
        text: Vec<Node>,
    },
    OrderedList
    {
        items: Vec<Vec<Node>>,
    },
    UnorderedList
    {
        items: Vec<Vec<Node>>,
    },
    Link
    {
        tag: Option<String>,
        link: String,
    },
    HTML
    {
        contents: String,
    },
}