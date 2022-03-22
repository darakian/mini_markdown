use crate::MiniIter;

/// Tokens are the intermediate representation format in the markdown to html conversion
#[derive(Debug, PartialEq)]
pub enum Token <'a>{
    /// String: Body of unstructured text
    Plaintext(String),
    /// u8: Header level (1..=6). String: Header text. Option<String>: html label
    Header(u8, String, Option<String>),
    /// String: Text for list entry
    UnorderedListEntry(String),
    /// String: Text for list entry
    OrderedListEntry(String),
    /// String: Text to be italicized
    Italic(String),
    /// String: Text to be bolded
    Bold(String),
    /// String: Text to be bolded and italicized
    BoldItalic(String),
    /// Corresponds to a </br> html tag
    LineBreak,
    /// Corresponds to a newline character
    Newline,
    /// Corresponds to a <hr /> html tag
    HorizontalRule,
    /// Used for control flow. Not directly rendered
    Tab,
    /// Used for control flow. Not directly rendered
    DoubleTab,
    /// String: Text to be struck through
    Strikethrough(String),
    /// String: Text to be placed within an inline code tag. eg. <code>String</code>
    Code(String),
    /// First String: Text to be placed within a multi-line code tag. Second String: Language
    CodeBlock(String, String),
    /// u8: Block quote level. String: Block quote text
    BlockQuote(u8, String),
    /// String: Link. Option<String>: Title for link.
    Image(String, Option<String>),
    /// String: Link. First Option<String>: Title for link. Second Option<String>: Hover text
    Link(String, Option<String>, Option<String>),
    /// String: Summary. Vec<Token>: Tokens to be rendered in the collapsable section
    Detail(String, Vec<Token<'a>>),
    /// Tuple of Vec<(Alignment, String)>: Which defines the table header and Vec<Vec<(Alignment, Vec<Token>)>> which defines the rows
    Table(Vec<(Alignment, &'a str)>, Vec<Vec<(Alignment, Vec<Token<'a>>)>>),
    /// TaskBox: Boolean state of the checked or unchecked box. String: List item text
    TaskListItem(TaskBox, String),
    /// First String: Reference id. Second String: Reference text
    Footnote(String, String),
}

/// Holds the possible states of a taskbox in a task list
#[derive(Debug, PartialEq)]
pub enum TaskBox {
    Checked,
    Unchecked,
}

impl Token <'_> {
    fn is_usable_in_table(&self) -> bool {
        match self {
            Token::Code(_) => true,
            Token::Link(_, _, _) => true,
            Token::Bold(_) => true,
            Token::Italic(_) => true,
            Token::BoldItalic(_) => true,
            Token::Plaintext(_) => true,
            _ => false
        }
    }
}

/// Holds the alignment states for the table token
#[derive(Debug, PartialEq, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center
}

use std::fmt;
impl fmt::Display for Alignment{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Alignment::Left => return write!(f, "left"),
            Alignment::Right => return write!(f, "right"),
            Alignment::Center => return write!(f, "center"),
        }
    }
}


#[derive(Debug)]
pub(crate) struct ParseError<'a>{
    pub(crate) content: &'a str,
}

pub(crate) fn push_str<'a>(t: &mut Vec<Token>, s: &'a str) {
    match t.last_mut() {
        Some(markdown_token) => {
            match markdown_token {
                Token::Plaintext(plaintext_token) => plaintext_token.push_str(s),
                _ => t.push(Token::Plaintext(s.to_string())),
            }
        }
        None => t.push(Token::Plaintext(s.to_string())),
    }
}

pub(crate) fn lex_heading<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let hashes = char_iter.consume_while_case_holds(&|c| c == "#").unwrap_or("");
    if char_iter.next_if_eq(&" ").is_none(){
        return Err(ParseError{content: hashes});
    }
    let level = std::cmp::min(6, hashes.len() as u8);
    let mut line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("").to_string();
    if line.contains("{#") && 
        line.contains('}') {
            let heading = line.chars().take_while(|c| c != &'{').collect::<String>();
            line.replace_range(..heading.len(), "");
            line.retain(|c| c != '{' && c != '#' && c != '}');
            return Ok(Token::Header(level, heading.trim().to_string(), Some(line)));
        }
    return Ok(Token::Header(level, line, None));
}

