pub mod lexer;
use crate::lexer::lex;

#[cfg(test)]
mod tests {
    use super::*;
    
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