use mini_markdown::render;


#[test]
fn commonmark_test_107_indented_code_blocks() {
    let test_html = render("    a simple\n      indented code block\n");
    let reference_html = "<pre><code>a simple\n  indented code block\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_108_indented_code_blocks() {
    let test_html = render("  - foo\n\n    bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_109_indented_code_blocks() {
    let test_html = render("1.  foo\n\n    - bar\n");
    let reference_html = "<ol>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_110_indented_code_blocks() {
    let test_html = render("    <a/>\n    *hi*\n\n    - one\n");
    let reference_html = "<pre><code>&lt;a/&gt;\n*hi*\n\n- one\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_111_indented_code_blocks() {
    let test_html = render("    chunk1\n\n    chunk2\n  \n \n \n    chunk3\n");
    let reference_html = "<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_112_indented_code_blocks() {
    let test_html = render("    chunk1\n      \n      chunk2\n");
    let reference_html = "<pre><code>chunk1\n  \n  chunk2\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_113_indented_code_blocks() {
    let test_html = render("Foo\n    bar\n\n");
    let reference_html = "<p>Foo\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_114_indented_code_blocks() {
    let test_html = render("    foo\nbar\n");
    let reference_html = "<pre><code>foo\n</code></pre>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_115_indented_code_blocks() {
    let test_html = render("# Heading\n    foo\nHeading\n------\n    foo\n----\n");
    let reference_html = "<h1>Heading</h1>\n<pre><code>foo\n</code></pre>\n<h2>Heading</h2>\n<pre><code>foo\n</code></pre>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_116_indented_code_blocks() {
    let test_html = render("        foo\n    bar\n");
    let reference_html = "<pre><code>    foo\nbar\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_117_indented_code_blocks() {
    let test_html = render("\n    \n    foo\n    \n\n");
    let reference_html = "<pre><code>foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_118_indented_code_blocks() {
    let test_html = render("    foo  \n");
    let reference_html = "<pre><code>foo  \n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


