#[cfg(test)]
mod tests {
    use crate::{lex};
    
    #[test]
    fn test_lex() {
        let heading_tests = vec![
            ("# Heading level 1", "<h1>Heading level 1</h1>"),
            ("## Heading level 2", "<h2>Heading level 2</h2>"),
            ("### Heading level 3", "<h3>Heading level 3</h3>"),
            ("#### Heading level 4", "<h4>Heading level 4</h4>"),
            ("##### Heading level 5", "<h5>Heading level 5</h5>"),
            ("###### Heading level 6", "<h6>Heading level 6</h6>"),
            ("####### Invalid Heading level 7", "<h6>Heading level 6</h6>"),
        ];
        for test in heading_tests.iter(){
            println!("Testing: {} -> {}", test.0, test.1);
            lex(test.0);
        }

        let bold_tests = vec![
            ("I just love **bold text**.", "I just love <strong>bold text</strong>."),
            ("I just love __bold text__.", "I just love <strong>bold text</strong>."),
            ("I just love *_bold text*_.", "I just love <strong>bold text</strong>."),
            ("I just love\n\n\n _*bold text_*.", "I just love <strong>bold text</strong>."),
        ];
        for test in bold_tests.iter(){
            // println!("Testing bold: {} -> {}", test.0, test.1);
            lex(test.0);
        }
    }
}

#[derive(Debug)]
pub enum MarkdownToken{
    MarkdownPlaintext(String),
    MarkdownBeginHeader(u8),
    MarkdownUnorderedListEntry,
    MarkdownOrderedListEntry,
    MarkdownItalic,
    MarkdownBold,
    MarkdownBoldItalic,
    MarkdownParagraphBreak,
    MarkdownLineBreak,
    MarkdownTab,
    MarkdownDoubleTab,
    MarkdownCode(String),
    MarkdownEscapedCode(String),
}

#[derive(Debug)]
pub struct MarkdownParseError{
    content: String,
}


/*
Tokens

>, >>, ... Quoteblocks


images ![text](link)
`x` code
``x`` escape backticks in x
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
                    tokens.push(MarkdownToken::MarkdownUnorderedListEntry)
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
                char_iter.next();
                tokens.push(MarkdownToken::MarkdownParagraphBreak)},
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

pub fn push_char(t: &mut Vec<MarkdownToken>, c: char) {
    match t.last_mut() {
        Some(markdown_token) => {
            match markdown_token {
                MarkdownToken::MarkdownPlaintext(mp) => mp.push(c),
                _ => t.push(MarkdownToken::MarkdownPlaintext(c.to_string())),
            }
        }
        None => t.push(MarkdownToken::MarkdownPlaintext(c.to_string())),
    }
}

use std::cmp;
pub fn lex_heading(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<MarkdownToken, MarkdownParseError>{
    let mut hashes = String::new();
    while char_iter.peek() == Some(&'#'){
        hashes.push(char_iter.next().unwrap());
    }
    match char_iter.peek(){
        Some(' ') => {
            char_iter.next();
            return Ok(MarkdownToken::MarkdownBeginHeader(cmp::min(6, hashes.len() as u8)));
        },
        _ => {Err(MarkdownParseError{content: hashes})}
    }
}

pub fn lex_asterisk_underscore(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<MarkdownToken, MarkdownParseError>{
    let mut asterunds = String::new();
    if char_iter.peek() == Some(&'*') {
        asterunds.push(char_iter.next().unwrap());
        if char_iter.next_if_eq(&' ').is_some(){
            return Ok(MarkdownToken::MarkdownUnorderedListEntry)
        }
    }
    while char_iter.peek() == Some(&'*') || char_iter.peek() == Some(&'_'){
        asterunds.push(char_iter.next().unwrap());
    }
    match asterunds.len() {
        1 => return Ok(MarkdownToken::MarkdownItalic),
        2 => return Ok(MarkdownToken::MarkdownBold),
        3 => return Ok(MarkdownToken::MarkdownBoldItalic),
        _ => return Err(MarkdownParseError{content: asterunds})
    }
}

pub fn lex_spaces(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<MarkdownToken, MarkdownParseError>{
    let mut spaces = char_iter.next().unwrap().to_string();
    // Case 1: space in text => return char
    if char_iter.peek() != Some(&' ') {
        return Ok(MarkdownToken::MarkdownPlaintext(spaces));
    }
    // Glob spaces
    while char_iter.peek() == Some(&' '){
        spaces.push(char_iter.next().unwrap())
    }
    // Case 2: two or more spaces followed by \n => line break
    if char_iter.peek() == Some(&'\n'){
        return Ok(MarkdownToken::MarkdownLineBreak);
    }
    /* Cases:
    3. four spaces or a tab => code block
    3a. four spaces in a list => add paragraph item to prior list element
    4. eight spaces or two tabs => code block in list
    */
    match spaces.len(){
        4 => return Ok(MarkdownToken::MarkdownTab),
        8 => return Ok(MarkdownToken::MarkdownDoubleTab),
        _ => {}
    }
    Err(MarkdownParseError{content: spaces})
}

pub fn lex_backticks(char_iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<MarkdownToken, MarkdownParseError>{
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
                            Some(&'`') => return Ok(MarkdownToken::MarkdownEscapedCode(s)),
                            Some(_) => s.push('`'),
                            None => return Err(MarkdownParseError{content: s})
                        }
                    },
                    Some(_) => {s.push(char_iter.next().unwrap())},
                    None => {return Err(MarkdownParseError{content: s})}
                }
            }
            return  Err(MarkdownParseError{content: s})
        },
        Some(_) => {
            let mut s = String::new();
            while char_iter.peek().is_some() && char_iter.peek() != Some(&'`'){
                s.push(char_iter.next().unwrap());
            }
            if char_iter.peek() == Some(&'`'){
                char_iter.next();
                return Ok(MarkdownToken::MarkdownCode(s))
            } else {
                return  Err(MarkdownParseError{content: s})
            }
        }
        None => {return Err(MarkdownParseError{content: ticks})}
    }
}