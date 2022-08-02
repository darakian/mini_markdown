use mini_markdown::render;


#[test]
fn commonmark_test_25_entity_and_numeric_character_references() {
    let test_html = render("&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;\n");
    let reference_html = "<p>  &amp; © Æ Ď\n¾ ℋ ⅆ\n∲ ≧̸</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_26_entity_and_numeric_character_references() {
    let test_html = render("&#35; &#1234; &#992; &#0;\n");
    let reference_html = "<p># Ӓ Ϡ �</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_27_entity_and_numeric_character_references() {
    let test_html = render("&#X22; &#XD06; &#xcab;\n");
    let reference_html = "<p>&quot; ആ ಫ</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_28_entity_and_numeric_character_references() {
    let test_html = render("&nbsp &x; &#; &#x;\n&#87654321;\n&#abcdef0;\n&ThisIsNotDefined; &hi?;\n");
    let reference_html = "<p>&amp;nbsp &amp;x; &amp;#; &amp;#x;\n&amp;#87654321;\n&amp;#abcdef0;\n&amp;ThisIsNotDefined; &amp;hi?;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_29_entity_and_numeric_character_references() {
    let test_html = render("&copy\n");
    let reference_html = "<p>&amp;copy</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_30_entity_and_numeric_character_references() {
    let test_html = render("&MadeUpEntity;\n");
    let reference_html = "<p>&amp;MadeUpEntity;</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_31_entity_and_numeric_character_references() {
    let test_html = render("<a href=\"&ouml;&ouml;.html\">\n");
    let reference_html = "<a href=\"&ouml;&ouml;.html\">\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_32_entity_and_numeric_character_references() {
    let test_html = render("[foo](/f&ouml;&ouml; \"f&ouml;&ouml;\")\n");
    let reference_html = "<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_33_entity_and_numeric_character_references() {
    let test_html = render("[foo]\n\n[foo]: /f&ouml;&ouml; \"f&ouml;&ouml;\"\n");
    let reference_html = "<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_34_entity_and_numeric_character_references() {
    let test_html = render("``` f&ouml;&ouml;\nfoo\n```\n");
    let reference_html = "<pre><code class=\"language-föö\">foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_35_entity_and_numeric_character_references() {
    let test_html = render("`f&ouml;&ouml;`\n");
    let reference_html = "<p><code>f&amp;ouml;&amp;ouml;</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_36_entity_and_numeric_character_references() {
    let test_html = render("    f&ouml;f&ouml;\n");
    let reference_html = "<pre><code>f&amp;ouml;f&amp;ouml;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_37_entity_and_numeric_character_references() {
    let test_html = render("&#42;foo&#42;\n*foo*\n");
    let reference_html = "<p>*foo*\n<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_38_entity_and_numeric_character_references() {
    let test_html = render("&#42; foo\n\n* foo\n");
    let reference_html = "<p>* foo</p>\n<ul>\n<li>foo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_39_entity_and_numeric_character_references() {
    let test_html = render("foo&#10;&#10;bar\n");
    let reference_html = "<p>foo\n\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_40_entity_and_numeric_character_references() {
    let test_html = render("&#9;foo\n");
    let reference_html = "<p>\tfoo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_41_entity_and_numeric_character_references() {
    let test_html = render("[a](url &quot;tit&quot;)\n");
    let reference_html = "<p>[a](url &quot;tit&quot;)</p>\n";
    assert_eq!(test_html, reference_html);
}


