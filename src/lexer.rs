#[derive(Debug, PartialEq)]
pub enum Token{
    Plaintext(String),
    Header(u8, String, Option<String>),
    UnorderedListEntry(String),
    OrderedListEntry(String),
    Italic(String),
    Bold(String),
    BoldItalic(String),
    LineBreak,
    ParagraphBreak,
    Newline,
    HorizontalRule,
    Tab,
    DoubleTab,
    Strikethrough(String),
    Code(String),
    CodeBlock(String, String),
    BlockQuote(u8, String),
    Image(String, String), // (Link, title)
    Link(String, Option<String>, Option<String>), //(link, title, hover text)
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


fn consume_until_next_is(char_iter: &mut std::iter::Peekable<std::str::Chars>, func: &dyn Fn(&char) -> bool) -> String{
    let mut s = String::new();
    while char_iter.peek().is_some() && func(char_iter.peek().unwrap()) {
        s.push(char_iter.next().unwrap());
    }
    s
}

use std::cmp;
pub(crate) fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let hashes = consume_until_next_is(char_iter, &|c| c == &'#');
    if char_iter.next_if_eq(&' ').is_none(){
        return Err(ParseError{content: hashes});
    }
    let level = cmp::min(6, hashes.len() as u8);
    let mut line = consume_until_next_is(char_iter, &|c| c != &'\n');
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
    let asterunds = consume_until_next_is(char_iter, &|c| c == &'*' || c == &'_');
    if asterunds.len() == 1 && char_iter.next_if_eq(&' ').is_some(){
        let s = consume_until_next_is(char_iter, &|c| c != &'\n');
        char_iter.next();
        return Ok(Token::UnorderedListEntry(s))
    }
    if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&'\n'){
        return Ok(Token::HorizontalRule)
    }
    match asterunds.len() {
        1 => {
            let s = consume_until_next_is(char_iter, &|c| c != &'*' && c != &'_');
            if char_iter.peek() != Some(&'*') || char_iter.peek() != Some(&'_'){
                char_iter.next();
                return Ok(Token::Italic(s))
            } else {
                return Err(ParseError{content: format!("{}{}", asterunds, s)});
            }
        },
        2 => {
            let s = consume_until_next_is(char_iter, &|c| c != &'*' && c != &'_');
            let trailing_astunds = consume_until_next_is(char_iter, &|c| c == &'*' || c == &'_');
            if trailing_astunds.len() == 2 {
                return Ok(Token::Bold(s))
            } else {
                return Err(ParseError{content: format!("{}{}{}", asterunds, s, trailing_astunds)});
            }
        },
        3 => {
            let s = consume_until_next_is(char_iter, &|c| c != &'*' && c != &'_');
            let trailing_astunds = consume_until_next_is(char_iter, &|c| c == &'*' || c == &'_');
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
    let spaces = consume_until_next_is(char_iter, &|c| c == &' ');
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
    let mut ticks = String::new();
    let mut lang = "plaintext".to_string();
    match char_iter.peek(){
        Some(&'`') => {
            ticks.push(char_iter.next().unwrap());
            if char_iter.peek() == Some(&'`'){
                ticks.push(char_iter.next().unwrap());
            }
            if char_iter.peek() == Some(&'`'){
                ticks.push(char_iter.next().unwrap());
            }
            if ticks.len() == 3 && char_iter.peek() == Some(&'\n'){
                char_iter.next().unwrap();
            } else if ticks.len() == 3 && char_iter.peek() != Some(&'\n') {
                lang.clear();
                while char_iter.peek().is_some() && char_iter.peek() != Some(&'\n'){
                    lang.push(char_iter.next().unwrap());
                }
                char_iter.next();
            }
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&'`') {
                s.push(char_iter.next().unwrap());
            }
            let mut end_ticks = String::new();
            while char_iter.peek() == Some(&'`') {
                end_ticks.push(char_iter.next().unwrap())
            }
            if ticks.len() != end_ticks.len(){
                return Err(ParseError{content: format!("{}{}{}",ticks,s,end_ticks)}) 
            } else {
                if ticks.len() == 1{
                    return Ok(Token::Code(s))
                } else {
                    return Ok(Token::CodeBlock(s, lang))
                }
            }
        },
        Some(_) => {
            return Err(ParseError{content: char_iter.next().unwrap().to_string()})
        }
        None => {return Err(ParseError{content: ticks})}
    }
}

pub(crate) fn lex_newlines(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    consume_until_next_is(char_iter, &|c| c == &'\n');
    return Ok(Token::Newline);
}

pub(crate) fn lex_blockquotes(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let right_arrows = consume_until_next_is(char_iter, &|c| c == &'>');
    match char_iter.peek() {
        Some(&' ') => {char_iter.next();},
        _ => {return Err(ParseError{content: right_arrows})}
    }
    let mut s = String::new();
    while char_iter.peek().is_some() && char_iter.peek() != Some(&'\n') {
        s.push(char_iter.next().unwrap());
    }
    Ok(Token::BlockQuote(right_arrows.len() as u8, s))
}

