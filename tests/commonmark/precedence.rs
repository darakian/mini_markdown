use mini_markdown::render;


#[test]
fn commonmark_test_42_precedence() {
    let test_html = render("- `one\n- two`\n");
    let reference_html = "<ul>\n<li>`one</li>\n<li>two`</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


