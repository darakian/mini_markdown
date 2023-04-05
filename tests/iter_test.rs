use mini_markdown::iter::MiniIter;

#[test]
fn peek_does_not_advance(){
    let some_text_iter = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text_iter);
    assert_eq!(Some("t"), some_text_iter.peek());
    assert_eq!(Some("t"), some_text_iter.peek());   
}

fn general_iter_test(){
    let some_text_iter = "this is some plaintext";
    let mut some_text_iter = MiniIter::new(&some_text_iter);
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
fn consume_peek_line_test(){
    let some_text_iter = "this is some plaintext in a line\nAnd a new line with more content";
    let mut some_text_iter = MiniIter::new(&some_text_iter);
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.consume_line_ahead());
    assert_ne!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
    assert_eq!(Some("And a new line with more content"), some_text_iter.peek_line_ahead());
    println!(">> {:?}", some_text_iter.peek_until_end());
    assert_eq!(Some("And a new line with more content"), some_text_iter.consume_line_ahead());
    println!(">> {:?}", some_text_iter.peek_until_end());

    assert_eq!(Some("this is some plaintext in a line\n"), some_text_iter.peek_line_ahead());
}