pub(crate) fn lex_asterisk_underscore<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let asterunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
    if asterunds.len() == 1 && char_iter.next_if_eq(&" ").is_some(){
        let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
        char_iter.next();
        return Ok(Token::UnorderedListEntry(s.to_string()))
    }
    if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&"\n"){
        return Ok(Token::HorizontalRule)
    }
    match asterunds.len() {
        1 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            if char_iter.peek() != Some("*") || char_iter.peek() != Some(&"_"){
                char_iter.next();
                return Ok(Token::Italic(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        2 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 2 {
                return Ok(Token::Bold(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        3 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 3 {
                return Ok(Token::BoldItalic(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        _ => {
            if asterunds.chars().all(|x| x == '*') || asterunds.chars().all(|x| x == '_'){
                return Ok(Token::HorizontalRule)
            } else {
                return Err(ParseError{content: asterunds})
            }
        }
    }
}

pub(crate) fn lex_spaces<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>>{
    let spaces = char_iter.consume_while_case_holds(&|c| c == " ").unwrap_or("");
    // Case 1: space in text => return char
    if spaces.len() == 1 {
        return Err(ParseError{content: spaces})
    }
    // Case 2: two or more spaces followed by \n => line break
    if char_iter.peek() == Some(&"\n"){
        return Ok(Token::LineBreak);
    }
    // Case 3: Tokenize for parser
    match spaces.len(){
        4 => return Ok(Token::Tab),
        8 => return Ok(Token::DoubleTab),
        _ => {}
    }
    Err(ParseError{content: spaces})
}

pub(crate) fn lex_backticks<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let leading_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
    let mut lang = "plaintext";
    if leading_ticks.len() != 1 && leading_ticks.len() != 3{
        return Err(ParseError{content: leading_ticks})
    }
    if leading_ticks.len() == 1 {
        let s = char_iter.consume_while_case_holds(&|c| c != "`").unwrap_or("");
        let trailing_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
        if leading_ticks.len() != trailing_ticks.len() {
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
        } else {
            return Ok(Token::Code(s.to_string()))
        }
    }
    // leading_ticks.len() == 3. Check for lang
    if char_iter.peek() != Some(&"\n") {
        lang = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
        char_iter.next();
    } else {
        char_iter.next();
    }
    let s = char_iter.consume_while_case_holds(&|c| c != "`").unwrap_or("");
    let trailing_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
    if leading_ticks.len() != trailing_ticks.len() {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
    } else {
        return Ok(Token::CodeBlock(s.to_string(), lang.to_string()))
    }
}

pub(crate) fn lex_newlines<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let newlines = char_iter.consume_while_case_holds(&|c| c == "\n").unwrap_or("");
    match newlines.len() {
        0..=1 => return Err(ParseError{content: newlines}),
        _ => return Ok(Token::Newline)
    }
}

pub(crate) fn lex_blockquotes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let right_arrows = char_iter.consume_while_case_holds(&|c| c == ">").unwrap_or("");
    match char_iter.peek() {
        Some(" ") => {char_iter.next();},
        _ => {return Err(ParseError{content: right_arrows})}
    }
    let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    char_iter.next_if_eq(&"\n");
    Ok(Token::BlockQuote(right_arrows.len() as u8, s.to_string()))
}

pub(crate) fn lex_images<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.peek() != Some(&"!"){
        return Err(ParseError{content: ""})
    }
    char_iter.next();
    let link_result = lex_links(char_iter);
    match link_result {
        Err(_e) => return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}),
        Ok(Token::Link(link, title, _)) => return Ok(Token::Image(link, title)),
        _ => return Err(ParseError{content: "Non link token returned from lex_links"})
    }
}

pub(crate) fn lex_links<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.peek() != Some(&"[") {
        return Err(ParseError{content: ""})
    }
    char_iter.next();
    let title = char_iter.consume_while_case_holds(&|c| c != "]").unwrap_or("");
    if char_iter.peek() != Some(&"]") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    char_iter.next();
    // Parse footnotes big and small
    if title.starts_with("^") && char_iter.peek() == Some(&":") {
        char_iter.next();
        let ref_id = title.strip_prefix("^").unwrap_or("");
        let mut note_text = String::new();
        loop {
           note_text.push_str(char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or(""));
           char_iter.next();
           if char_iter.peek() != Some(&" ") && char_iter.peek() != Some(&"\t") {
            break;
           }
           if char_iter.peek() == Some(&"\t") {
            char_iter.next();
            note_text.push('\n');
            continue;
           }
           if char_iter.peek() == Some(&" ") {
            let spaces = char_iter.consume_while_case_holds(&|c| c == " ").unwrap_or("");
            match spaces.len() {
                2 | 4 => {note_text.push('\n');},
                _ => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})},
            }
            continue
           }
           break;
        }
        if ref_id.contains(char::is_whitespace){
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
        }
        return Ok(Token::Footnote(ref_id.to_string(), note_text.trim_start().to_string()));
    }
    if char_iter.peek() != Some(&"(") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    char_iter.next();
    let link = char_iter.consume_while_case_holds(&|c| c != ")" && c != " ").unwrap_or("");
    if char_iter.peek() != Some(&")") && char_iter.peek() != Some(&" ") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    if char_iter.peek() == Some(&")") {
        char_iter.next();
        return Ok(Token::Link(link.to_string(), Some(title.to_string()), None));
    }
    if char_iter.peek() == Some(&" ") {
        let hover = char_iter.consume_while_case_holds(&|c| c != ")").unwrap_or("");
        char_iter.skip_while(|c| c != &"\n").next();
        return Ok(Token::Link(link.to_string(), Some(title.to_string()), Some(hover.to_string())));
    }
    Err(ParseError{content: ""})
}

