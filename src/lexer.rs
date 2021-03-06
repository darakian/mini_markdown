use crate::MiniIter;

/// Tokens are the intermediate representation format in the markdown to html conversion
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    /// String: Body of unstructured text
    Plaintext(String),
    /// u8: Header level (1..=6). str: Header text. Option<str>: html label
    Header(u8, &'a str, Option<&'a str>),
    /// str: Text for list entry
    UnorderedListEntry(&'a str),
    /// str: Text for list entry
    OrderedListEntry(&'a str),
    /// str: Text to be italicized
    Italic(&'a str),
    /// str: Text to be bolded
    Bold(&'a str),
    /// str: Text to be bolded and italicized
    BoldItalic(&'a str),
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
    /// str: Text to be struck through
    Strikethrough(&'a str),
    /// str: Text to be placed within an inline code tag. eg. <code>str</code>
    Code(&'a str),
    /// First str: Text to be placed within a multi-line code tag. Second str: Language
    CodeBlock(&'a str, &'a str),
    /// u8: Block quote level. str: Block quote text
    BlockQuote(u8, &'a str),
    /// str: Link. Option<str>: Title for link.
    Image(&'a str, Option<&'a str>),
    /// str: Link. First Option<str>: Title for link. Second Option<str>: Hover text
    Link(&'a str, Option<&'a str>, Option<&'a str>),
    /// str: Summary. Vec<Token>: Tokens to be rendered in the collapsable section
    Detail(&'a str, Vec<Token<'a>>),
    /// Tuple of Vec<(Alignment, str)>: Which defines the table header and Vec<Vec<(Alignment, Vec<Token>)>> which defines the rows
    Table(Vec<(Alignment, &'a str)>, Vec<Vec<(Alignment, Vec<Token<'a>>)>>),
    /// TaskBox: Boolean state of the checked or unchecked box. str: List item text
    TaskListItem(TaskBox, &'a str),
    /// First str: Reference id. Second str: Reference text
    Footnote(&'a str, &'a str),
}

/// Holds the possible states of a taskbox in a task list
#[derive(Debug, PartialEq)]
pub enum TaskBox {
    Checked,
    Unchecked,
}

impl <'a> Token<'a> {
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
        Some(Token::Plaintext(token)) => {
            token.push_str(s)
        }
        _ => t.push(Token::Plaintext(s.to_string())),
    }
}

pub(crate) fn lex_heading<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let hashes = char_iter.consume_while_case_holds(&|c| c == "#").unwrap_or("");
    if char_iter.next_if_eq(&" ").is_none(){
        return Err(ParseError{content: hashes});
    }
    let level = std::cmp::min(6, hashes.len() as u8);
    let line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    if line.contains("{#") && 
        line.contains('}') {
            let (heading, _title) = line.split_once("{").unwrap_or(("",""));
            let line = line.strip_prefix(&heading).unwrap()
                            .strip_prefix("{#").unwrap()
                            .strip_suffix("}").unwrap();
            return Ok(Token::Header(level, heading.trim(), Some(line)));
        }
    return Ok(Token::Header(level, line, None));
}

pub(crate) fn lex_asterisk_underscore<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let asterunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
    if asterunds.len() == 1 && char_iter.next_if_eq(&" ").is_some(){
        let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
        char_iter.next();
        return Ok(Token::UnorderedListEntry(s))
    }
    if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&"\n"){
        return Ok(Token::HorizontalRule)
    }
    match asterunds.len() {
        1 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            if char_iter.peek() != Some("*") || char_iter.peek() != Some(&"_"){
                char_iter.next();
                return Ok(Token::Italic(s))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        2 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 2 {
                return Ok(Token::Bold(s))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        3 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 3 {
                return Ok(Token::BoldItalic(s))
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
    if char_iter.next_if_eq("\n").is_some() {
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
    let mut lang = "";
    if leading_ticks.len() == 3 {
        if char_iter.next_if_eq("\n") != Some(&"\n") {
            lang = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
            char_iter.next();
        }
        let s = char_iter.consume_while_case_holds(&|c| c != "`").unwrap_or("");
        let trailing_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
        if leading_ticks.len() != trailing_ticks.len() {
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
        } else {
            return Ok(Token::CodeBlock(s, lang))
        }
    }

    // let s = char_iter.consume_while_case_holds(&|c| c != "`" && c!= "\n").unwrap_or("");
    let tail = &(0..leading_ticks.len() as u64).map(|_| "`").collect::<String>();
    let s = char_iter.consume_until_tail_is(tail).unwrap_or("");
    if !s.ends_with(tail) {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
    } else {
        let s = s.trim_end_matches(tail);
        if s.starts_with(' ') && s.ends_with(' ') {
            return Ok(Token::Code(s.trim_start_matches(' ').trim_end_matches(' ')))
        }
        return Ok(Token::Code(s))
    }

    // leading_ticks.len() == 3. Check for lang

}

pub(crate) fn lex_newlines<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    match char_iter.consume_while_case_holds(&|c| c == "\n") {
        Some(s) if s.len() >= 1 => return Ok(Token::Newline),
        Some(s) if s.len() < 1 => return Err(ParseError{content: s}),
        _ => return Err(ParseError{content: ""}),
    }
}

pub(crate) fn lex_tabs<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    match char_iter.consume_while_case_holds(&|c| c == "\t") {
        Some(s) if s.len() > 1 => return Ok(Token::DoubleTab),
        Some(s) if s.len() == 1 => return Ok(Token::Tab),
        _ => return Err(ParseError{content: ""}),
    }
}

pub(crate) fn lex_blockquotes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let right_arrows = char_iter.consume_while_case_holds(&|c| c == ">").unwrap_or("");
    match char_iter.next_if_eq(" ") {
        Some(" ") => {},
        _ => {return Err(ParseError{content: right_arrows})}
    }
    let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    char_iter.next_if_eq(&"\n");
    Ok(Token::BlockQuote(right_arrows.len() as u8, s))
}

pub(crate) fn lex_images<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.next_if_eq("!") != Some(&"!"){
        return Err(ParseError{content: ""})
    }
    let link_result = lex_links(char_iter);
    match link_result {
        Err(_e) => return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}),
        Ok(Token::Link(link, title, _)) => return Ok(Token::Image(link, title)),
        _ => return Err(ParseError{content: "Non link token returned from lex_links"})
    }
}

pub(crate) fn lex_links<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.next_if_eq("[") != Some(&"[") {
        return Err(ParseError{content: ""})
    }
    let title = char_iter.consume_while_case_holds(&|c| c != "]").unwrap_or("");
    if char_iter.next_if_eq("]") != Some(&"]") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    // Parse footnotes big and small
    if title.starts_with("^") && char_iter.next_if_eq(":") == Some(&":") {
        let ref_id = title.strip_prefix("^").unwrap_or("");
        let note_index = char_iter.get_index();
        loop {
           char_iter.consume_while_case_holds(&|c| c != "\n");
           char_iter.next();
           if char_iter.peek() != Some(&" ") && char_iter.peek() != Some(&"\t") {
            break;
           }
           if char_iter.next_if_eq("\t") == Some(&"\t") {
            continue;
           }
           if char_iter.peek() == Some(&" ") {
            let spaces = char_iter.consume_while_case_holds(&|c| c == " ").unwrap_or("");
            match spaces.len() {
                2 | 4 => {},
                _ => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})},
            }
            continue
           }
           break;
        }
        if ref_id.contains(char::is_whitespace){
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
        }
        return Ok(Token::Footnote(ref_id, char_iter.get_substring_from(note_index).unwrap_or("").trim()));
    }
    if char_iter.next_if_eq("(") != Some(&"(") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    let link = char_iter.consume_while_case_holds(&|c| c != ")" && c != " ").unwrap_or("");
    if char_iter.peek() != Some(&")") && char_iter.peek() != Some(&" ") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    if char_iter.next_if_eq(")") == Some(&")") {
        return Ok(Token::Link(link, Some(title), None));
    }
    if char_iter.peek() == Some(&" ") {
        let hover = char_iter.consume_while_case_holds(&|c| c != ")").unwrap_or("");
        char_iter.skip_while(|c| c != &"\n").next();
        return Ok(Token::Link(link, Some(title), Some(hover)));
    }
    Err(ParseError{content: ""})
}

