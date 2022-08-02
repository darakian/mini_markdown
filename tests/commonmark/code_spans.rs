use mini_markdown::render;


#[test]
fn commonmark_test_328_code_spans() {
    let test_html = render("`foo`\n");
    let reference_html = "<p><code>foo</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_329_code_spans() {
    let test_html = render("`` foo ` bar ``\n");
    let reference_html = "<p><code>foo ` bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_330_code_spans() {
    let test_html = render("` `` `\n");
    let reference_html = "<p><code>``</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_331_code_spans() {
    let test_html = render("`  ``  `\n");
    let reference_html = "<p><code> `` </code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_332_code_spans() {
    let test_html = render("` a`\n");
    let reference_html = "<p><code> a</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_333_code_spans() {
    let test_html = render("` b `\n");
    let reference_html = "<p><code> b </code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_334_code_spans() {
    let test_html = render("` `\n`  `\n");
    let reference_html = "<p><code> </code>\n<code>  </code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_335_code_spans() {
    let test_html = render("``\nfoo\nbar  \nbaz\n``\n");
    let reference_html = "<p><code>foo bar   baz</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_336_code_spans() {
    let test_html = render("``\nfoo \n``\n");
    let reference_html = "<p><code>foo </code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_337_code_spans() {
    let test_html = render("`foo   bar \nbaz`\n");
    let reference_html = "<p><code>foo   bar  baz</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_338_code_spans() {
    let test_html = render("`foo\\`bar`\n");
    let reference_html = "<p><code>foo\\</code>bar`</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_339_code_spans() {
    let test_html = render("``foo`bar``\n");
    let reference_html = "<p><code>foo`bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_340_code_spans() {
    let test_html = render("` foo `` bar `\n");
    let reference_html = "<p><code>foo `` bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_341_code_spans() {
    let test_html = render("*foo`*`\n");
    let reference_html = "<p>*foo<code>*</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_342_code_spans() {
    let test_html = render("[not a `link](/foo`)\n");
    let reference_html = "<p>[not a <code>link](/foo</code>)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_343_code_spans() {
    let test_html = render("`<a href=\"`\">`\n");
    let reference_html = "<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_344_code_spans() {
    let test_html = render("<a href=\"`\">`\n");
    let reference_html = "<p><a href=\"`\">`</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_345_code_spans() {
    let test_html = render("`<http://foo.bar.`baz>`\n");
    let reference_html = "<p><code>&lt;http://foo.bar.</code>baz&gt;`</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_346_code_spans() {
    let test_html = render("<http://foo.bar.`baz>`\n");
    let reference_html = "<p><a href=\"http://foo.bar.%60baz\">http://foo.bar.`baz</a>`</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_347_code_spans() {
    let test_html = render("```foo``\n");
    let reference_html = "<p>```foo``</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_348_code_spans() {
    let test_html = render("`foo\n");
    let reference_html = "<p>`foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_349_code_spans() {
    let test_html = render("`foo``bar``\n");
    let reference_html = "<p>`foo<code>bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}


