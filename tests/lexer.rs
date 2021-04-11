use mini_markdown::lex;
use mini_markdown::lexer::Token;

#[test]
fn test_lex() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", vec![Token::Header(1, "Heading level 1".to_string())]),
        ("## Heading level 2", vec![Token::Header(2, "Heading level 2".to_string())]),
        ("### Heading level 3", vec![Token::Header(3, "Heading level 3".to_string())]),
        ("#### Heading level 4", vec![Token::Header(4, "Heading level 4".to_string())]),
        ("##### Heading level 5", vec![Token::Header(5, "Heading level 5".to_string())]),
        ("###### Heading level 6", vec![Token::Header(6, "Heading level 6".to_string())]),
        ("####### Invalid Heading level 7", vec![Token::Header(6, "Invalid Heading level 7".to_string())]), 
    ]);
    tests.extend(vec![
        ("I just love **bold text**.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love __bold text__.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love *_bold text*_.", vec![Token::Plaintext("I just love ".to_string()), Token::Bold("bold text".to_string()), Token::Plaintext(".".to_string())]),
    ]);
    tests.extend(vec![
        ("I just love *italic text*.", vec![Token::Plaintext("I just love ".to_string()), Token::Italic("italic text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love _italic text_.", vec![Token::Plaintext("I just love ".to_string()), Token::Italic("italic text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love *italic text_.", vec![Token::Plaintext("I just love ".to_string()), Token::Italic("italic text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just\n love *italic\n text_.", vec![Token::Plaintext("I just\n love ".to_string()), Token::Italic("italic\n text".to_string()), Token::Plaintext(".".to_string())]),
    ]);
    tests.extend(vec![
        ("I just love ***bold italic text***.", vec![Token::Plaintext("I just love ".to_string()), Token::BoldItalic("bold italic text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love ___bold italic text___.", vec![Token::Plaintext("I just love ".to_string()), Token::BoldItalic("bold italic text".to_string()), Token::Plaintext(".".to_string())]),
        ("I just love _*_bold italic text*_*.", vec![Token::Plaintext("I just love ".to_string()), Token::BoldItalic("bold italic text".to_string()), Token::Plaintext(".".to_string())]),
    ]);
    tests.extend(vec![
        ("* unodered list\n", vec![Token::UnorderedListEntry("unodered list".to_string())]),
        ("* unodered list\n* with two\n", vec![Token::UnorderedListEntry("unodered list".to_string()), Token::UnorderedListEntry("with two".to_string())]),
        ("* unodered list\n* with two\n* with three\n", vec![Token::UnorderedListEntry("unodered list".to_string()), Token::UnorderedListEntry("with two".to_string()), Token::UnorderedListEntry("with three".to_string())]),
    ]);
    tests.extend(vec![
        ("Some text _with italics_ in the same paragraph\n", vec![Token::Plaintext("Some text ".to_string()), Token::Italic("with italics".to_string()), Token::Plaintext(" in the same paragraph\n".to_string())]),
    ]);
    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}