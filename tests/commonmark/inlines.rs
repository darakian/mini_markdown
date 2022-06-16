use mini_markdown::render;


#[test]
fn commonmark_test_327_inlines() {
    let test_html = render("`hi`lo`\n");
    let reference_html = "<p><code>hi</code>lo`</p>\n";
    assert_eq!(test_html, reference_html);
}


