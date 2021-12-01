#[derive(Debug, PartialEq)]
pub enum Token {
    Plaintext(String),
    Header(u8, String, Option<String>),
    UnorderedListEntry(String),
    OrderedListEntry(String),
    Italic(String),
    Bold(String),
    BoldItalic(String),
    LineBreak,
    Newline,
    HorizontalRule,
    Tab,
    DoubleTab,
    Strikethrough(String),
    Code(String),
    CodeBlock(String, String),
    BlockQuote(u8, String),
    Image(String, Option<String>), // (Link, title)
    Link(String, Option<String>, Option<String>), //(link, title, hover text)
    Detail(String, Vec<Token>),
    Table(Vec<(Alignment, String)>, Vec<Vec<(Alignment, Vec<Token>)>>),
    TaskListItem(TaskBox, String),
    Footnote(String, String),
}

#[derive(Debug, PartialEq)]
pub enum TaskBox {
    Checked,
    Unchecked,
}

impl Token {
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
pub(crate) struct ParseError{
    pub(crate) content: String,
}

pub(crate) fn push_char(t: &mut Vec<Token>, c: char) {
    match t.last_mut() {
        Some(markdown_token) => {
            match markdown_token {
                Token::Plaintext(plaintext_token) => plaintext_token.push(c),
                _ => t.push(Token::Plaintext(c.to_string())),
            }
        }
        None => t.push(Token::Plaintext(c.to_string())),
    }
}

pub(crate) fn push_str(t: &mut Vec<Token>, s: String) {
    match t.last_mut() {
        Some(markdown_token) => {
            match markdown_token {
                Token::Plaintext(plaintext_token) => plaintext_token.push_str(&s),
                _ => t.push(Token::Plaintext(s)),
            }
        }
        None => t.push(Token::Plaintext(s)),
    }
}


fn consume_while_case_holds(char_iter: &mut std::iter::Peekable<std::str::Chars>, func: &dyn Fn(&char) -> bool) -> String{
    let mut s = String::new();
    while char_iter.peek().is_some() && func(char_iter.peek().unwrap()) {
        s.push(char_iter.next().unwrap());
    }
    s
}

fn consume_until_tail_is(char_iter: &mut std::iter::Peekable<std::str::Chars>, tail: &str) -> String{
    let mut s = String::new();
    while char_iter.peek().is_some() && !s.ends_with(tail) {
        s.push(char_iter.next().unwrap());
    }
    s
}

pub(crate) fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let hashes = consume_while_case_holds(char_iter, &|c| c == &'#');
    if char_iter.next_if_eq(&' ').is_none(){
        return Err(ParseError{content: hashes});
    }
    let level = std::cmp::min(6, hashes.len() as u8);
    let mut line = consume_while_case_holds(char_iter, &|c| c != &'\n');
    if line.contains("{#") && 
        line.contains('}') {
            let heading = line.chars().take_while(|c| c != &'{').collect::<String>();
            line.replace_range(..heading.len(), "");
            line.retain(|c| c != '{' && c != '#' && c != '}');
            return Ok(Token::Header(level, heading.trim().to_string(), Some(line)));
        }
    return Ok(Token::Header(level, line, None));
}

