use mini_markdown::render;
use mini_markdown::lex;


#[test]
fn test_simple_render() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", "<h1 id=\"heading-level-1\">Heading level 1</h1>\n"),
        ("## Heading level 2", "<h2 id=\"heading-level-2\">Heading level 2</h2>\n"),
        ("### Heading level 3", "<h3 id=\"heading-level-3\">Heading level 3</h3>\n"),
        ("#### Heading level 4", "<h4 id=\"heading-level-4\">Heading level 4</h4>\n"),
        ("##### Heading level 5", "<h5 id=\"heading-level-5\">Heading level 5</h5>\n"),
        ("###### Heading level 6", "<h6 id=\"heading-level-6\">Heading level 6</h6>\n"),
        ("####### Invalid Heading level 7", "<h6 id=\"invalid-heading-level-7\">Invalid Heading level 7</h6>\n"),
        ("Some text _with italics_ in the same paragraph\n", "<p>Some text <em>with italics</em> in the same paragraph\n</p>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_moderate_render(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("Text attributes _italic_, \n**bold**, `monospace`. Some implementations may use *single-asterisks* for italic text.",
        "<p>Text attributes <em>italic</em>, \n<strong>bold</strong>, <code>monospace</code>. Some implementations may use <em>single-asterisks</em> for italic text.</p>"),
        ("Horizontal rule:\n\n---\n\nStrikethrough:\n\n~~strikethrough~~\n\n",
        "<p>Horizontal rule:</p>\n<hr /><p>Strikethrough:</p>\n<strike>strikethrough</strike>"
        ),
        ("> Outer quote with some text.\n> \n>> Inner quote with some other text",
        "<blockquote>\n  <p>Outer quote with some text.</p>\n \n  <blockquote>\n    <p>Inner quote with some other text</p>\n  </blockquote>\n</blockquote>\n"
        )
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

use std::fs;

#[test]
fn test_full_render(){
    // let tests_dir = fs::read_dir("tests/pages").expect("Error opening test dir");
    // for entry in tests_dir{
    //     if !entry.is_ok() {
    //         continue;
    //     }
    //     let e = entry.unwrap();
    //     if e.path().extension().unwrap() == "md"{
    //         println!("Testing: {:?}", e.path());
    //         let markdown = fs::read_to_string(e.path()).expect("Error reading markdown");
    //         let associated_html: String = e.path().to_str().unwrap().replace(".md", ".html");
    //         let html = fs::read_to_string(associated_html).expect("Error reading html");
    //         assert_eq!(render(&markdown), html)
    //     }
    // }
}