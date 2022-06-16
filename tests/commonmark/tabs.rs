use mini_markdown::render;


#[test]
fn commonmark_test_1_tabs() {
    let test_html = render("\tfoo\tbaz\t\tbim\n");
    let reference_html = "<pre><code>foo\tbaz\t\tbim\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_2_tabs() {
    let test_html = render("  \tfoo\tbaz\t\tbim\n");
    let reference_html = "<pre><code>foo\tbaz\t\tbim\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_3_tabs() {
    let test_html = render("    a\ta\n    ὐ\ta\n");
    let reference_html = "<pre><code>a\ta\nὐ\ta\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_4_tabs() {
    let test_html = render("  - foo\n\n\tbar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_5_tabs() {
    let test_html = render("- foo\n\n\t\tbar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<pre><code>  bar\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_6_tabs() {
    let test_html = render(">\t\tfoo\n");
    let reference_html = "<blockquote>\n<pre><code>  foo\n</code></pre>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_7_tabs() {
    let test_html = render("-\t\tfoo\n");
    let reference_html = "<ul>\n<li>\n<pre><code>  foo\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_8_tabs() {
    let test_html = render("    foo\n\tbar\n");
    let reference_html = "<pre><code>foo\nbar\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_9_tabs() {
    let test_html = render(" - foo\n   - bar\n\t - baz\n");
    let reference_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_10_tabs() {
    let test_html = render("#\tFoo\n");
    let reference_html = "<h1>Foo</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_11_tabs() {
    let test_html = render("*\t*\t*\t\n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


