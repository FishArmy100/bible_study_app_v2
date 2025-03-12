use crate::{ast::Node, reader::CharReader};

pub fn parse(text: &str) -> Node
{
    let Some(mut reader) = CharReader::new(text) else {
        return Node::Root { children: vec![] };
    };

    let mut children = vec![];
    while let Some(c) = reader.current()
    {
        match c 
        {
            '#' => children.push(parse_heading(&mut reader).unwrap()),
            _ => children.extend(parse_line(&mut reader))
        }
    }

    Node::Root { children }
}

pub fn parse_heading(reader: &mut CharReader) -> Option<Node>
{
    let count = reader.advance_count('#');
    if count == 0 { return None; }

    reader.read_spaces(0); // advance past spaces
    let nodes = parse_line(reader);
    Some(Node::Heading { level: count as u32, children: nodes })
}

pub fn parse_line(reader: &mut CharReader) -> Vec<Node>
{
    let mut nodes = vec![];
    let mut text = String::new();

    while !reader.at_end()
    {
        if let Some(style) = check_style_type(reader)
        {
            if text.len() > 0
            {
                nodes.push(Node::Text { text });
                text = String::new();
            }

            let text = read_style(reader, style);
            nodes.push(match style
            {
                StyleType::Bold => Node::Bold { text },
                StyleType::Italics => Node::Italics { text },
                StyleType::Code => Node::Code { text },
                StyleType::BoldItalic => Node::BoldItalic { text },
            });
        }
        else if let Some(c) = reader.advance()
        {
            if c == '\n'
            {
                break;
            }

            text.push(c);
        }
    }

    if text.len() > 0
    {
        nodes.push(Node::Text { text });
    }

    nodes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StyleType
{
    Bold,
    Italics,
    Code,
    BoldItalic,
}

fn check_style_type(reader: &mut CharReader) -> Option<StyleType>
{
    if reader.check_many("***").is_some()
    {
        Some(StyleType::BoldItalic)
    }
    else if reader.check_many("**").is_some()
    {
        Some(StyleType::Bold)
    }
    else if reader.check_many("*").is_some()
    {
        Some(StyleType::Italics)
    }
    else if reader.check_many("`").is_some()
    {
        Some(StyleType::Code)
    }
    else 
    {
        None    
    }
}

fn read_style(reader: &mut CharReader, style_type: StyleType) -> String
{
    let mut text = String::new();

    let check_fn = match style_type
    {
        StyleType::Bold => |reader: &mut CharReader| reader.check_many("**").is_some(),
        StyleType::Italics => |reader: &mut CharReader| reader.check_many("*").is_some(),
        StyleType::Code => |reader: &mut CharReader| reader.check_many("`").is_some(),
        StyleType::BoldItalic => |reader: &mut CharReader| reader.check_many("***").is_some(),
    };

    while let Some(c) = reader.current() 
    {
        if c == '\n' || check_fn(reader) { 
            break; 
        }
        text.push(reader.advance().unwrap());
    }
    
    text
}