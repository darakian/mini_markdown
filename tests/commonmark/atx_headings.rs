use mini_markdown::render;


#[test]
fn commonmark_test_62_atx_headings() {
    let test_html = render("# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo\n");
    let reference_html = "<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</h4>\n<h5>foo</h5>\n<h6>foo</h6>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_63_atx_headings() {
    let test_html = render("####### foo\n");
    let reference_html = "<p>####### foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_64_atx_headings() {
    let test_html = render("#5 bolt\n\n#hashtag\n");
    let reference_html = "<p>#5 bolt</p>\n<p>#hashtag</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_65_atx_headings() {
    let test_html = render("\\## foo\n");
    let reference_html = "<p>## foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_66_atx_headings() {
    let test_html = render("# foo *bar* \\*baz\\*\n");
    let reference_html = "<h1>foo <em>bar</em> *baz*</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_67_atx_headings() {
    let test_html = render("#                  foo                     \n");
    let reference_html = "<h1>foo</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_68_atx_headings() {
    let test_html = render(" ### foo\n  ## foo\n   # foo\n");
    let reference_html = "<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_69_atx_headings() {
    let test_html = render("    # foo\n");
    let reference_html = "<pre><code># foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_70_atx_headings() {
    let test_html = render("foo\n    # bar\n");
    let reference_html = "<p>foo\n# bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_71_atx_headings() {
    let test_html = render("## foo ##\n  ###   bar    ###\n");
    let reference_html = "<h2>foo</h2>\n<h3>bar</h3>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_72_atx_headings() {
    let test_html = render("# foo ##################################\n##### foo ##\n");
    let reference_html = "<h1>foo</h1>\n<h5>foo</h5>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_73_atx_headings() {
    let test_html = render("### foo ###     \n");
    let reference_html = "<h3>foo</h3>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_74_atx_headings() {
    let test_html = render("### foo ### b\n");
    let reference_html = "<h3>foo ### b</h3>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_75_atx_headings() {
    let test_html = render("# foo#\n");
    let reference_html = "<h1>foo#</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_76_atx_headings() {
    let test_html = render("### foo \\###\n## foo #\\##\n# foo \\#\n");
    let reference_html = "<h3>foo ###</h3>\n<h2>foo ###</h2>\n<h1>foo #</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_77_atx_headings() {
    let test_html = render("****\n## foo\n****\n");
    let reference_html = "<hr />\n<h2>foo</h2>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_78_atx_headings() {
    let test_html = render("Foo bar\n# baz\nBar foo\n");
    let reference_html = "<p>Foo bar</p>\n<h1>baz</h1>\n<p>Bar foo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_79_atx_headings() {
    let test_html = render("## \n#\n### ###\n");
    let reference_html = "<h2></h2>\n<h1></h1>\n<h3></h3>\n";
    assert_eq!(test_html, reference_html);
}