pub(crate) fn lex_side_carrot<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    match char_iter.peek() {
        Some("<") => {
            char_iter.next();
            let s = char_iter.consume_while_case_holds(&|c| c != ">").unwrap_or("");
            match char_iter.peek(){
                Some(">") if s != "details" && s != "/details" => {
                    char_iter.next();
                    return Ok(Token::Link(s.to_string(), None, None))
                },
                Some(">") if s == "details" || s == "/details"  => {
                    char_iter.next();
                    char_iter.next_if_eq(&"\r");
                    if !char_iter.next_if_eq(&"\n").is_some(){
                        return Err(ParseError{content: s});
                    }
                    println!(">> {:?}", char_iter.peek());
                    return parse_details(char_iter)
                },
                _ => {
                    return Err(ParseError{content: s});
                }
            }
        }
        _ => return Err(ParseError{content: ""})
    }
}

pub(crate) fn lex_plus_minus<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token, ParseError<'a>> {
    let s = char_iter.consume_while_case_holds(&|c| c == "-" || c == "+").unwrap_or("");
    match s.len() {
        3..=usize::MAX => { return Ok(Token::HorizontalRule)},
        2 => {return Ok(Token::Plaintext(s.to_string()))},
        1 => {},
        _ => {return Err(ParseError{content: "string length error"})},
    }
    let line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    if line.starts_with(" [ ] ") {
        return Ok(Token::TaskListItem(TaskBox::Unchecked,line.strip_prefix(" [ ] ").unwrap_or("").to_string()))
    } else if line.starts_with(" [x] ") {
        return Ok(Token::TaskListItem(TaskBox::Checked,line.strip_prefix(" [x] ").unwrap_or("").to_string()))
    } else if line.starts_with(" [X] ") {
        return Ok(Token::TaskListItem(TaskBox::Checked,line.strip_prefix(" [X] ").unwrap_or("").to_string()))
    } else if line.starts_with(" "){
        return Ok(Token::UnorderedListEntry(line.strip_prefix(" ").unwrap_or("").to_string()))
    } else {
        return Ok(Token::Plaintext(s.to_owned()+line))
    }
}

pub(crate) fn lex_numbers<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let c = char_iter.next().unwrap();
    match char_iter.peek() {
        Some(".") => {
            let dot = char_iter.next().unwrap();
            if char_iter.peek() != Some(&" "){
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
            }
            char_iter.next();
            let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
            return Ok(Token::OrderedListEntry(s.to_string()))
        },
        _ => return Err(ParseError{content: c})
    }
}

