use mini_markdown::render;


#[test]
fn commonmark_test_43_thematic_breaks() {
    let test_html = render("***\n---\n___\n");
    let reference_html = "<hr />\n<hr />\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_44_thematic_breaks() {
    let test_html = render("+++\n");
    let reference_html = "<p>+++</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_45_thematic_breaks() {
    let test_html = render("===\n");
    let reference_html = "<p>===</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_46_thematic_breaks() {
    let test_html = render("--\n**\n__\n");
    let reference_html = "<p>--\n**\n__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_47_thematic_breaks() {
    let test_html = render(" ***\n  ***\n   ***\n");
    let reference_html = "<hr />\n<hr />\n<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_48_thematic_breaks() {
    let test_html = render("    ***\n");
    let reference_html = "<pre><code>***\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_49_thematic_breaks() {
    let test_html = render("Foo\n    ***\n");
    let reference_html = "<p>Foo\n***</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_50_thematic_breaks() {
    let test_html = render("_____________________________________\n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_51_thematic_breaks() {
    let test_html = render(" - - -\n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_52_thematic_breaks() {
    let test_html = render(" **  * ** * ** * **\n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_53_thematic_breaks() {
    let test_html = render("-     -      -      -\n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_54_thematic_breaks() {
    let test_html = render("- - - -    \n");
    let reference_html = "<hr />\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_55_thematic_breaks() {
    let test_html = render("_ _ _ _ a\n\na------\n\n---a---\n");
    let reference_html = "<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_56_thematic_breaks() {
    let test_html = render(" *-*\n");
    let reference_html = "<p><em>-</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_57_thematic_breaks() {
    let test_html = render("- foo\n***\n- bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n<hr />\n<ul>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_58_thematic_breaks() {
    let test_html = render("Foo\n***\nbar\n");
    let reference_html = "<p>Foo</p>\n<hr />\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_59_thematic_breaks() {
    let test_html = render("Foo\n---\nbar\n");
    let reference_html = "<h2>Foo</h2>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_60_thematic_breaks() {
    let test_html = render("* Foo\n* * *\n* Bar\n");
    let reference_html = "<ul>\n<li>Foo</li>\n</ul>\n<hr />\n<ul>\n<li>Bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_61_thematic_breaks() {
    let test_html = render("- Foo\n- * * *\n");
    let reference_html = "<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