pub(crate) fn lex_asterisk_underscore(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let asterunds = consume_while_case_holds(char_iter, &|c| c == &'*' || c == &'_');
    if asterunds.len() == 1 && char_iter.next_if_eq(&' ').is_some(){
        let s = consume_while_case_holds(char_iter, &|c| c != &'\n');
        char_iter.next();
        return Ok(Token::UnorderedListEntry(s))
    }
    if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&'\n'){
        return Ok(Token::HorizontalRule)
    }
    match asterunds.len() {
        1 => {
            let s = consume_while_case_holds(char_iter, &|c| c != &'*' && c != &'_');
            if char_iter.peek() != Some(&'*') || char_iter.peek() != Some(&'_'){
                char_iter.next();
                return Ok(Token::Italic(s))
            } else {
                return Err(ParseError{content: format!("{}{}", asterunds, s)});
            }
        },
        2 => {
            let s = consume_while_case_holds(char_iter, &|c| c != &'*' && c != &'_');
            let trailing_astunds = consume_while_case_holds(char_iter, &|c| c == &'*' || c == &'_');
            if trailing_astunds.len() == 2 {
                return Ok(Token::Bold(s))
            } else {
                return Err(ParseError{content: format!("{}{}{}", asterunds, s, trailing_astunds)});
            }
        },
        3 => {
            let s = consume_while_case_holds(char_iter, &|c| c != &'*' && c != &'_');
            let trailing_astunds = consume_while_case_holds(char_iter, &|c| c == &'*' || c == &'_');
            if trailing_astunds.len() == 3 {
                return Ok(Token::BoldItalic(s))
            } else {
                return Err(ParseError{content: format!("{}{}{}", asterunds, s, trailing_astunds)});
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

pub(crate) fn lex_spaces(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError>{
    let spaces = consume_while_case_holds(char_iter, &|c| c == &' ');
    // Case 1: space in text => return char
    if spaces.len() == 1 {
        return Err(ParseError{content: spaces})
    }
    // Case 2: two or more spaces followed by \n => line break
    if char_iter.peek() == Some(&'\n'){
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

pub(crate) fn lex_backticks(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let leading_ticks = consume_while_case_holds(char_iter, &|c| c == &'`');
    let mut lang = "plaintext".to_string();
    if leading_ticks.len() != 1 && leading_ticks.len() != 3{
        return Err(ParseError{content: format!("{}",leading_ticks)})
    }
    if leading_ticks.len() == 1 {
        let s = consume_while_case_holds(char_iter, &|c| c != &'`');
        let trailing_ticks = consume_while_case_holds(char_iter, &|c| c == &'`');
        if leading_ticks.len() != trailing_ticks.len() {
            return Err(ParseError{content: format!("{}{}{}",leading_ticks, s, trailing_ticks)}) 
        } else {
            return Ok(Token::Code(s))
        }
    }
    // leading_ticks.len() == 3. Check for lang
    if char_iter.peek() != Some(&'\n') {
        lang = consume_while_case_holds(char_iter, &|c| c != &'\n');
        char_iter.next();
    } else {
        char_iter.next();
    }
    let s = consume_while_case_holds(char_iter, &|c| c != &'`');
    let trailing_ticks = consume_while_case_holds(char_iter, &|c| c == &'`');
    if leading_ticks.len() != trailing_ticks.len() {
        return Err(ParseError{content: format!("{}{}{}",leading_ticks, s, trailing_ticks)}) 
    } else {
        return Ok(Token::CodeBlock(s, lang))
    }
}

pub(crate) fn lex_newlines(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let newlines = consume_while_case_holds(char_iter, &|c| c == &'\n');
    match newlines.len() {
        0..=1 => return Err(ParseError{content: newlines}),
        _ => return Ok(Token::Newline)
    }
}

pub(crate) fn lex_blockquotes(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let right_arrows = consume_while_case_holds(char_iter, &|c| c == &'>');
    match char_iter.peek() {
        Some(&' ') => {char_iter.next();},
        _ => {return Err(ParseError{content: right_arrows})}
    }
    let s = consume_while_case_holds(char_iter, &|c| c != &'\n');
    char_iter.next_if_eq(&'\n');
    Ok(Token::BlockQuote(right_arrows.len() as u8, s))
}

pub(crate) fn lex_images(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    if char_iter.peek() != Some(&'!'){
        return Err(ParseError{content: "".to_string()})
    }
    char_iter.next();
    let link_result = lex_links(char_iter);
    match link_result {
        Err(e) => return Err(e),
        Ok(Token::Link(link, title, _)) => return Ok(Token::Image(link, title)),
        _ => return Err(ParseError{content: "Non link token returned from lex_links".to_string()})
    }
}

pub(crate) fn lex_links(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    if char_iter.peek() != Some(&'[') {
        return Err(ParseError{content: "".to_string()})
    }
    char_iter.next();
    let title = consume_while_case_holds(char_iter, &|c| c != &']');
    if char_iter.peek() != Some(&']') {
        return Err(ParseError{content: "[".to_string()+&title})
    }
    char_iter.next();
    // Parse footnotes big and small
    if title.starts_with("^") && char_iter.peek() == Some(&':') {
        char_iter.next();
        let ref_id = title.strip_prefix("^").unwrap_or("");
        let mut note_text = String::new();
        loop {
           note_text.push_str(&consume_while_case_holds(char_iter, &|c| c != &'\n'));
           char_iter.next();
           if char_iter.peek() != Some(&' ') && char_iter.peek() != Some(&'\t') {
            break;
           }
           if char_iter.peek() == Some(&'\t') {
            char_iter.next();
            note_text.push('\n');
            continue;
           }
           if char_iter.peek() == Some(&' ') {
            let spaces = consume_while_case_holds(char_iter, &|c| c == &' ');
            match spaces.len() {
                2 | 4 => {note_text.push('\n');},
                _ => {return Err(ParseError{content: "[^".to_string()+&ref_id.to_string()+&"]:".to_string()+&note_text+&spaces})},
            }
            continue
           }
           break;
        }
        if ref_id.contains(char::is_whitespace){
            return Err(ParseError{content: "[^".to_string()+&ref_id.to_string()+&"]:".to_string()+&note_text})
        }
        return Ok(Token::Footnote(ref_id.to_string(), note_text.trim_start().to_string()));
    }
    if char_iter.peek() != Some(&'(') {
        return Err(ParseError{content: "[".to_string()+&title+"]"})
    }
    char_iter.next();
    let link = consume_while_case_holds(char_iter, &|c| c != &')' && c != &' ');
    if char_iter.peek() != Some(&')') && char_iter.peek() != Some(&' ') {
        return Err(ParseError{content: "[".to_string()+&title+&"]".to_string()+&link})
    }
    if char_iter.peek() == Some(&')') {
        char_iter.next();
        return Ok(Token::Link(link, Some(title), None));
    }
    if char_iter.peek() == Some(&' ') {
        let hover = consume_while_case_holds(char_iter, &|c| c != &')');
        char_iter.skip_while(|c| c != &'\n').next();
        return Ok(Token::Link(link, Some(title), Some(hover)));
    }
    Err(ParseError{content: "".to_string()})
}

pub(crate) fn lex_side_carrot(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    match char_iter.peek() {
        Some(&'<') => {
            char_iter.next();
            let s = consume_while_case_holds(char_iter, &|c| c != &'>');
            match char_iter.peek(){
                Some(&'>') if s != "details" => {
                    char_iter.next();
                    return Ok(Token::Link(s, None, None))
                },
                Some(&'>') if s == "details" => {
                    char_iter.next();
                    if !char_iter.next_if_eq(&'\n').is_some(){
                        return Err(ParseError{content: s});
                    }
                    return parse_details(char_iter)
                },
                _ => {
                    return Err(ParseError{content: s});
                }
            }
        }
        _ => return Err(ParseError{content: "".to_string()})
    }
}

pub(crate) fn lex_plus_minus(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let s = consume_while_case_holds(char_iter, &|c| c == &'-');
    match s.len() {
        3..=usize::MAX => { return Ok(Token::HorizontalRule)},
        2 => {return Ok(Token::Plaintext(s))},
        1 => {},
        _ => {return Err(ParseError{content: "negative string length".to_string()})},
    }
    let line = consume_while_case_holds(char_iter, &|c| c != &'\n');
    if line.starts_with(" [ ] ") {
        return Ok(Token::TaskListItem(TaskBox::Unchecked,line.strip_prefix(" [ ] ").unwrap_or("").to_string()))
    } else if line.starts_with(" [x] ") {
        return Ok(Token::TaskListItem(TaskBox::Checked,line.strip_prefix(" [x] ").unwrap_or("").to_string()))
    } else if line.starts_with(" [X] ") {
        return Ok(Token::TaskListItem(TaskBox::Checked,line.strip_prefix(" [X] ").unwrap_or("").to_string()))
    } else {
        return Ok(Token::Plaintext(s+&line))
    }
}

pub(crate) fn lex_numbers(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let c = char_iter.next().unwrap();
    match char_iter.peek() {
        Some('.') => {
            let dot = char_iter.next().unwrap();
            if char_iter.peek() != Some(&' '){
                return Err(ParseError{content: format!("{}{}",c,dot)})
            }
            char_iter.next();
            let s = consume_while_case_holds(char_iter, &|c| c != &'\n');
            return Ok(Token::OrderedListEntry(s))
        },
        _ => return Err(ParseError{content: c.to_string()})
    }
}

pub(crate) fn lex_tilde(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut lead_tildes = char_iter.next().unwrap().to_string();
    if char_iter.peek() != Some(&'~') {
        return Err(ParseError{content: lead_tildes})
    }
    lead_tildes.push(char_iter.next().unwrap());
    if char_iter.peek() == Some(&'~') || char_iter.peek() == None {
        return Err(ParseError{content: lead_tildes})
    }
    let s = consume_while_case_holds(char_iter, &|c| c != &'~');
    let tail_tildes = char_iter.next().unwrap().to_string();
    if char_iter.peek() != Some(&'~') {
        return Err(ParseError{content: format!("{}{}{}", lead_tildes, s, tail_tildes)})
    } else {
        char_iter.next();
        return Ok(Token::Strikethrough(s));
    }
}

fn parse_details(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError>{
    let mut summary_line = consume_while_case_holds(char_iter, &|c| c != &'\n');
    if !summary_line.starts_with("<summary") || !summary_line.ends_with("</summary>") {
        return Err(ParseError{content: summary_line});
    }
    summary_line = summary_line.strip_prefix("<summary").unwrap_or("").to_string();
    summary_line = summary_line.strip_suffix("</summary>").unwrap_or("").to_string();
    match summary_line.chars().nth(0){
        Some('>') => {summary_line.clear()},
        Some(' ') => {
            if !summary_line.starts_with(" markdown=\"span\">"){
                return Err(ParseError{content:format!("{}{}{}","<summary" ,summary_line, "</summary>")})
            }
            summary_line = summary_line.strip_prefix(" markdown=\"span\">").unwrap_or("").to_string();
        },
        Some(c) => {
            return Err(ParseError{content:format!("{}{}{}{}","<summary", c,summary_line, "</summary>")})
        },
        None => return Err(ParseError{content:format!("{}{}{}","<summary" ,summary_line, "</summary>")})
    }
    let mut remaining_text = consume_until_tail_is(char_iter, "</details>");
    if remaining_text.contains("<details>") {
        let mut opens = remaining_text.matches("<details>").count();
        let mut closes = remaining_text.matches("</details>").count();
        while opens == closes {
            remaining_text = remaining_text+&consume_until_tail_is(char_iter, "</details>");
            opens = remaining_text.matches("<details>").count();
            closes = remaining_text.matches("</details>").count();
        }
    }
    let inner_tokens = crate::lex(remaining_text.strip_suffix("</details>").unwrap_or(""));
    Ok(Token::Detail(summary_line, inner_tokens))
}

pub(crate) fn lex_pipes(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut lines = Vec::new();
    while char_iter.next_if_eq(&'|') == Some('|') {
        lines.push(consume_while_case_holds(char_iter, &|c| c != &'\n'));
        char_iter.next();
    }
    if lines.len() < 3 {
        return Err(ParseError{content: lines.join("\n")})
    }
    if !lines.iter().all(|l| l.matches("|").count() == lines[0].matches("|").count()) {
        return Err(ParseError{content: lines.join("\n")})
    }
    let headings: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim().to_string())
        .collect();
    let alignments: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| 
            {
            match (x.trim().to_string().starts_with(":"), x.trim().to_string().ends_with(":")) {
                (true, false) => Alignment::Left,
                (true, true) => Alignment::Center,
                (false, true) => Alignment::Right,
                _ => Alignment::Left,
            }}
        )
        .collect();
    let mut rows = Vec::new();
    for l in lines.iter() {
        let elements: Vec<String> = l.split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim().to_string())
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