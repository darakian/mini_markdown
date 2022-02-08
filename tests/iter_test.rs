use mini_markdown::iter::MiniIter;

#[test]
fn iter_test(){
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
}