use mini_markdown::sanitize_display_text;

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
        let untagged = sanitize_display_text(&mut test.0);
        assert_eq!(untagged, test.1);
    }
}

use mini_markdown::render;
#[test]
fn test_image_xss(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("![Alt text](foo.jpeg)", "<img src=\"foo.jpeg\" alt=\"Alt text\">"),
        ("![Alt text]()", "<img src=\"data:,\" alt=\"Alt text\">"),
        ("![Alt text](   )", "<img src=\"data:,\" alt=\"Alt text\">"),
        ("![Alt text](javascript:alert(0))", "<img src=\"data:,\" alt=\"Alt text\">"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_link_xss() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("[text](javascript:alert(0))", "<a href=\"\">text</a>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}