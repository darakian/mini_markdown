use mini_markdown::sanitize;

#[test]
fn test_simple_tag_removal() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("<p>foobar <script src=123.com> text for context </script> </p> <junk>".to_string(), "&lt;p&gt;foobar &lt;script src=123.com&gt; text for context &lt;/script&gt; &lt;/p&gt; &lt;junk&gt;".to_string()),
        ("Regular text with no tags but maybe a \n or a \t or something".to_string(), "Regular text with no tags but maybe a \n or a \t or something".to_string()),
        ("And one more for the <p> money to test <!--- Comment> unbalanced tags <a>".to_string(), "And one more for the &lt;p&gt; money to test &lt;!--- Comment&gt; unbalanced tags &lt;a&gt;".to_string()),
        ("Unbalanced <script src=<p> <script> <a> <foo>test".to_string(), "Unbalanced &lt;script src=&lt;p&gt; &lt;script&gt; &lt;a&gt; &lt;foo&gt;test".to_string()),
        ("Nested <script src=<p> <script> <a> <foo>>test".to_string(), "Nested &lt;script src=&lt;p&gt; &lt;script&gt; &lt;a&gt; &lt;foo&gt;&gt;test".to_string()),
        ("".to_string(), "".to_string()),
    ]);

    for test in tests.iter_mut(){
        let untagged = sanitize(&mut test.0);
        assert_eq!(untagged, test.1);
    }
}