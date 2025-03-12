use md_parser::parser;

fn main() 
{
    const MARKDOWN: &str = "## **Hello World!**\nThis is Nate Craver, at your service";
    let node = parser::parse(MARKDOWN);

    println!("TEXT:\n{}\n\n", MARKDOWN);
    println!("RENDERED:\n{}", serde_json::to_string_pretty(&node).unwrap());
}
