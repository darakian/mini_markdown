use mini_markdown::render;


#[test]
fn commonmark_test_481_links() {
    let test_html = render("[link](/uri \"title\")\n");
    let reference_html = "<p><a href=\"/uri\" title=\"title\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_482_links() {
    let test_html = render("[link](/uri)\n");
    let reference_html = "<p><a href=\"/uri\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_483_links() {
    let test_html = render("[](./target.md)\n");
    let reference_html = "<p><a href=\"./target.md\"></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_484_links() {
    let test_html = render("[link]()\n");
    let reference_html = "<p><a href=\"\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_485_links() {
    let test_html = render("[link](<>)\n");
    let reference_html = "<p><a href=\"\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_486_links() {
    let test_html = render("[]()\n");
    let reference_html = "<p><a href=\"\"></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_487_links() {
    let test_html = render("[link](/my uri)\n");
    let reference_html = "<p>[link](/my uri)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_488_links() {
    let test_html = render("[link](</my uri>)\n");
    let reference_html = "<p><a href=\"/my%20uri\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_489_links() {
    let test_html = render("[link](foo\nbar)\n");
    let reference_html = "<p>[link](foo\nbar)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_490_links() {
    let test_html = render("[link](<foo\nbar>)\n");
    let reference_html = "<p>[link](<foo\nbar>)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_491_links() {
    let test_html = render("[a](<b)c>)\n");
    let reference_html = "<p><a href=\"b)c\">a</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_492_links() {
    let test_html = render("[link](<foo\\>)\n");
    let reference_html = "<p>[link](&lt;foo&gt;)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_493_links() {
    let test_html = render("[a](<b)c\n[a](<b)c>\n[a](<b>c)\n");
    let reference_html = "<p>[a](&lt;b)c\n[a](&lt;b)c&gt;\n[a](<b>c)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_494_links() {
    let test_html = render("[link](\\(foo\\))\n");
    let reference_html = "<p><a href=\"(foo)\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_495_links() {
    let test_html = render("[link](foo(and(bar)))\n");
    let reference_html = "<p><a href=\"foo(and(bar))\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_496_links() {
    let test_html = render("[link](foo(and(bar))\n");
    let reference_html = "<p>[link](foo(and(bar))</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_497_links() {
    let test_html = render("[link](foo\\(and\\(bar\\))\n");
    let reference_html = "<p><a href=\"foo(and(bar)\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_498_links() {
    let test_html = render("[link](<foo(and(bar)>)\n");
    let reference_html = "<p><a href=\"foo(and(bar)\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_499_links() {
    let test_html = render("[link](foo\\)\\:)\n");
    let reference_html = "<p><a href=\"foo):\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_500_links() {
    let test_html = render("[link](#fragment)\n\n[link](http://example.com#fragment)\n\n[link](http://example.com?foo=3#frag)\n");
    let reference_html = "<p><a href=\"#fragment\">link</a></p>\n<p><a href=\"http://example.com#fragment\">link</a></p>\n<p><a href=\"http://example.com?foo=3#frag\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_501_links() {
    let test_html = render("[link](foo\\bar)\n");
    let reference_html = "<p><a href=\"foo%5Cbar\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_502_links() {
    let test_html = render("[link](foo%20b&auml;)\n");
    let reference_html = "<p><a href=\"foo%20b%C3%A4\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_503_links() {
    let test_html = render("[link](\"title\")\n");
    let reference_html = "<p><a href=\"%22title%22\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_504_links() {
    let test_html = render("[link](/url \"title\")\n[link](/url \'title\')\n[link](/url (title))\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_505_links() {
    let test_html = render("[link](/url \"title \\\"&quot;\")\n");
    let reference_html = "<p><a href=\"/url\" title=\"title &quot;&quot;\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_506_links() {
    let test_html = render("[link](/url \"title\")\n");
    let reference_html = "<p><a href=\"/url%C2%A0%22title%22\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_507_links() {
    let test_html = render("[link](/url \"title \"and\" title\")\n");
    let reference_html = "<p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_508_links() {
    let test_html = render("[link](/url \'title \"and\" title\')\n");
    let reference_html = "<p><a href=\"/url\" title=\"title &quot;and&quot; title\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_509_links() {
    let test_html = render("[link](   /uri\n  \"title\"  )\n");
    let reference_html = "<p><a href=\"/uri\" title=\"title\">link</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_510_links() {
    let test_html = render("[link] (/uri)\n");
    let reference_html = "<p>[link] (/uri)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_511_links() {
    let test_html = render("[link [foo [bar]]](/uri)\n");
    let reference_html = "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_512_links() {
    let test_html = render("[link] bar](/uri)\n");
    let reference_html = "<p>[link] bar](/uri)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_513_links() {
    let test_html = render("[link [bar](/uri)\n");
    let reference_html = "<p>[link <a href=\"/uri\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_514_links() {
    let test_html = render("[link \\[bar](/uri)\n");
    let reference_html = "<p><a href=\"/uri\">link [bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_515_links() {
    let test_html = render("[link *foo **bar** `#`*](/uri)\n");
    let reference_html = "<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_516_links() {
    let test_html = render("[![moon](moon.jpg)](/uri)\n");
    let reference_html = "<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_517_links() {
    let test_html = render("[foo [bar](/uri)](/uri)\n");
    let reference_html = "<p>[foo <a href=\"/uri\">bar</a>](/uri)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_518_links() {
    let test_html = render("[foo *[bar [baz](/uri)](/uri)*](/uri)\n");
    let reference_html = "<p>[foo <em>[bar <a href=\"/uri\">baz</a>](/uri)</em>](/uri)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_519_links() {
    let test_html = render("![[[foo](uri1)](uri2)](uri3)\n");
    let reference_html = "<p><img src=\"uri3\" alt=\"[foo](uri2)\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_520_links() {
    let test_html = render("*[foo*](/uri)\n");
    let reference_html = "<p>*<a href=\"/uri\">foo*</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_521_links() {
    let test_html = render("[foo *bar](baz*)\n");
    let reference_html = "<p><a href=\"baz*\">foo *bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_522_links() {
    let test_html = render("*foo [bar* baz]\n");
    let reference_html = "<p><em>foo [bar</em> baz]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_523_links() {
    let test_html = render("[foo <bar attr=\"](baz)\">\n");
    let reference_html = "<p>[foo <bar attr=\"](baz)\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_524_links() {
    let test_html = render("[foo`](/uri)`\n");
    let reference_html = "<p>[foo<code>](/uri)</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_525_links() {
    let test_html = render("[foo<http://example.com/?search=](uri)>\n");
    let reference_html = "<p>[foo<a href=\"http://example.com/?search=%5D(uri)\">http://example.com/?search=](uri)</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_526_links() {
    let test_html = render("[foo][bar]\n\n[bar]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_527_links() {
    let test_html = render("[link [foo [bar]]][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_528_links() {
    let test_html = render("[link \\[bar][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p><a href=\"/uri\">link [bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_529_links() {
    let test_html = render("[link *foo **bar** `#`*][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_530_links() {
    let test_html = render("[![moon](moon.jpg)][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_531_links() {
    let test_html = render("[foo [bar](/uri)][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p>[foo <a href=\"/uri\">bar</a>]<a href=\"/uri\">ref</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_532_links() {
    let test_html = render("[foo *bar [baz][ref]*][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p>[foo <em>bar <a href=\"/uri\">baz</a></em>]<a href=\"/uri\">ref</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_533_links() {
    let test_html = render("*[foo*][ref]\n\n[ref]: /uri\n");
    let reference_html = "<p>*<a href=\"/uri\">foo*</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_534_links() {
    let test_html = render("[foo *bar][ref]*\n\n[ref]: /uri\n");
    let reference_html = "<p><a href=\"/uri\">foo *bar</a>*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_535_links() {
    let test_html = render("[foo <bar attr=\"][ref]\">\n\n[ref]: /uri\n");
    let reference_html = "<p>[foo <bar attr=\"][ref]\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_536_links() {
    let test_html = render("[foo`][ref]`\n\n[ref]: /uri\n");
    let reference_html = "<p>[foo<code>][ref]</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_537_links() {
    let test_html = render("[foo<http://example.com/?search=][ref]>\n\n[ref]: /uri\n");
    let reference_html = "<p>[foo<a href=\"http://example.com/?search=%5D%5Bref%5D\">http://example.com/?search=][ref]</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_538_links() {
    let test_html = render("[foo][BaR]\n\n[bar]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_539_links() {
    let test_html = render("[ẞ]\n\n[SS]: /url\n");
    let reference_html = "<p><a href=\"/url\">ẞ</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_540_links() {
    let test_html = render("[Foo\n  bar]: /url\n\n[Baz][Foo bar]\n");
    let reference_html = "<p><a href=\"/url\">Baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_541_links() {
    let test_html = render("[foo] [bar]\n\n[bar]: /url \"title\"\n");
    let reference_html = "<p>[foo] <a href=\"/url\" title=\"title\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_542_links() {
    let test_html = render("[foo]\n[bar]\n\n[bar]: /url \"title\"\n");
    let reference_html = "<p>[foo]\n<a href=\"/url\" title=\"title\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_543_links() {
    let test_html = render("[foo]: /url1\n\n[foo]: /url2\n\n[bar][foo]\n");
    let reference_html = "<p><a href=\"/url1\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_544_links() {
    let test_html = render("[bar][foo\\!]\n\n[foo!]: /url\n");
    let reference_html = "<p>[bar][foo!]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_545_links() {
    let test_html = render("[foo][ref[]\n\n[ref[]: /uri\n");
    let reference_html = "<p>[foo][ref[]</p>\n<p>[ref[]: /uri</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_546_links() {
    let test_html = render("[foo][ref[bar]]\n\n[ref[bar]]: /uri\n");
    let reference_html = "<p>[foo][ref[bar]]</p>\n<p>[ref[bar]]: /uri</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_547_links() {
    let test_html = render("[[[foo]]]\n\n[[[foo]]]: /url\n");
    let reference_html = "<p>[[[foo]]]</p>\n<p>[[[foo]]]: /url</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_548_links() {
    let test_html = render("[foo][ref\\[]\n\n[ref\\[]: /uri\n");
    let reference_html = "<p><a href=\"/uri\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_549_links() {
    let test_html = render("[bar\\\\]: /uri\n\n[bar\\\\]\n");
    let reference_html = "<p><a href=\"/uri\">bar\\</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_550_links() {
    let test_html = render("[]\n\n[]: /uri\n");
    let reference_html = "<p>[]</p>\n<p>[]: /uri</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_551_links() {
    let test_html = render("[\n ]\n\n[\n ]: /uri\n");
    let reference_html = "<p>[\n]</p>\n<p>[\n]: /uri</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_552_links() {
    let test_html = render("[foo][]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_553_links() {
    let test_html = render("[*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_554_links() {
    let test_html = render("[Foo][]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_555_links() {
    let test_html = render("[foo] \n[]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a>\n[]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_556_links() {
    let test_html = render("[foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_557_links() {
    let test_html = render("[*foo* bar]\n\n[*foo* bar]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_558_links() {
    let test_html = render("[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"\n");
    let reference_html = "<p>[<a href=\"/url\" title=\"title\"><em>foo</em> bar</a>]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_559_links() {
    let test_html = render("[[bar [foo]\n\n[foo]: /url\n");
    let reference_html = "<p>[[bar <a href=\"/url\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_560_links() {
    let test_html = render("[Foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_561_links() {
    let test_html = render("[foo] bar\n\n[foo]: /url\n");
    let reference_html = "<p><a href=\"/url\">foo</a> bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_562_links() {
    let test_html = render("\\[foo]\n\n[foo]: /url \"title\"\n");
    let reference_html = "<p>[foo]</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_563_links() {
    let test_html = render("[foo*]: /url\n\n*[foo*]\n");
    let reference_html = "<p>*<a href=\"/url\">foo*</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_564_links() {
    let test_html = render("[foo][bar]\n\n[foo]: /url1\n[bar]: /url2\n");
    let reference_html = "<p><a href=\"/url2\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_565_links() {
    let test_html = render("[foo][]\n\n[foo]: /url1\n");
    let reference_html = "<p><a href=\"/url1\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_566_links() {
    let test_html = render("[foo]()\n\n[foo]: /url1\n");
    let reference_html = "<p><a href=\"\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_567_links() {
    let test_html = render("[foo](not a link)\n\n[foo]: /url1\n");
    let reference_html = "<p><a href=\"/url1\">foo</a>(not a link)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_568_links() {
    let test_html = render("[foo][bar][baz]\n\n[baz]: /url\n");
    let reference_html = "<p>[foo]<a href=\"/url\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_569_links() {
    let test_html = render("[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2\n");
    let reference_html = "<p><a href=\"/url2\">foo</a><a href=\"/url1\">baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_570_links() {
    let test_html = render("[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2\n");
    let reference_html = "<p>[foo]<a href=\"/url1\">bar</a></p>\n";
    assert_eq!(test_html, reference_html);
}


