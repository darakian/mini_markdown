use mini_markdown::render;


#[test]
fn commonmark_test_253_list_items() {
    let test_html = render("A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.\n");
    let reference_html = "<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_254_list_items() {
    let test_html = render("1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_255_list_items() {
    let test_html = render("- one\n\n two\n");
    let reference_html = "<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_256_list_items() {
    let test_html = render("- one\n\n  two\n");
    let reference_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_257_list_items() {
    let test_html = render(" -    one\n\n     two\n");
    let reference_html = "<ul>\n<li>one</li>\n</ul>\n<pre><code> two\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_258_list_items() {
    let test_html = render(" -    one\n\n      two\n");
    let reference_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_259_list_items() {
    let test_html = render("   > > 1.  one\n>>\n>>     two\n");
    let reference_html = "<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_260_list_items() {
    let test_html = render(">>- one\n>>\n  >  > two\n");
    let reference_html = "<blockquote>\n<blockquote>\n<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_261_list_items() {
    let test_html = render("-one\n\n2.two\n");
    let reference_html = "<p>-one</p>\n<p>2.two</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_262_list_items() {
    let test_html = render("- foo\n\n\n  bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_263_list_items() {
    let test_html = render("1.  foo\n\n    ```\n    bar\n    ```\n\n    baz\n\n    > bam\n");
    let reference_html = "<ol>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n<blockquote>\n<p>bam</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_264_list_items() {
    let test_html = render("- Foo\n\n      bar\n\n\n      baz\n");
    let reference_html = "<ul>\n<li>\n<p>Foo</p>\n<pre><code>bar\n\n\nbaz\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_265_list_items() {
    let test_html = render("123456789. ok\n");
    let reference_html = "<ol start=\"123456789\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_266_list_items() {
    let test_html = render("1234567890. not ok\n");
    let reference_html = "<p>1234567890. not ok</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_267_list_items() {
    let test_html = render("0. ok\n");
    let reference_html = "<ol start=\"0\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_268_list_items() {
    let test_html = render("003. ok\n");
    let reference_html = "<ol start=\"3\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_269_list_items() {
    let test_html = render("-1. not ok\n");
    let reference_html = "<p>-1. not ok</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_270_list_items() {
    let test_html = render("- foo\n\n      bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_271_list_items() {
    let test_html = render("  10.  foo\n\n           bar\n");
    let reference_html = "<ol start=\"10\">\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_272_list_items() {
    let test_html = render("    indented code\n\nparagraph\n\n    more code\n");
    let reference_html = "<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_273_list_items() {
    let test_html = render("1.     indented code\n\n   paragraph\n\n       more code\n");
    let reference_html = "<ol>\n<li>\n<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_274_list_items() {
    let test_html = render("1.      indented code\n\n   paragraph\n\n       more code\n");
    let reference_html = "<ol>\n<li>\n<pre><code> indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_275_list_items() {
    let test_html = render("   foo\n\nbar\n");
    let reference_html = "<p>foo</p>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_276_list_items() {
    let test_html = render("-    foo\n\n  bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_277_list_items() {
    let test_html = render("-  foo\n\n   bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_278_list_items() {
    let test_html = render("-\n  foo\n-\n  ```\n  bar\n  ```\n-\n      baz\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li>\n<pre><code>bar\n</code></pre>\n</li>\n<li>\n<pre><code>baz\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_279_list_items() {
    let test_html = render("-   \n  foo\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_280_list_items() {
    let test_html = render("-\n\n  foo\n");
    let reference_html = "<ul>\n<li></li>\n</ul>\n<p>foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_281_list_items() {
    let test_html = render("- foo\n-\n- bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_282_list_items() {
    let test_html = render("- foo\n-   \n- bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_283_list_items() {
    let test_html = render("1. foo\n2.\n3. bar\n");
    let reference_html = "<ol>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_284_list_items() {
    let test_html = render("*\n");
    let reference_html = "<ul>\n<li></li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_285_list_items() {
    let test_html = render("foo\n*\n\nfoo\n1.\n");
    let reference_html = "<p>foo\n*</p>\n<p>foo\n1.</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_286_list_items() {
    let test_html = render(" 1.  A paragraph\n     with two lines.\n\n         indented code\n\n     > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_287_list_items() {
    let test_html = render("  1.  A paragraph\n      with two lines.\n\n          indented code\n\n      > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_288_list_items() {
    let test_html = render("   1.  A paragraph\n       with two lines.\n\n           indented code\n\n       > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_289_list_items() {
    let test_html = render("    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.\n");
    let reference_html = "<pre><code>1.  A paragraph\n    with two lines.\n\n        indented code\n\n    &gt; A block quote.\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_290_list_items() {
    let test_html = render("  1.  A paragraph\nwith two lines.\n\n          indented code\n\n      > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_291_list_items() {
    let test_html = render("  1.  A paragraph\n    with two lines.\n");
    let reference_html = "<ol>\n<li>A paragraph\nwith two lines.</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_292_list_items() {
    let test_html = render("> 1. > Blockquote\ncontinued here.\n");
    let reference_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_293_list_items() {
    let test_html = render("> 1. > Blockquote\n> continued here.\n");
    let reference_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_294_list_items() {
    let test_html = render("- foo\n  - bar\n    - baz\n      - boo\n");
    let reference_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz\n<ul>\n<li>boo</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_295_list_items() {
    let test_html = render("- foo\n - bar\n  - baz\n   - boo\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li>bar</li>\n<li>baz</li>\n<li>boo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_296_list_items() {
    let test_html = render("10) foo\n    - bar\n");
    let reference_html = "<ol start=\"10\">\n<li>foo\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_297_list_items() {
    let test_html = render("10) foo\n   - bar\n");
    let reference_html = "<ol start=\"10\">\n<li>foo</li>\n</ol>\n<ul>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_298_list_items() {
    let test_html = render("- - foo\n");
    let reference_html = "<ul>\n<li>\n<ul>\n<li>foo</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_299_list_items() {
    let test_html = render("1. - 2. foo\n");
    let reference_html = "<ol>\n<li>\n<ul>\n<li>\n<ol start=\"2\">\n<li>foo</li>\n</ol>\n</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_300_list_items() {
    let test_html = render("- # Foo\n- Bar\n  ---\n  baz\n");
    let reference_html = "<ul>\n<li>\n<h1>Foo</h1>\n</li>\n<li>\n<h2>Bar</h2>\nbaz</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


