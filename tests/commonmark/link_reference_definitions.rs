use mini_markdown::render;


#[test]
fn commonmark_test_192_link_reference_definitions() {
    let test_html = render("[foo]: /url \"title\"\n\n[foo]\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_193_link_reference_definitions() {
    let test_html = render("   [foo]: \n      /url  \n           'the title'  \n\n[foo]\n");
    let reference_html = "<p><a href=\"/url\" title=\"the title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_194_link_reference_definitions() {
    let test_html = render("[Foo*bar\\]]:my_(url) 'title (with parens)'\n\n[Foo*bar\\]]\n");
    let reference_html = "<p><a href=\"my_(url)\" title=\"title (with parens)\">Foo*bar]</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_195_link_reference_definitions() {
    let test_html = render("[Foo bar]:\n<my url>\n'title'\n\n[Foo bar]\n");
    let reference_html = "<p><a href=\"my%20url\" title=\"title\">Foo bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_196_link_reference_definitions() {
    let test_html = render("[foo]: /url '\ntitle\nline1\nline2\n'\n\n[foo]\n");
    let reference_html = "<p><a href=\"/url\" title=\"\ntitle\nline1\nline2\n\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_197_link_reference_definitions() {
    let test_html = render("[foo]: /url 'title\n\nwith blank line'\n\n[foo]\n");
    let reference_html = "<p>[foo]: /url 'title</p>\n<p>with blank line'</p>\n<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_198_link_reference_definitions() {
    let test_html = render("[foo]:\n/url\n\n[foo]\n");
    let reference_html = "<p><a href=\"/url\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_199_link_reference_definitions() {
    let test_html = render("[foo]:\n\n[foo]\n");
    let reference_html = "<p>[foo]:</p>\n<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_200_link_reference_definitions() {
    let test_html = render("[foo]: <>\n\n[foo]\n");
    let reference_html = "<p><a href=\"\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_201_link_reference_definitions() {
    let test_html = render("[foo]: <bar>(baz)\n\n[foo]\n");
    let reference_html = "<p>[foo]: <bar>(baz)</p>\n<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_202_link_reference_definitions() {
    let test_html = render("[foo]: /url\\bar\\*baz \"foo\\\"bar\\baz\"\n\n[foo]\n");
    let reference_html = "<p><a href=\"/url%5Cbar*baz\" title=\"foo&quot;bar\\baz\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_203_link_reference_definitions() {
    let test_html = render("[foo]\n\n[foo]: url\n");
    let reference_html = "<p><a href=\"url\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_204_link_reference_definitions() {
    let test_html = render("[foo]\n\n[foo]: first\n[foo]: second\n");
    let reference_html = "<p><a href=\"first\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_205_link_reference_definitions() {
    let test_html = render("[FOO]: /url\n\n[Foo]\n");
    let reference_html = "<p><a href=\"/url\">Foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_206_link_reference_definitions() {
    let test_html = render("[ΑΓΩ]: /φου\n\n[αγω]\n");
    let reference_html = "<p><a href=\"/%CF%86%CE%BF%CF%85\">αγω</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_207_link_reference_definitions() {
    let test_html = render("[foo]: /url\n");
    let reference_html = "";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_208_link_reference_definitions() {
    let test_html = render("[\nfoo\n]: /url\nbar\n");
    let reference_html = "<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_209_link_reference_definitions() {
    let test_html = render("[foo]: /url \"title\" ok\n");
    let reference_html = "<p>[foo]: /url &quot;title&quot; ok</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_210_link_reference_definitions() {
    let test_html = render("[foo]: /url\n\"title\" ok\n");
    let reference_html = "<p>&quot;title&quot; ok</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_211_link_reference_definitions() {
    let test_html = render("    [foo]: /url \"title\"\n\n[foo]\n");
    let reference_html = "<pre><code>[foo]: /url &quot;title&quot;\n</code></pre>\n<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_212_link_reference_definitions() {
    let test_html = render("```\n[foo]: /url\n```\n\n[foo]\n");
    let reference_html = "<pre><code>[foo]: /url\n</code></pre>\n<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_213_link_reference_definitions() {
    let test_html = render("Foo\n[bar]: /baz\n\n[bar]\n");
    let reference_html = "<p>Foo\n[bar]: /baz</p>\n<p>[bar]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_214_link_reference_definitions() {
    let test_html = render("# [Foo]\n[foo]: /url\n> bar\n");
    let reference_html = "<h1><a href=\"/url\">Foo</a></h1>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_215_link_reference_definitions() {
    let test_html = render("[foo]: /url\nbar\n===\n[foo]\n");
    let reference_html = "<h1>bar</h1>\n<p><a href=\"/url\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_216_link_reference_definitions() {
    let test_html = render("[foo]: /url\n===\n[foo]\n");
    let reference_html = "<p>===\n<a href=\"/url\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_217_link_reference_definitions() {
    let test_html = render("[foo]: /foo-url \"foo\"\n[bar]: /bar-url\n  \"bar\"\n[baz]: /baz-url\n\n[foo],\n[bar],\n[baz]\n");
    let reference_html = "<p><a href=\"/foo-url\" title=\"foo\">foo</a>,\n<a href=\"/bar-url\" title=\"bar\">bar</a>,\n<a href=\"/baz-url\">baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_218_link_reference_definitions() {
    let test_html = render("[foo]\n\n> [foo]: /url\n");
    let reference_html = "<p><a href=\"/url\">foo</a></p>\n<blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}


