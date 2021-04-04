#[derive(Debug)]
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
    Tab,
    DoubleTab,
    Code(String),
    EscapedCode(String),
    InlineNewline,
    BlockQuote(u8),
}

#[derive(Debug)]
pub struct ParseError{
    content: String,
}


/*
Tokens:
images ![text](link)
links [text](link) / [text](link "hover text")
link without text <link>

*/

pub fn lex(source: &str) -> (){
    let mut char_iter = source.chars().peekable();
    let mut tokens = Vec::new();
    while char_iter.peek().is_some(){
        // println!("Working {:?}", char_iter.peek());
        match char_iter.peek(){
            None => {return},
            Some('#') => {
                let token = lex_heading(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e),
                }
            },
            Some('*') | Some('_') => {
                let token = lex_asterisk_underscore(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e),
                }
            },
            Some('-') | Some('+') => {
                let c = char_iter.next().unwrap();
                if char_iter.next_if_eq(&' ').is_some(){
                    tokens.push(Token::UnorderedListEntry)
                } else {
                    push_char(&mut tokens, c);
                }
            },
            Some(' ') => {
                let token = lex_spaces(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('`') => {
                let token = lex_backticks(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            }
            Some('\n') => {
                let token = lex_newlines(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            }
            Some('>') => {
                let token = lex_blockquotes(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            }
            Some(_) => {
                let c = char_iter.next().unwrap();
                push_char(&mut tokens, c);
            },
        }
    }
    // for token in tokens.iter() {
    //     println!("Token: {:?}", token)
    // }
}

pub fn push_char(t: &mut Vec<Token>, c: char) {
    match t.last_mut() {
        Some(markdown_token) => {
            match markdown_token {
                Token::Plaintext(mp) => mp.push(c),
                _ => t.push(Token::Plaintext(c.to_string())),
            }
        }
        None => t.push(Token::Plaintext(c.to_string())),
    }
}

use std::cmp;
pub fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
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

pub fn lex_asterisk_underscore(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
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
        3 => return Ok(Token::BoldItalic),
        _ => return Err(ParseError{content: asterunds})
    }
}

pub fn lex_spaces(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError>{
    let mut spaces = char_iter.next().unwrap().to_string();
    // Case 1: space in text => return char
    if char_iter.peek() != Some(&' ') {
        return Ok(Token::Plaintext(spaces));
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

pub fn lex_backticks(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
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

pub fn lex_newlines(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut new_lines = char_iter.next().unwrap().to_string();
    // Case 1: newline in text => return char
    if char_iter.peek() != Some(&'\n') {
        return Ok(Token::InlineNewline);
    }
    // Glob new lines
    while char_iter.peek() == Some(&'\n'){
        new_lines.push(char_iter.next().unwrap())
    }
    Ok(Token::ParagraphBreak)
}

pub fn lex_blockquotes(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, ParseError> {
    let mut level = 0;
    while char_iter.next_if_eq(&'>').is_some() {
        level+=1;
    }
    Ok(Token::BlockQuote(level))
}