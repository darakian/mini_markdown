#[test]
fn test_simple_tag_injection() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("foobar <script src=123.com> text for context </script> <junk>".to_string(), 
            "<p>foobar <a href=\"\" referrerpolicy=\"no-referrer\"></a> text for context <a href=\"/script\" referrerpolicy=\"no-referrer\">/script</a> <a href=\"junk\" referrerpolicy=\"no-referrer\">junk</a></p>\n".to_string()),
        ("<SCRIPT SRC=http://xss.rocks/xss.js></SCRIPT>".to_string(), 
            "<a href=\"\" referrerpolicy=\"no-referrer\"></a><a href=\"/SCRIPT\" referrerpolicy=\"no-referrer\">/SCRIPT</a>".to_string()),
    ]);

    for test in tests.iter_mut(){
        let html = render(&test.0);
        assert_eq!(test.1, html);
    }
}

use mini_markdown::render;
#[test]
fn test_image_xss(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("![Alt text](foo.jpeg)", "<img src=\"foo.jpeg\" alt=\"Alt text\" referrerpolicy=\"no-referrer\">"),
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
        ("[text](javascript:alert(0))", "<a href=\"\" referrerpolicy=\"no-referrer\">text</a>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}