pub(crate) fn lex_side_carrot<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    match char_iter.next_if_eq("<") {
        Some("<") => {
            let s = char_iter.consume_while_case_holds(&|c| c != ">").unwrap_or("");
            match (s, char_iter.next_if_eq(">")) {
                ("details", Some(">")) => {
                    char_iter.next_if_eq(&"\r");
                    if !char_iter.next_if_eq(&"\n").is_some(){
                        return Err(ParseError{content: s});
                    }
                    return parse_details(char_iter)
                },
                (_, Some(">")) => return Ok(Token::Link(s, None, None)),
                (_, _) => return Err(ParseError{content: s}),
            }
        }
        _ => return Err(ParseError{content: ""})
    }
}

pub(crate) fn lex_plus_minus<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let s = char_iter.consume_while_case_holds(&|c| c == "-" || c == "+").unwrap_or("");
    match s.len() {
        3..=usize::MAX => { return Ok(Token::HorizontalRule)},
        2 => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})},
        1 => {},
        _ => {return Err(ParseError{content: "string length error"})},
    }
    let line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    if line.starts_with(" [ ] "){return Ok(Token::TaskListItem(TaskBox::Unchecked, &line[5..]))}
    else if line.starts_with(" [x] ") || line.starts_with(" [X] "){return Ok(Token::TaskListItem(TaskBox::Checked, &line[5..]))}
    else if line.starts_with(" "){return Ok(Token::UnorderedListEntry(&line[1..]))}
    else {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})}
}

