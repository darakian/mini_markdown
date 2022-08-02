use mini_markdown::render;


#[test]
fn commonmark_test_648_soft_line_breaks() {
    let test_html = render("foo\nbaz\n");
    let reference_html = "<p>foo\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_649_soft_line_breaks() {
    let test_html = render("foo \n baz\n");
    let reference_html = "<p>foo\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


