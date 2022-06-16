use mini_markdown::render;


#[test]
fn commonmark_test_228_block_quotes() {
    let test_html = render("> # Foo\n> bar\n> baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_229_block_quotes() {
    let test_html = render("># Foo\n>bar\n> baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_230_block_quotes() {
    let test_html = render("   > # Foo\n   > bar\n > baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_231_block_quotes() {
    let test_html = render("    > # Foo\n    > bar\n    > baz\n");
    let reference_html = "<pre><code>&gt; # Foo\n&gt; bar\n&gt; baz\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_232_block_quotes() {
    let test_html = render("> # Foo\n> bar\nbaz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_233_block_quotes() {
    let test_html = render("> bar\nbaz\n> foo\n");
    let reference_html = "<blockquote>\n<p>bar\nbaz\nfoo</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_234_block_quotes() {
    let test_html = render("> foo\n---\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_235_block_quotes() {
    let test_html = render("> - foo\n- bar\n");
    let reference_html = "<blockquote>\n<ul>\n<li>foo</li>\n</ul>\n</blockquote>\n<ul>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_236_block_quotes() {
    let test_html = render(">     foo\n    bar\n");
    let reference_html = "<blockquote>\n<pre><code>foo\n</code></pre>\n</blockquote>\n<pre><code>bar\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_237_block_quotes() {
    let test_html = render("> ```\nfoo\n```\n");
    let reference_html = "<blockquote>\n<pre><code></code></pre>\n</blockquote>\n<p>foo</p>\n<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_238_block_quotes() {
    let test_html = render("> foo\n    - bar\n");
    let reference_html = "<blockquote>\n<p>foo\n- bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_239_block_quotes() {
    let test_html = render(">\n");
    let reference_html = "<blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_240_block_quotes() {
    let test_html = render(">\n>  \n> \n");
    let reference_html = "<blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_241_block_quotes() {
    let test_html = render(">\n> foo\n>  \n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_242_block_quotes() {
    let test_html = render("> foo\n\n> bar\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_243_block_quotes() {
    let test_html = render("> foo\n> bar\n");
    let reference_html = "<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_244_block_quotes() {
    let test_html = render("> foo\n>\n> bar\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_245_block_quotes() {
    let test_html = render("foo\n> bar\n");
    let reference_html = "<p>foo</p>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_246_block_quotes() {
    let test_html = render("> aaa\n***\n> bbb\n");
    let reference_html = "<blockquote>\n<p>aaa</p>\n</blockquote>\n<hr />\n<blockquote>\n<p>bbb</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_247_block_quotes() {
    let test_html = render("> bar\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_248_block_quotes() {
    let test_html = render("> bar\n\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_249_block_quotes() {
    let test_html = render("> bar\n>\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_250_block_quotes() {
    let test_html = render("> > > foo\nbar\n");
    let reference_html = "<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_251_block_quotes() {
    let test_html = render(">>> foo\n> bar\n>>baz\n");
    let reference_html = "<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar\nbaz</p>\n</blockquote>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_252_block_quotes() {
    let test_html = render(">     code\n\n>    not code\n");
    let reference_html = "<blockquote>\n<pre><code>code\n</code></pre>\n</blockquote>\n<blockquote>\n<p>not code</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


