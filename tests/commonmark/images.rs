use mini_markdown::render;


#[test]
fn commonmark_test_571_images() {
    let test_html = render("![foo](/url \"title\")\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_572_images() {
    let test_html = render("![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n");
    let reference_html = "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_573_images() {
    let test_html = render("![foo ![bar](/url)](/url2)\n");
    let reference_html = "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_574_images() {
    let test_html = render("![foo [bar](/url)](/url2)\n");
    let reference_html = "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_575_images() {
    let test_html = render("![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n");
    let reference_html = "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_576_images() {
    let test_html = render("![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\"\n");
    let reference_html = "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_577_images() {
    let test_html = render("![foo](train.jpg)\n");
    let reference_html = "<p><img src=\"train.jpg\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_578_images() {
    let test_html = render("My ![foo bar](/path/to/train.jpg  \"title\"   )\n");
    let reference_html = "<p>My <img src=\"/path/to/train.jpg\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_579_images() {
    let test_html = render("![foo](<url>)\n");
    let reference_html = "<p><img src=\"url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_580_images() {
    let test_html = render("![](/url)\n");
    let reference_html = "<p><img src=\"/url\" alt=\"\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_581_images() {
    let test_html = render("![foo][bar]\n\n[bar]: /url\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_582_images() {
    let test_html = render("![foo][bar]\n\n[BAR]: /url\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_583_images() {
    let test_html = render("![foo][]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_584_images() {
    let test_html = render("![*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_585_images() {
    let test_html = render("![Foo][]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_586_images() {
    let test_html = render("![foo] \n[]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" title=\"title\" />\n[]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_587_images() {
    let test_html = render("![foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_588_images() {
    let test_html = render("![*foo* bar]\n\n[*foo* bar]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_589_images() {
    let test_html = render("![[foo]]\n\n[[foo]]: /url \"title\"\n");
    let reference_html = "<p>![[foo]]</p>\n<p>[[foo]]: /url &quot;title&quot;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_590_images() {
    let test_html = render("![Foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_591_images() {
    let test_html = render("!\\[foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p>![foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_592_images() {
    let test_html = render("\\![foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p>!<a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


