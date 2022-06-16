use mini_markdown::render;


#[test]
fn commonmark_test_612_raw_html() {
    let test_html = render("<a><bab><c2c>\n");
    let reference_html = "<p><a><bab><c2c></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_613_raw_html() {
    let test_html = render("<a/><b2/>\n");
    let reference_html = "<p><a/><b2/></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_614_raw_html() {
    let test_html = render("<a  /><b2\ndata=\"foo\" >\n");
    let reference_html = "<p><a  /><b2\ndata=\"foo\" ></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_615_raw_html() {
    let test_html = render("<a foo=\"bar\" bam = \'baz <em>\"</em>\'\n_boolean zoop:33=zoop:33 />\n");
    let reference_html = "<p><a foo=\"bar\" bam = \'baz <em>\"</em>\'\n_boolean zoop:33=zoop:33 /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_616_raw_html() {
    let test_html = render("Foo <responsive-image src=\"foo.jpg\" />\n");
    let reference_html = "<p>Foo <responsive-image src=\"foo.jpg\" /></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_617_raw_html() {
    let test_html = render("<33> <__>\n");
    let reference_html = "<p>&lt;33&gt; &lt;__&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_618_raw_html() {
    let test_html = render("<a h*#ref=\"hi\">\n");
    let reference_html = "<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_619_raw_html() {
    let test_html = render("<a href=\"hi\'> <a href=hi\'>\n");
    let reference_html = "<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_620_raw_html() {
    let test_html = render("< a><\nfoo><bar/ >\n<foo bar=baz\nbim!bop />\n");
    let reference_html = "<p>&lt; a&gt;&lt;\nfoo&gt;&lt;bar/ &gt;\n&lt;foo bar=baz\nbim!bop /&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_621_raw_html() {
    let test_html = render("<a href='bar'title=title>\n");
    let reference_html = "<p>&lt;a href='bar'title=title&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_622_raw_html() {
    let test_html = render("</a></foo >\n");
    let reference_html = "<p></a></foo ></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_623_raw_html() {
    let test_html = render("</a href=\"foo\">\n");
    let reference_html = "<p>&lt;/a href=&quot;foo&quot;&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_624_raw_html() {
    let test_html = render("foo <!-- this is a\ncomment - with hyphen -->\n");
    let reference_html = "<p>foo <!-- this is a\ncomment - with hyphen --></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_625_raw_html() {
    let test_html = render("foo <!-- not a comment -- two hyphens -->\n");
    let reference_html = "<p>foo &lt;!-- not a comment -- two hyphens --&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_626_raw_html() {
    let test_html = render("foo <!--> foo -->\n\nfoo <!-- foo--->\n");
    let reference_html = "<p>foo &lt;!--&gt; foo --&gt;</p>\n<p>foo &lt;!-- foo---&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_627_raw_html() {
    let test_html = render("foo <?php echo $a; ?>\n");
    let reference_html = "<p>foo <?php echo $a; ?></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_628_raw_html() {
    let test_html = render("foo <!ELEMENT br EMPTY>\n");
    let reference_html = "<p>foo <!ELEMENT br EMPTY></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_629_raw_html() {
    let test_html = render("foo <![CDATA[>&<]]>\n");
    let reference_html = "<p>foo <![CDATA[>&<]]></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_630_raw_html() {
    let test_html = render("foo <a href=\"&ouml;\">\n");
    let reference_html = "<p>foo <a href=\"&ouml;\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_631_raw_html() {
    let test_html = render("foo <a href=\"\\*\">\n");
    let reference_html = "<p>foo <a href=\"\\*\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_632_raw_html() {
    let test_html = render("<a href=\"\\\"\">\n");
    let reference_html = "<p>&lt;a href=&quot;&quot;&quot;&gt;</p>\n";
    assert_eq!(test_html, reference_html);
}


