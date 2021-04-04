#[derive(Debug, PartialEq)]
pub enum Token{
    Plaintext(String),
    Header(u8),
    UnorderedListEntry,
    OrderedListEntry,
    Italic,
    Bold,
    BoldItalic,
    ParagraphBreak,
    LineBreak,
    HorizontalRule,
    Tab,
    DoubleTab,
    Code(String),
    EscapedCode(String),
    InlineNewline,
    BlockQuote(u8),
    Image(String, String), // (Link, title)
    Link(String, Option<String>, Option<String>), //(link, title, hover text)
}

#[derive(Debug)]
pub(crate) struct ParseError{
    pub content: String,
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

use std::cmp;
pub(crate) fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut hashes = String::new();
    while char_iter.peek() == Some(&'#'){
        hashes.push(char_iter.next().unwrap());
    }
    match char_iter.peek(){
        Some(' ') => {
            char_iter.next();
            return Ok(Token::Header(cmp::min(6, hashes.len() as u8)));
        },
        _ => {Err(ParseError{content: hashes})}
    }
}

pub(crate) fn lex_asterisk_underscore(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut asterunds = String::new();
    if char_iter.peek() == Some(&'*') {
        asterunds.push(char_iter.next().unwrap());
        if char_iter.next_if_eq(&' ').is_some(){
            return Ok(Token::UnorderedListEntry)
        }
    }
    while char_iter.peek() == Some(&'*') || char_iter.peek() == Some(&'_'){
        asterunds.push(char_iter.next().unwrap());
    }
    match asterunds.len() {
        1 => return Ok(Token::Italic),
        2 => return Ok(Token::Bold),
        3 => {
            if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&'\n'){
                return Ok(Token::HorizontalRule)
            } else {
                return Ok(Token::BoldItalic)   
            }},
        _ => {
            if asterunds.chars().all(|x| x == '*') || asterunds.chars().all(|x| x == '_'){
                return Ok(Token::HorizontalRule)
            } else {
                return Err(ParseError{content: asterunds})
            }}
    }
}

pub(crate) fn lex_spaces(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError>{
    let mut spaces = char_iter.next().unwrap().to_string();
    // Case 1: space in text => return char
    if char_iter.peek() != Some(&' ') {
        return Err(ParseError{content: spaces})
    }
    // Glob spaces
    while char_iter.peek() == Some(&' '){
        spaces.push(char_iter.next().unwrap())
    }
    // Case 2: two or more spaces followed by \n => line break
    if char_iter.peek() == Some(&'\n'){
        return Ok(Token::LineBreak);
    }
    /* Cases:
    3. four spaces or a tab => code block
    3a. four spaces in a list => add paragraph item to prior list element
    4. eight spaces or two tabs => code block in list
    */
    match spaces.len(){
        4 => return Ok(Token::Tab),
        8 => return Ok(Token::DoubleTab),
        _ => {}
    }
    Err(ParseError{content: spaces})
}

pub(crate) fn lex_backticks(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut ticks = char_iter.next().unwrap().to_string();
    match char_iter.peek(){
        Some(&'`') => {
            ticks.push(char_iter.next().unwrap());
            let mut s = String::new();
            while char_iter.peek().is_some() {
                match char_iter.peek() {
                    Some(&'`') => {
                        ticks.push(char_iter.next().unwrap());
                        match char_iter.peek(){
                            Some(&'`') => return Ok(Token::EscapedCode(s)),
                            Some(_) => s.push('`'),
                            None => return Err(ParseError{content: s})
                        }
                    },
                    Some(_) => {s.push(char_iter.next().unwrap())},
                    None => {return Err(ParseError{content: s})}
                }
            }
            return  Err(ParseError{content: s})
        },
        Some(_) => {
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&'`'){
                s.push(char_iter.next().unwrap());
            }
            if char_iter.peek() == Some(&'`'){
                char_iter.next();
                return Ok(Token::Code(s))
            } else {
                return  Err(ParseError{content: s})
            }
        }
        None => {return Err(ParseError{content: ticks})}
    }
}

pub(crate) fn lex_newlines(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut new_lines = char_iter.next().unwrap().to_string();
    if char_iter.peek() != Some(&'\n') {
        return Ok(Token::InlineNewline);
    }
    while char_iter.peek() == Some(&'\n'){
        new_lines.push(char_iter.next().unwrap())
    }
    Ok(Token::ParagraphBreak)
}

pub(crate) fn lex_blockquotes(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut level = 0;
    while char_iter.next_if_eq(&'>').is_some() {
        level+=1;
    }
    Ok(Token::BlockQuote(level))
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
                                    return Ok(Token::Link(link, Some(title), None))
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

pub(crate) fn lex_easy_links(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    match char_iter.peek() {
        Some(&'<') => {
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&'>'){
                s.push(char_iter.next().unwrap());
            }
            match char_iter.peek(){
                Some(&'>') => {
                    return Ok(Token::Link(s, None, None))
                },
                _ => {
                    let mut e = String::new();
                    e.push('<');
                    e.push_str(&s);
                    return Err(ParseError{content: e});
                }
            }
        }
        _ => return Err(ParseError{content: "".to_string()})
    }
}

pub(crate) fn lex_plus_minus(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let c = char_iter.next().unwrap();
    if char_iter.next_if_eq(&' ').is_some(){
        return Ok(Token::UnorderedListEntry)
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