pub trait CommonmarkBlock {
    fn render(&self) -> String;
}

#[derive(Debug)]
pub enum LeafBlock{
    ThematicBreak,
    /// u8: ATXHeading level (1..=6). str: ATXHeading text.
    ATXHeading(u8, String),
    /// u8: SetextHeading level (1..=6). str: ATXHeading text.
    SetextHeading(u8, String),
    /// String: Codeblock text
    IndentedCodeBlock(String),
    /// String: Unescaped Htmlblock text
    HTMLBlock(String),
    /// String: Link label. String: Link destination. Option<String> Link title
    LinkReferenceDefinitions(String, String, Option<String>),
    /// String: Body of unstructured text
    Paragraph(String),
    /// usize: Number of blank lines
    BlankLines(usize),
}

impl CommonmarkBlock for LeafBlock {
    fn render (&self) -> String {
        match self {
            LeafBlock::ThematicBreak => "<hr />".to_string(),
            LeafBlock::ATXHeading(level, text) => {format!("<h{level}>{text}</h{level}>")},
            LeafBlock::SetextHeading(level, text) => {format!("<h{level}>{text}</h{level}>")},
            LeafBlock::IndentedCodeBlock(code_text) => {format!("<pre><code>{code_text}</code></pre>")},
            LeafBlock::HTMLBlock(raw_html) => {raw_html.to_string()},
            LeafBlock::LinkReferenceDefinitions(_, _, _) => {"TODO".to_string()},
            LeafBlock::Paragraph(para_text) => {format!("<p>{para_text}</p>")},
            LeafBlock::BlankLines(_num) => {"".to_string()},
        }   
    }
    
}

pub enum ContainerBlock{
    /// Vec: Interior blocks
    BlockQuote(Vec<Box<dyn CommonmarkBlock>>),
    /// Vec: Interior blocks
    BulletListMarker(Vec<Box<dyn CommonmarkBlock>>),
    /// usize: list item number. Vec: Interior blocks
    OrderedListMarker(usize, Vec<Box<dyn CommonmarkBlock>>),
}


impl CommonmarkBlock for ContainerBlock {
    fn render (&self) -> String {
        match self {
            ContainerBlock::BlockQuote(blocks) => {
                let inner_text = blocks.iter().map(|x| x.render()).collect::<Vec<_>>().join("");
                format!("<blockquote>\n{inner_text}\n</blockquote>")
            },
            ContainerBlock::BulletListMarker(blocks) => {"TODO".to_string()},
            ContainerBlock::OrderedListMarker(list_num, blocks) => {"TODO".to_string()},
 
        }   
    }
    
}