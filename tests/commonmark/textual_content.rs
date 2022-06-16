use mini_markdown::render;


#[test]
fn commonmark_test_650_textual_content() {
    let test_html = render("hello $.;'there\n");
    let reference_html = "<p>hello $.;'there</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_651_textual_content() {
    let test_html = render("Foo χρῆν\n");
    let reference_html = "<p>Foo χρῆν</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_652_textual_content() {
    let test_html = render("Multiple     spaces\n");
    let reference_html = "<p>Multiple     spaces</p>\n";
    assert_eq!(test_html, reference_html);
}