pub(crate) fn lex_images(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let bang = char_iter.next().unwrap();
    match char_iter.peek(){
        Some(&'[') => {
            char_iter.next();
            let mut title = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&']'){
                title.push(char_iter.next().unwrap());
            }
            match char_iter.peek() {
                Some(&']') => {
                    char_iter.next();
                    match char_iter.peek() {
                        Some(&'(') => {
                            char_iter.next();
                            let mut link = String::new();
                            while char_iter.peek().is_some() && char_iter.peek() != Some(&')'){
                                link.push(char_iter.next().unwrap());
                            }
                            match char_iter.peek() {
                                Some(')') => {
                                    return Ok(Token::Image(link, title))
                                },
                                _ => {
                                    let mut s = String::new();
                                    s.push(bang);
                                    s.push('[');
                                    s.push_str(&title);
                                    s.push(']');
                                    s.push_str(&link);
                                    return Err(ParseError{content: s});  
                                }
                            }
                        }, 
                        _ => {
                            let mut s = String::new();
                            s.push(bang);
                            s.push('[');
                            s.push_str(&title);
                            s.push(']');
                            return Err(ParseError{content: s});
                        }
                    }
                },
                _ => {
                    let mut s = String::new();
                    s.push(bang);
                    s.push('[');
                    s.push_str(&title);
                    return Err(ParseError{content: s});
                }
            }
        },
        _ => return Ok(Token::Plaintext(bang.to_string()))
    }
}

pub(crate) fn lex_links(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    match char_iter.peek(){
        Some(&'[') => {
            char_iter.next();
            let mut title = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&']'){
                title.push(char_iter.next().unwrap());
            }
            match char_iter.peek() {
                Some(&']') => {
                    char_iter.next();
                    match char_iter.peek() {
                        Some(&'(') => {
                            char_iter.next();
                            let mut link = String::new();
                            while char_iter.peek().is_some() && char_iter.peek() != Some(&')') && char_iter.peek() != Some(&' '){
                                link.push(char_iter.next().unwrap());
                            }
                            match char_iter.peek() {
                                Some(')') => {
                                    char_iter.skip_while(|c| c != &'\n').next();
                                    return Ok(Token::Link(link, Some(title), None));
                                },
                                Some(' ') => {
                                    let mut hover = String::new();
                                    while char_iter.peek().is_some() && char_iter.peek() != Some(&')'){
                                        hover.push(char_iter.next().unwrap());
                                    }
                                    match char_iter.peek() {
                                        Some(')') => {
                                            return Ok(Token::Link(link, Some(title), Some(hover)))
                                        },
                                        _ => {
                                            let mut s = String::new();
                                            s.push('[');
                                            s.push_str(&title);
                                            s.push(']');
                                            s.push_str(&link);
                                            s.push_str(&hover);
                                            return Err(ParseError{content: s});  
                                        }
                                    }
                                },
                                _ => {
                                    let mut s = String::new();
                                    s.push('[');
                                    s.push_str(&title);
                                    s.push(']');
                                    s.push_str(&link);
                                    return Err(ParseError{content: s});  
                                }
                            }
                        }, 
                        _ => {
                            let mut s = String::new();
                            s.push('[');
                            s.push_str(&title);
                            s.push(']');
                            return Err(ParseError{content: s});
                        }
                    }
                },
                _ => {
                    let mut s = String::new();
                    s.push('[');
                    s.push_str(&title);
                    return Err(ParseError{content: s});
                }
            }
        },
        _ => return Err(ParseError{content: "".to_string()})
    }
}

pub(crate) fn lex_side_carrot(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    match char_iter.peek() {
        Some(&'<') => {
            let s = char_iter.take_while(|c| c != &'>').collect::<String>();
            match char_iter.peek(){
                Some(&'>') => {
                    return Ok(Token::Link(s, None, None))
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
    let c = char_iter.next().unwrap();
    if char_iter.next_if_eq(&' ').is_some(){
        let mut s = String::new();
        while char_iter.peek().is_some() && char_iter.peek() != Some(&'\n'){
            s.push(char_iter.next().unwrap());
        }
        return Ok(Token::UnorderedListEntry(s))
    }
    match c {
        '-' => {
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() == Some(&'-'){
                s.push(char_iter.next().unwrap());
            }
            if s.chars().all(|x| x == '-') && char_iter.peek() == Some(&'\n'){
                return Ok(Token::HorizontalRule)
            } else {
                s.insert(0, c);
                return Ok(Token::Plaintext(s))
            }
        },
        _ => return Ok(Token::Plaintext(c.to_string())),
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
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&'\n'){
                s.push(char_iter.next().unwrap());
            }
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
    let mut s = String::new();
    while char_iter.peek().is_some() && char_iter.peek() != Some(&'~'){
        s.push(char_iter.next().unwrap());
    }
    let tail_tildes = char_iter.next().unwrap().to_string();
    if char_iter.peek() != Some(&'~') {
        return Err(ParseError{content: format!("{}{}{}", lead_tildes, s, tail_tildes)})
    } else {
        char_iter.next();
        return Ok(Token::Strikethrough(s));
    }
}