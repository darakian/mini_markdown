use mini_markdown::lex;
use mini_markdown::lexer::{Token, TaskBox};

#[test]
fn test_lex() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", vec![Token::Header(1, "Heading level 1".to_string(), None)]),
        ("## Heading level 2", vec![Token::Header(2, "Heading level 2".to_string(), None)]),
        ("### Heading level 3", vec![Token::Header(3, "Heading level 3".to_string(), None)]),
        ("#### Heading level 4", vec![Token::Header(4, "Heading level 4".to_string(), None)]),
        ("##### Heading level 5", vec![Token::Header(5, "Heading level 5".to_string(), None)]),
        ("###### Heading level 6", vec![Token::Header(6, "Heading level 6".to_string(), None)]),
        ("####### Invalid Heading level 7", vec![Token::Header(6, "Invalid Heading level 7".to_string(), None)]), 
    ]);
    tests.extend(vec![
        ("# Heading level 1 {#Test label}", vec![Token::Header(1, "Heading level 1".to_string(), Some("Test label".to_string()))]),
        ("## Heading level 2 {#Test label}", vec![Token::Header(2, "Heading level 2".to_string(), Some("Test label".to_string()))]),
        ("### Heading level 3 {#Test label}", vec![Token::Header(3, "Heading level 3".to_string(), Some("Test label".to_string()))]),
        ("#### Heading level 4 {#Test label}", vec![Token::Header(4, "Heading level 4".to_string(), Some("Test label".to_string()))]),
        ("##### Heading level 5 {#Test label}", vec![Token::Header(5, "Heading level 5".to_string(), Some("Test label".to_string()))]),
        ("###### Heading level 6 {#Test label}", vec![Token::Header(6, "Heading level 6".to_string(), Some("Test label".to_string()))]),
        ("####### Invalid Heading level 7 {#Test label}", vec![Token::Header(6, "Invalid Heading level 7".to_string(), Some("Test label".to_string()))]), 
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
        ("Some text _with italics_ in the same paragraph", vec![Token::Plaintext("Some text ".to_string()), Token::Italic("with italics".to_string()), Token::Plaintext(" in the same paragraph".to_string())]),
        ("Text attributes _italic_, \n**bold**, `monospace`. Some implementations may use *single-asterisks* for italic text.", 
        vec![
            Token::Plaintext("Text attributes ".to_string()), 
            Token::Italic("italic".to_string()), 
            Token::Plaintext(", \n".to_string()),
            Token::Bold("bold".to_string()), 
            Token::Plaintext(", ".to_string()), 
            Token::Code("monospace".to_string()),
            Token::Plaintext(". Some implementations may use ".to_string()),
            Token::Italic("single-asterisks".to_string()), 
            Token::Plaintext(" for italic text.".to_string()),
        ])
    ]);
    tests.extend(vec![
        ("![alt](https://example.com/foo.jpeg)", vec![Token::Image("https://example.com/foo.jpeg".to_string(), Some("alt".to_string()))]),
        ("![alt]()", vec![Token::Image("".to_string(), Some("alt".to_string()))]),
        ("Some test text [^1]", vec![Token::Plaintext("Some test text [^1]".to_string())]),
        ("[^1]: First footnote", vec![Token::Footnote("1".to_string(), "First footnote".to_string())]),
        ("[^HUGE]: Big footnote", vec![Token::Footnote("HUGE".to_string(), "Big footnote".to_string())]),
        ("[^BORK ED]: Big footnote", vec![Token::Plaintext("[^BORK ED]: Big footnote".to_string())]),

    ]);
    tests.extend(vec![
        ("---", vec![Token::HorizontalRule]),
        ("-----", vec![Token::HorizontalRule]),
        ("--", vec![Token::Plaintext("--".to_string())]),
        ("- [ ] Unchecked box", vec![Token::TaskListItem(TaskBox::Unchecked, "Unchecked box".to_string())]),
        ("+ [ ] Unchecked box", vec![Token::TaskListItem(TaskBox::Unchecked, "Unchecked box".to_string())]),
        ("- [x] Checked box", vec![Token::TaskListItem(TaskBox::Checked, "Checked box".to_string())]),
        ("- [X] Also a checked box", vec![Token::TaskListItem(TaskBox::Checked, "Also a checked box".to_string())]),
        ("- [X]Not a checked box", vec![Token::UnorderedListEntry("[X]Not a checked box".to_string())]),
        ("- [X] A checked box\n- [X] Also a checked box", vec![Token::TaskListItem(TaskBox::Checked, "A checked box".to_string()), Token::Plaintext("\n".to_string()), Token::TaskListItem(TaskBox::Checked, "Also a checked box".to_string())]),
    ]);
    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}

#[test]
fn test_lex_plaintext() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("¯\\\\\\_(ツ)\\_/¯", vec![Token::Plaintext("¯\\_(ツ)_/¯".to_string())]),
        ("\\_test\\_", vec![Token::Plaintext("_test_".to_string())]),
        ("\\*escaping\\_", vec![Token::Plaintext("*escaping_".to_string())]),
        ("\\>controls\\<", vec![Token::Plaintext(">controls<".to_string())])
    ]);
    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}

#[test]
fn test_blockquote_lex() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("> ", vec![Token::BlockQuote(1, "".to_string())]),
        ("> \n>> text", vec![Token::BlockQuote(1, "".to_string()), Token::BlockQuote(2, "text".to_string())]),
        ("> text\n> \n>> more text", vec![Token::BlockQuote(1, "text".to_string()) ,Token::BlockQuote(1, "".to_string()), Token::BlockQuote(2, "more text".to_string())]),

    ]);

    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}

#[test]
fn test_footnote_lex() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("[^1]: Footnote #1", vec![Token::Footnote("1".to_string(), "Footnote #1".to_string())]),
        ("[^1]: Footnote #1\n  with a second line", vec![Token::Footnote("1".to_string(), "Footnote #1\nwith a second line".to_string())]),
        ("[^1]: Footnote #1\n\twith a second line", vec![Token::Footnote("1".to_string(), "Footnote #1\nwith a second line".to_string())]),
        ("[^1]: Footnote #1\n    with a second line", vec![Token::Footnote("1".to_string(), "Footnote #1\nwith a second line".to_string())]),
        ("[^1]: Footnote #1\n    with a second line\n\tand a third line", vec![Token::Footnote("1".to_string(), "Footnote #1\nwith a second line\nand a third line".to_string())]),
    ]);

    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}

#[test]
fn test_link_lex(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("another (See [Sewer Shark](https://en.wikipedia.org/wiki/Sewer_Shark)). Video", 
        vec![Token::Plaintext("another (See ".to_string()), Token::Link("https://en.wikipedia.org/wiki/Sewer_Shark".to_string(), Some("Sewer Shark".to_string()), None), Token::Plaintext("). Video".to_string())]),
        ("r [Distant Worlds](https://www.youtube.com/watch?v=yd3KYOei8o4) a",
        vec![Token::Plaintext("r ".to_string()), Token::Link("https://www.youtube.com/watch?v=yd3KYOei8o4".to_string(), Some("Distant Worlds".to_string()), None), Token::Plaintext(" a".to_string())])
    ]);

    for test in tests.iter(){
        let tokens = lex(test.0);
        assert_eq!(&tokens[..], &test.1[..]);
    }
}