pub(crate) fn lex_numbers<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let c = char_iter.next().unwrap();
    match char_iter.next_if_eq(".") {
        Some(".") => {
            if char_iter.next_if_eq(" ") != Some(&" "){
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
            }
            let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
            return Ok(Token::OrderedListEntry(s))
        },
        _ => return Err(ParseError{content: c})
    }
}

pub(crate) fn lex_tilde<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let lead_tildes = match char_iter.consume_while_case_holds(&|s| s == "~") {
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
            }
            return Ok(Token::Strikethrough(line));
        }
        _ => return Err(ParseError{content: lead_tildes}),
    }
}

fn parse_details<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>>{
    let mut summary_line = char_iter.consume_while_case_holds(&|c| c != "\n" && c != "\r").unwrap_or("");
    char_iter.next_if_eq("\r");
    if (!summary_line.starts_with("<summary>") || !summary_line.ends_with("</summary>")) && !summary_line.len() >= 20 {
        return Err(ParseError{content: summary_line});
    }
    summary_line = summary_line.strip_prefix("<summary>").unwrap()
                    .strip_suffix("</summary>").unwrap_or("");
    let remaining_text_index = char_iter.get_index();
    let mut remaining_text = char_iter.consume_until_tail_is("</details>").unwrap_or("");
    if remaining_text.contains("<details>") {
        let mut opens = remaining_text.matches("<details>").count();
        let mut closes = remaining_text.matches("</details>").count();
        while opens == closes {
            char_iter.consume_until_tail_is("</details>").unwrap_or("");
            remaining_text = char_iter.get_substring_from(remaining_text_index).unwrap_or("");
            opens = remaining_text.matches("<details>").count();
            closes = remaining_text.matches("</details>").count();
        }
    }
    let inner_tokens = crate::lex(remaining_text.strip_suffix("</details>").unwrap_or(""));
    Ok(Token::Detail(summary_line, inner_tokens))
}

pub(crate) fn lex_pipes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let mut lines = Vec::new();
    while char_iter.next_if_eq(&"|") == Some("|") {
        lines.push(char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or(""));
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
    for l in lines.into_iter() {
        let elements: Vec<&str> = l.split("|")
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