pub(crate) fn lex_tilde<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let lead_tildes = match char_iter.consume_while_case_holds(&|s| s == "~"){
        Some(s) => s,
        None => return Err(ParseError{content: "Failure to parse ~"}),
    };
    match lead_tildes.len() {
        1 => return Err(ParseError{content: lead_tildes}),
        2 => {
            let line = char_iter.consume_while_case_holds(&|s| s != "~").unwrap_or("");
            let tail_tildes = char_iter.consume_while_case_holds(&|s| s == "~").unwrap_or("");
            if lead_tildes.len() != tail_tildes.len() {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
                // return Err(ParseError{content: format!("{}{}{}",  lead_tildes, line, tail_tildes)})
            }
            return Ok(Token::Strikethrough(line.to_string()));
        }
        _ => return Err(ParseError{content: lead_tildes}),
    }
}

fn parse_details<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>>{
    let mut summary_line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    if summary_line.ends_with("\r") {
        summary_line = summary_line.strip_suffix("\r").unwrap_or("");
    }
    if !summary_line.starts_with("<summary") || !summary_line.ends_with("</summary>") {
        return Err(ParseError{content: summary_line});
    }
    summary_line = summary_line.strip_prefix("<summary>").unwrap_or("");
    summary_line = summary_line.strip_suffix("</summary>").unwrap_or("");
    match summary_line.len() {
        0 => {return Err(ParseError{content: "<summary></summary>"})},
        _ => {},
    }
    println!("? {:?}", summary_line);
    let pre_detail_text = char_iter.consume_while_case_holds(&|c| c != "<").unwrap_or("");
    println!("? {:?}", pre_detail_text);
    let text_marker = char_iter.get_index();
    let mut remaining_text = char_iter.consume_until_tail_is("details>").unwrap_or(""); // Consume until start of end detail tag
    if remaining_text.contains("<details>") {
        let mut opens = remaining_text.matches("<details>").count();
        let mut closes = remaining_text.matches("</details>").count();
        while opens != closes {
            char_iter.consume_until_tail_is("</details>");
            opens = remaining_text.matches("<details>").count();
            closes = remaining_text.matches("</details>").count();
            remaining_text = char_iter.get_substring_from(text_marker).unwrap_or("");
        }
    remaining_text = char_iter.get_substring_from(text_marker).unwrap_or("");
    } else if remaining_text.contains("</details>") {
        remaining_text = remaining_text.strip_suffix("</details>").unwrap_or("");
    }
    else {return Err(ParseError{content: remaining_text});}
    println!(">>  {:?}", remaining_text);
    let mut pre_detail_tokens = crate::lex(pre_detail_text);
    let mut inner_tokens = crate::lex(remaining_text);
    println!(">>  {:?}", inner_tokens);
    pre_detail_tokens.append(&mut inner_tokens);
    println!("Returning?? {:?}:{:?}", summary_line, pre_detail_tokens);
    Ok(Token::Detail(summary_line.to_string(), pre_detail_tokens))
}

pub(crate) fn lex_pipes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let mut lines = Vec::new();
    while char_iter.next_if_eq(&"|") == Some("|") {
        let line_start = char_iter.get_index();
        char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
        lines.push(char_iter.get_substring_from(line_start).unwrap_or(""));
        char_iter.next();
    }
    if lines.len() < 3 {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    if !lines.iter().all(|l| l.matches("|").count() == lines[0].matches("|").count()) {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    let headings: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim())
        .collect();
    let alignments: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| 
            {
            match (x.trim().starts_with(":"), x.trim().ends_with(":")) {
                (true, false) => Alignment::Left,
                (true, true) => Alignment::Center,
                (false, true) => Alignment::Right,
                _ => Alignment::Left,
            }}
        )
        .collect();
    let mut rows = Vec::new();
    for l in lines.iter() {
        let elements: Vec<_> = l.split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim())
        .collect();
        let mut r = Vec::new();
        for e in elements.into_iter() {
            let mut inner_tokens = crate::lex(&e);
            inner_tokens.retain(|token| token.is_usable_in_table());
            r.push(inner_tokens);
        }
        rows.push(alignments.clone().into_iter().zip(r.into_iter()).collect());
    }
    Ok(Token::Table(alignments.into_iter().zip(headings.into_iter()).collect(), rows))
}