use mini_markdown::iter::MiniIter;

#[test]
fn peek_does_not_advance(){
    let some_text = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.next());   
}

#[test]
fn peek_does_not_advance_utf(){
    let some_text = "الْأَ";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("ا"), some_text_iter.peek());
    assert_eq!(Some("ا"), some_text_iter.peek());
    assert_eq!(Some("ا"), some_text_iter.next());   
}

#[test]
fn modern_standard_arabic_test(){
    let some_text = "الْﺦﷺأَ"; // لْ is a weird character. 2 bytes are valid for the base and two more add the little circle on top.
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("الْﺦﷺأَ"), some_text_iter.consume_line_ahead());
    assert_eq!(None, some_text_iter.next());
}

#[test]
fn consume_until_end_consumes_full_string() {
    let some_text = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("this is some plaintext"), some_text_iter.consume_until_end());
    assert_eq!(None, some_text_iter.next());
}

#[test]
fn next_advances_utf_correctly() {
    let some_text = "اa木b겅c€d𐍈e";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("ا"), some_text_iter.next());
    assert_eq!(Some("a"), some_text_iter.peek());
    assert_eq!(Some("a"), some_text_iter.next());
    assert_eq!(Some("木"), some_text_iter.peek());
    assert_eq!(Some("木"), some_text_iter.next());
    assert_eq!(Some("b"), some_text_iter.next());
    assert_eq!(Some("겅"), some_text_iter.peek());
    assert_eq!(Some("겅"), some_text_iter.next());
    assert_eq!(Some("c"), some_text_iter.next());
    assert_eq!(Some("€"), some_text_iter.next());
    assert_eq!(Some("d"), some_text_iter.peek());
    assert_eq!(Some("d"), some_text_iter.next());
    assert_eq!(Some("𐍈"), some_text_iter.next());
    assert_eq!(Some("e"), some_text_iter.next());
}

#[test]
fn test_slashes() {
    let slashes = "¯\\\\\\\\\\¯";
    let mut slash_iter = MiniIter::new(&slashes);
    assert_eq!(Some("¯"), slash_iter.peek());
    assert_eq!(Some("¯"), slash_iter.next());
    assert_eq!(Some("\\"), slash_iter.peek());
    assert_eq!(Some("\\\\\\\\\\"), slash_iter.consume_while_case_holds(&|c| c == "\\"));
    assert_eq!(Some("¯"), slash_iter.peek());
    assert_eq!(Some("¯"), slash_iter.next());
    assert_eq!(None, slash_iter.next());
}

#[test]
fn general_iter_test(){
    let some_text = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.next());
    assert_eq!(Some("his"), some_text_iter.consume_while_case_holds(&|c| c != " "));
    assert_eq!(Some(" is some plain"), some_text_iter.consume_until_tail_is("plain"));
    assert_eq!(Some("text"), some_text_iter.consume_until_end());
    assert_eq!(None, some_text_iter.next());

    let other_text = "jkfsgbkfgbdklfdsbh gkhsdfbg <details> and more chars";
    let mut other_text_iter = MiniIter::new(&other_text);
    assert_eq!(Some("jkfsgbkfgbdklfdsbh gkhsdfbg <details>"), other_text_iter.consume_until_tail_is("<details>"));
    assert_eq!(Some(" and more chars") ,other_text_iter.consume_until_end());
    assert_eq!(None, other_text_iter.peek());
}

#[test]
fn consume_peek_line_test(){
    let some_text = "this is some plaintext in a line\nAnd a new line with more content";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.consume_line_ahead());
    assert_ne!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("And a new line with more content"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("And a new line with more content"), some_text_iter.consume_line_ahead());
    assert_eq!(None, some_text_iter.peek_line_ahead());
}

#[test]
fn test_degenerate_newlines() {
    let some_text = "\n\n\n\n\nfoo\n";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("foo\n"), some_text_iter.consume_line_ahead());
    assert_eq!(None, some_text_iter.consume_line_ahead());
}

#[test]
fn test_mixed_chars() {
    let some_text = "  - foo\n\n\tbar\n";
    let mut some_text_iter = MiniIter::new(&some_text);
    assert_eq!(Some("  - foo\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
    assert_eq!(Some("\tbar\n"), some_text_iter.consume_line_ahead());
}




