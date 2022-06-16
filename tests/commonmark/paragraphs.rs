use mini_markdown::render;


#[test]
fn commonmark_test_219_paragraphs() {
    let test_html = render("aaa\n\nbbb\n");
    let reference_html = "<p>aaa</p>\n<p>bbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_220_paragraphs() {
    let test_html = render("aaa\nbbb\n\nccc\nddd\n");
    let reference_html = "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_221_paragraphs() {
    let test_html = render("aaa\n\n\nbbb\n");
    let reference_html = "<p>aaa</p>\n<p>bbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_222_paragraphs() {
    let test_html = render("  aaa\n bbb\n");
    let reference_html = "<p>aaa\nbbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_223_paragraphs() {
    let test_html = render("aaa\n             bbb\n                                       ccc\n");
    let reference_html = "<p>aaa\nbbb\nccc</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_224_paragraphs() {
    let test_html = render("   aaa\nbbb\n");
    let reference_html = "<p>aaa\nbbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_225_paragraphs() {
    let test_html = render("    aaa\nbbb\n");
    let reference_html = "<pre><code>aaa\n</code></pre>\n<p>bbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_226_paragraphs() {
    let test_html = render("aaa     \nbbb     \n");
    let reference_html = "<p>aaa<br />\nbbb</p>\n";
    assert_eq!(test_html, reference_html);
}


