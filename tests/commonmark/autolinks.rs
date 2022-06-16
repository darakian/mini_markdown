use mini_markdown::render;


#[test]
fn commonmark_test_593_autolinks() {
    let test_html = render("<http://foo.bar.baz>\n");
    let reference_html = "<p><a href=\"http://foo.bar.baz\">http://foo.bar.baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_594_autolinks() {
    let test_html = render("<http://foo.bar.baz/test?q=hello&id=22&boolean>\n");
    let reference_html = "<p><a href=\"http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean\">http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_595_autolinks() {
    let test_html = render("<irc://foo.bar:2233/baz>\n");
    let reference_html = "<p><a href=\"irc://foo.bar:2233/baz\">irc://foo.bar:2233/baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_596_autolinks() {
    let test_html = render("<MAILTO:FOO@BAR.BAZ>\n");
    let reference_html = "<p><a href=\"MAILTO:FOO@BAR.BAZ\">MAILTO:FOO@BAR.BAZ</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_597_autolinks() {
    let test_html = render("<a+b+c:d>\n");
    let reference_html = "<p><a href=\"a+b+c:d\">a+b+c:d</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_598_autolinks() {
    let test_html = render("<made-up-scheme://foo,bar>\n");
    let reference_html = "<p><a href=\"made-up-scheme://foo,bar\">made-up-scheme://foo,bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_599_autolinks() {
    let test_html = render("<http://../>\n");
    let reference_html = "<p><a href=\"http://../\">http://../</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_600_autolinks() {
    let test_html = render("<localhost:5001/foo>\n");
    let reference_html = "<p><a href=\"localhost:5001/foo\">localhost:5001/foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_601_autolinks() {
    let test_html = render("<http://foo.bar/baz bim>\n");
    let reference_html = "<p>&lt;http://foo.bar/baz bim&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_602_autolinks() {
    let test_html = render("<http://example.com/\\[\\>\n");
    let reference_html = "<p><a href=\"http://example.com/%5C%5B%5C\">http://example.com/\\[\\</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_603_autolinks() {
    let test_html = render("<foo@bar.example.com>\n");
    let reference_html = "<p><a href=\"mailto:foo@bar.example.com\">foo@bar.example.com</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_604_autolinks() {
    let test_html = render("<foo+special@Bar.baz-bar0.com>\n");
    let reference_html = "<p><a href=\"mailto:foo+special@Bar.baz-bar0.com\">foo+special@Bar.baz-bar0.com</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_605_autolinks() {
    let test_html = render("<foo\\+@bar.example.com>\n");
    let reference_html = "<p>&lt;foo+@bar.example.com&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_606_autolinks() {
    let test_html = render("<>\n");
    let reference_html = "<p>&lt;&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_607_autolinks() {
    let test_html = render("< http://foo.bar >\n");
    let reference_html = "<p>&lt; http://foo.bar &gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_608_autolinks() {
    let test_html = render("<m:abc>\n");
    let reference_html = "<p>&lt;m:abc&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_609_autolinks() {
    let test_html = render("<foo.bar.baz>\n");
    let reference_html = "<p>&lt;foo.bar.baz&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_610_autolinks() {
    let test_html = render("http://example.com\n");
    let reference_html = "<p>http://example.com</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_611_autolinks() {
    let test_html = render("foo@bar.example.com\n");
    let reference_html = "<p>foo@bar.example.com</p>\n";
    assert_eq!(test_html, reference_html);
}


