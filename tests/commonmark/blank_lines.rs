use mini_markdown::render;


#[test]
fn commonmark_test_227_blank_lines() {
    let test_html = render("  \n\naaa\n  \n\n# aaa\n\n  \n");
    let reference_html = "<p>aaa</p>\n<h1>aaa</h1>\n";
    assert_eq!(test_html, reference_html);
}


