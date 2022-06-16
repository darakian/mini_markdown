use mini_markdown::render;


#[test]
fn commonmark_test_301_lists() {
    let test_html = render("- foo\n- bar\n+ baz\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<ul>\n<li>baz</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_302_lists() {
    let test_html = render("1. foo\n2. bar\n3) baz\n");
    let reference_html = "<ol>\n<li>foo</li>\n<li>bar</li>\n</ol>\n<ol start=\"3\">\n<li>baz</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_303_lists() {
    let test_html = render("Foo\n- bar\n- baz\n");
    let reference_html = "<p>Foo</p>\n<ul>\n<li>bar</li>\n<li>baz</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_304_lists() {
    let test_html = render("The number of windows in my house is\n14.  The number of doors is 6.\n");
    let reference_html = "<p>The number of windows in my house is\n14.  The number of doors is 6.</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_305_lists() {
    let test_html = render("The number of windows in my house is\n1.  The number of doors is 6.\n");
    let reference_html = "<p>The number of windows in my house is</p>\n<ol>\n<li>The number of doors is 6.</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_306_lists() {
    let test_html = render("- foo\n\n- bar\n\n\n- baz\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n</li>\n<li>\n<p>bar</p>\n</li>\n<li>\n<p>baz</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_307_lists() {
    let test_html = render("- foo\n  - bar\n    - baz\n\n\n      bim\n");
    let reference_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>\n<p>baz</p>\n<p>bim</p>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_308_lists() {
    let test_html = render("- foo\n- bar\n\n<!-- -->\n\n- baz\n- bim\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<!-- -->\n<ul>\n<li>baz</li>\n<li>bim</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_309_lists() {
    let test_html = render("-   foo\n\n    notcode\n\n-   foo\n\n<!-- -->\n\n    code\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>notcode</p>\n</li>\n<li>\n<p>foo</p>\n</li>\n</ul>\n<!-- -->\n<pre><code>code\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_310_lists() {
    let test_html = render("- a\n - b\n  - c\n   - d\n  - e\n - f\n- g\n");
    let reference_html = "<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d</li>\n<li>e</li>\n<li>f</li>\n<li>g</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_311_lists() {
    let test_html = render("1. a\n\n  2. b\n\n   3. c\n");
    let reference_html = "<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_312_lists() {
    let test_html = render("- a\n - b\n  - c\n   - d\n    - e\n");
    let reference_html = "<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d\n- e</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_313_lists() {
    let test_html = render("1. a\n\n  2. b\n\n    3. c\n");
    let reference_html = "<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n</ol>\n<pre><code>3. c\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_314_lists() {
    let test_html = render("- a\n- b\n\n- c\n");
    let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_315_lists() {
    let test_html = render("* a\n*\n\n* c\n");
    let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li></li>\n<li>\n<p>c</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_316_lists() {
    let test_html = render("- a\n- b\n\n  c\n- d\n");
    let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_317_lists() {
    let test_html = render("- a\n- b\n\n  [ref]: /url\n- d\n");
    let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_318_lists() {
    let test_html = render("- a\n- ```\n  b\n\n\n  ```\n- c\n");
    let reference_html = "<ul>\n<li>a</li>\n<li>\n<pre><code>b\n\n\n</code></pre>\n</li>\n<li>c</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_319_lists() {
    let test_html = render("- a\n  - b\n\n    c\n- d\n");
    let reference_html = "<ul>\n<li>a\n<ul>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n</ul>\n</li>\n<li>d</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_320_lists() {
    let test_html = render("* a\n  > b\n  >\n* c\n");
    let reference_html = "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n</li>\n<li>c</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_321_lists() {
    let test_html = render("- a\n  > b\n  ```\n  c\n  ```\n- d\n");
    let reference_html = "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n<pre><code>c\n</code></pre>\n</li>\n<li>d</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_322_lists() {
    let test_html = render("- a\n");
    let reference_html = "<ul>\n<li>a</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_323_lists() {
    let test_html = render("- a\n  - b\n");
    let reference_html = "<ul>\n<li>a\n<ul>\n<li>b</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_324_lists() {
    let test_html = render("1. ```\n   foo\n   ```\n\n   bar\n");
    let reference_html = "<ol>\n<li>\n<pre><code>foo\n</code></pre>\n<p>bar</p>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_325_lists() {
    let test_html = render("* foo\n  * bar\n\n  baz\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n<p>baz</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_326_lists() {
    let test_html = render("- a\n  - b\n  - c\n\n- d\n  - e\n  - f\n");
    let reference_html = "<ul>\n<li>\n<p>a</p>\n<ul>\n<li>b</li>\n<li>c</li>\n</ul>\n</li>\n<li>\n<p>d</p>\n<ul>\n<li>e</li>\n<li>f</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


