use mini_markdown::render;


#[test]
fn commonmark_test_633_hard_line_breaks() {
    let test_html = render("foo  \nbaz\n");
    let reference_html = "<p>foo<br />\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_634_hard_line_breaks() {
    let test_html = render("foo\\\nbaz\n");
    let reference_html = "<p>foo<br />\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_635_hard_line_breaks() {
    let test_html = render("foo       \nbaz\n");
    let reference_html = "<p>foo<br />\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_636_hard_line_breaks() {
    let test_html = render("foo  \n     bar\n");
    let reference_html = "<p>foo<br />\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_637_hard_line_breaks() {
    let test_html = render("foo\\\n     bar\n");
    let reference_html = "<p>foo<br />\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_638_hard_line_breaks() {
    let test_html = render("*foo  \nbar*\n");
    let reference_html = "<p><em>foo<br />\nbar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_639_hard_line_breaks() {
    let test_html = render("*foo\\\nbar*\n");
    let reference_html = "<p><em>foo<br />\nbar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_640_hard_line_breaks() {
    let test_html = render("`code  \nspan`\n");
    let reference_html = "<p><code>code   span</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_641_hard_line_breaks() {
    let test_html = render("`code\\\nspan`\n");
    let reference_html = "<p><code>code\\ span</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_642_hard_line_breaks() {
    let test_html = render("<a href=\"foo  \nbar\">\n");
    let reference_html = "<p><a href=\"foo  \nbar\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_643_hard_line_breaks() {
    let test_html = render("<a href=\"foo\\\nbar\">\n");
    let reference_html = "<p><a href=\"foo\\\nbar\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_644_hard_line_breaks() {
    let test_html = render("foo\\\n");
    let reference_html = "<p>foo\\</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_645_hard_line_breaks() {
    let test_html = render("foo  \n");
    let reference_html = "<p>foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_646_hard_line_breaks() {
    let test_html = render("### foo\\\n");
    let reference_html = "<h3>foo\\</h3>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_647_hard_line_breaks() {
    let test_html = render("### foo  \n");
    let reference_html = "<h3>foo</h3>\n";
    assert_eq!(test_html, reference_html);
}


