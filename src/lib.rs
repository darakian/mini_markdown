pub mod lexer;
use crate::lexer::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lex() {
        // let heading_tests = vec![
        //     ("# Heading level 1", vec![Token::Header(1), ]),
        //     ("## Heading level 2", "<h2>Heading level 2</h2>"),
        //     ("### Heading level 3", "<h3>Heading level 3</h3>"),
        //     ("#### Heading level 4", "<h4>Heading level 4</h4>"),
        //     ("##### Heading level 5", "<h5>Heading level 5</h5>"),
        //     ("###### Heading level 6", "<h6>Heading level 6</h6>"),
        //     ("####### Invalid Heading level 7", "<h6>Heading level 6</h6>"), 
        // 1
        // ];
        let mut heading_tests = Vec::new();
        heading_tests.push(
            ("# Heading level 1", 
            vec![Token::Header(1),Token::Plaintext("Heading level 1".to_string()),]
            )
        );
        for test in heading_tests.iter(){
            println!("Testing: {} -> {:?}", test.0, test.1);
            let tokens = lex(test.0);
            assert_eq!(&tokens[..], &test.1[..]);
            for token in tokens{
                println!("Token: {:?}", token)
            }
        }

        // let bold_tests = vec![
        //     ("I just love **bold text**.", "I just love <strong>bold text</strong>."),
        //     ("I just love __bold text__.", "I just love <strong>bold text</strong>."),
        //     ("I just love *_bold text*_.", "I just love <strong>bold text</strong>."),
        //     ("I just love\n\n\n _*bold text_*.", "I just love <strong>bold text</strong>."),
        // ];
        // for test in bold_tests.iter(){
        //     println!("Testing bold: {} -> {}", test.0, test.1);
        //     lex(test.0);
        // }
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
                let token = lex_plus_minus(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e),
                }
            },
            Some(' ') => {
                let token = lex_spaces(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => {
                        for c in e.content.chars(){push_char(&mut tokens, c)}
                        println!("{:?}", e)}
                }
            },
            Some('`') => {
                let token = lex_backticks(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('\n') => {
                let token = lex_newlines(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('>') => {
                let token = lex_blockquotes(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('!') => {
                let token = lex_images(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('[') => {
                let token = lex_links(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
                }
            },
            Some('<') => {
                let token = lex_easy_links(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => println!("{:?}", e)
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