use mini_markdown::iter::MiniIter;

#[test]
fn peek_does_not_advance(){
    let some_text_iter = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text_iter);
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.next());   
}

#[test]
fn peek_does_not_advance_utf(){
    let some_text_iter = "الْأَ";
    let mut some_text_iter = MiniIter::new(&some_text_iter);
    assert_eq!(Some("ا"), some_text_iter.peek());
    assert_eq!(Some("ا"), some_text_iter.peek());
    assert_eq!(Some("ا"), some_text_iter.next());   
}

#[test]
fn consume_until_end_consumes_full_string() {
    let text_iter = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&text_iter);
    assert_eq!(Some("this is some plaintext"), some_text_iter.consume_until_end());
    assert_eq!(None, some_text_iter.next());
}

#[test]
fn next_advances_utf_correctly() {
    let text_iter = "اa木b겅c€d";
    let mut some_text_iter = MiniIter::new(&text_iter);
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
    assert_eq!(Some("d"), some_text_iter.next());


}

#[test]
fn test_slashes() {
    let slashes = "¯\\\\\\\\\\¯";
    let mut slash_iter = MiniIter::new(&slashes);
    println!(">> {:?}", slash_iter.peek_until_end());
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
    let text_iter = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&text_iter);
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
    let text_iter = "this is some plaintext in a line\nAnd a new line with more content";
    let mut some_text_iter = MiniIter::new(&text_iter);
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.consume_line_ahead());
    assert_ne!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("And a new line with more content"), some_text_iter.peek_line_ahead());
    println!(">> {:?}", some_text_iter.peek_until_end());
    assert_eq!(Some("And a new line with more content"), some_text_iter.consume_line_ahead());
    println!(">> {:?}", some_text_iter.peek_until_end());
    assert_eq!(None, some_text_iter.peek_line_ahead());
}







