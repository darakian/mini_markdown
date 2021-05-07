use mini_markdown::remove_tags;

#[test]
fn test_simple_tag_removal() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("<p>foobar <script src=123.com> text for context </script> </p> <junk>".to_string(), "foobar  text for context   ".to_string()),
    ]);

    for test in tests.iter_mut(){
        let untagged = remove_tags(&mut test.0);
        assert_eq!(untagged, test.1);
    }
}