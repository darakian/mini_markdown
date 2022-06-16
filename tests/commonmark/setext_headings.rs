use mini_markdown::render;


#[test]
fn commonmark_test_80_setext_headings() {
    let test_html = render("Foo *bar*\n=========\n\nFoo *bar*\n---------\n");
    let reference_html = "<h1>Foo <em>bar</em></h1>\n<h2>Foo <em>bar</em></h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_81_setext_headings() {
    let test_html = render("Foo *bar\nbaz*\n====\n");
    let reference_html = "<h1>Foo <em>bar\nbaz</em></h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_82_setext_headings() {
    let test_html = render("  Foo *bar\nbaz*\t\n====\n");
    let reference_html = "<h1>Foo <em>bar\nbaz</em></h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_83_setext_headings() {
    let test_html = render("Foo\n-------------------------\n\nFoo\n=\n");
    let reference_html = "<h2>Foo</h2>\n<h1>Foo</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_84_setext_headings() {
    let test_html = render("   Foo\n---\n\n  Foo\n-----\n\n  Foo\n  ===\n");
    let reference_html = "<h2>Foo</h2>\n<h2>Foo</h2>\n<h1>Foo</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_85_setext_headings() {
    let test_html = render("    Foo\n    ---\n\n    Foo\n---\n");
    let reference_html = "<pre><code>Foo\n---\n\nFoo\n</code></pre>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_86_setext_headings() {
    let test_html = render("Foo\n   ----      \n");
    let reference_html = "<h2>Foo</h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_87_setext_headings() {
    let test_html = render("Foo\n    ---\n");
    let reference_html = "<p>Foo\n---</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_88_setext_headings() {
    let test_html = render("Foo\n= =\n\nFoo\n--- -\n");
    let reference_html = "<p>Foo\n= =</p>\n<p>Foo</p>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_89_setext_headings() {
    let test_html = render("Foo  \n-----\n");
    let reference_html = "<h2>Foo</h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_90_setext_headings() {
    let test_html = render("Foo\\\n----\n");
    let reference_html = "<h2>Foo\\</h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_91_setext_headings() {
    let test_html = render("`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>\n");
    let reference_html = "<h2>`Foo</h2>\n<p>`</p>\n<h2>&lt;a title=&quot;a lot</h2>\n<p>of dashes&quot;/&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_92_setext_headings() {
    let test_html = render("> Foo\n---\n");
    let reference_html = "<blockquote>\n<p>Foo</p>\n</blockquote>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_93_setext_headings() {
    let test_html = render("> foo\nbar\n===\n");
    let reference_html = "<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_94_setext_headings() {
    let test_html = render("- Foo\n---\n");
    let reference_html = "<ul>\n<li>Foo</li>\n</ul>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_95_setext_headings() {
    let test_html = render("Foo\nBar\n---\n");
    let reference_html = "<h2>Foo\nBar</h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_96_setext_headings() {
    let test_html = render("---\nFoo\n---\nBar\n---\nBaz\n");
    let reference_html = "<hr />\n<h2>Foo</h2>\n<h2>Bar</h2>\n<p>Baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_97_setext_headings() {
    let test_html = render("\n====\n");
    let reference_html = "<p>====</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_98_setext_headings() {
    let test_html = render("---\n---\n");
    let reference_html = "<hr />\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_99_setext_headings() {
    let test_html = render("- foo\n-----\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_100_setext_headings() {
    let test_html = render("    foo\n---\n");
    let reference_html = "<pre><code>foo\n</code></pre>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_101_setext_headings() {
    let test_html = render("> foo\n-----\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_102_setext_headings() {
    let test_html = render("\\> foo\n------\n");
    let reference_html = "<h2>&gt; foo</h2>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_103_setext_headings() {
    let test_html = render("Foo\n\nbar\n---\nbaz\n");
    let reference_html = "<p>Foo</p>\n<h2>bar</h2>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_104_setext_headings() {
    let test_html = render("Foo\nbar\n\n---\n\nbaz\n");
    let reference_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_105_setext_headings() {
    let test_html = render("Foo\nbar\n* * *\nbaz\n");
    let reference_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_106_setext_headings() {
    let test_html = render("Foo\nbar\n\\---\nbaz\n");
    let reference_html = "<p>Foo\nbar\n---\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


