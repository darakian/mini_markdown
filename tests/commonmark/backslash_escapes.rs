use mini_markdown::render;


#[test]
fn commonmark_test_12_backslash_escapes() {
    let test_html = render("\\!\\\"\\#\\$\\%\\&\\\'\\(\\)\\*\\+\\,\\-\\.\\/\\:\\;\\<\\=\\>\\?\\@\\[\\\\\\]\\^\\_\\`\\{\\|\\}\\~\n");
    let reference_html = "<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\\]^_`{|}~</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_13_backslash_escapes() {
    let test_html = render("\\\t\\A\\a\\ \\3\\φ\\«\n");
    let reference_html = "<p>\\\t\\A\\a\\ \\3\\φ\\«</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_14_backslash_escapes() {
    let test_html = render("\\*not emphasized*\n\\<br/> not a tag\n\\[not a link](/foo)\n\\`not code`\n1\\. not a list\n\\* not a list\n\\# not a heading\n\\[foo]: /url \"not a reference\"\n\\&ouml; not a character entity\n");
    let reference_html = "<p>*not emphasized*\n&lt;br/&gt; not a tag\n[not a link](/foo)\n`not code`\n1. not a list\n* not a list\n# not a heading\n[foo]: /url &quot;not a reference&quot;\n&amp;ouml; not a character entity</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_15_backslash_escapes() {
    let test_html = render("\\\\*emphasis*\n");
    let reference_html = "<p>\\<em>emphasis</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_16_backslash_escapes() {
    let test_html = render("foo\\\nbar\n");
    let reference_html = "<p>foo<br />\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_17_backslash_escapes() {
    let test_html = render("`` \\[\\` ``\n");
    let reference_html = "<p><code>\\[\\`</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_18_backslash_escapes() {
    let test_html = render("    \\[\\]\n");
    let reference_html = "<pre><code>\\[\\]\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_19_backslash_escapes() {
    let test_html = render("~~~\n\\[\\]\n~~~\n");
    let reference_html = "<pre><code>\\[\\]\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_20_backslash_escapes() {
    let test_html = render("<http://example.com?find=\\*>\n");
    let reference_html = "<p><a href=\"http://example.com?find=%5C*\">http://example.com?find=\\*</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_21_backslash_escapes() {
    let test_html = render("<a href=\"/bar\\/)\">\n");
    let reference_html = "<a href=\"/bar\\/)\">\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_22_backslash_escapes() {
    let test_html = render("[foo](/bar\\* \"ti\\*tle\")\n");
    let reference_html = "<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_23_backslash_escapes() {
    let test_html = render("[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"\n");
    let reference_html = "<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_24_backslash_escapes() {
    let test_html = render("``` foo\\+bar\nfoo\n```\n");
    let reference_html = "<pre><code class=\"language-foo+bar\">foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


