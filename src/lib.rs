pub mod lexer;
use crate::lexer::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lex() {
        let heading_tests = vec![
            ("# Heading level 1", vec![Token::Header(1, "Heading level 1".to_string())]),
            ("## Heading level 2", vec![Token::Header(2, "Heading level 2".to_string())]),
            ("### Heading level 3", vec![Token::Header(3, "Heading level 3".to_string())]),
            ("#### Heading level 4", vec![Token::Header(4, "Heading level 4".to_string())]),
            ("##### Heading level 5", vec![Token::Header(5, "Heading level 5".to_string())]),
            ("###### Heading level 6", vec![Token::Header(6, "Heading level 6".to_string())]),
            ("####### Invalid Heading level 7", vec![Token::Header(6, "Invalid Heading level 7".to_string())]), 
        ];
        for test in heading_tests.iter(){
            let tokens = lex(test.0);
            assert_eq!(&tokens[..], &test.1[..]);
        }

        let bold_tests = vec![
            ("I just love **bold text**.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
            ("I just love __bold text__.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
            ("I just love *_bold text*_.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
        ];
        for test in bold_tests.iter(){
            let tokens = lex(test.0);
            assert_eq!(&tokens[..], &test.1[..]);
        }
    }
}

pub fn lex(source: &str) -> Vec<Token>{
    let mut char_iter = source.chars().peekable();
    let mut tokens = Vec::new();
    while char_iter.peek().is_some(){
        match char_iter.peek(){
            None => {return tokens},
            Some('#') => {
                let token = lex_heading(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('*') | Some('_') => {
                let token = lex_asterisk_underscore(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('-') | Some('+') => {
                let token = lex_plus_minus(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some(' ') => {
                let token = lex_spaces(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('`') => {
                let token = lex_backticks(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('\n') => {
                let token = lex_newlines(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('>') => {
                let token = lex_blockquotes(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('!') => {
                let token = lex_images(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('[') => {
                let token = lex_links(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some('<') => {
                let token = lex_easy_links(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            Some(_) => {
                let c = char_iter.next().unwrap();
                push_char(&mut tokens, c);
            },
        }
    }
    tokens
}

pub fn parse(tokens: Vec<Token>) -> String {
    let mut html = String::new();
    for token in tokens.iter(){
        match token {
            Token::Plaintext(t) => {html.push_str(format!("<p>{}</p>", t).as_str())},
            Token::Header(l, t) => {html.push_str(format!("<h{level}>{text}</{level}>", level=l, text=t).as_str())},
            // Token::UnorderedListEntry => {},
            // Token::OrderedListEntry => {},
            Token::Italic(t) => {html.push_str(format!("<em>{}</em>", t).as_str())},
            Token::Bold(t) => {html.push_str(format!("<strong>{}</strong>", t).as_str())},
            Token::BoldItalic(t) => {html.push_str(format!("<strong><em>{}</em></strong>", t).as_str())},
            Token::LineBreak => {html.push_str("<br>")},
            Token::HorizontalRule => {html.push_str("<hr>")},
            // Token::Tab => {},
            // Token::DoubleTab => {},
            Token::Code(t) | Token::EscapedCode(t) => {html.push_str(format!("<code>{}</code>", t).as_str())},
            // Token::BlockQuote(u8) => {},
            Token::Image(l, t) => html.push_str(format!("<img src=\"{link}\" alt=\"{text}\"", link=l, text=t).as_str()), // (Link, title)
            Token::Link(l, t, ht) => {
                match (t, ht){
                    (Some(t), Some(ht)) => html.push_str(format!("<a href=>\"{link}\" title=\"{hover}\">{text}", link=l, text=t, hover=ht).as_str()),
                    (Some(t), None) => html.push_str(format!("<a href=\"{link}\">{text}</a>", link=l, text=t).as_str()),
                    (None, Some(ht)) => html.push_str(format!("<a href=\"{link}\" title=\"{hover}\">{link}</a>", link=l, hover=ht).as_str()),
                    (None, None) => html.push_str(format!("<a href=\"{link}\">{link}</a>", link=l).as_str()),
                }
            },
            _ => {},
        }
    }
    html
}