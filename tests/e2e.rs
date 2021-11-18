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
        ("Some text _with italics_ in the same paragraph\n", "<p>Some text <em>with italics</em>in the same paragraph\n</p>\n"),
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
        "<p>Text attributes <em>italic</em>, \n<strong>bold</strong>, <code>monospace</code>. Some implementations may use <em>single-asterisks</em>for italic text.</p>\n"),
        ("Horizontal rule:\n\n---\n\nStrikethrough:\n\n~~strikethrough~~\n\n",
        "<p>Horizontal rule:</p>\n<hr />\n<p>Strikethrough:</p>\n<p><strike>strikethrough</strike></p>\n"
        ),
        ("> Outer quote with some text 1.\n> \n>> Inner quote with some other text\n> Outer again",
        "<blockquote>Outer quote with some text 1.<blockquote>Inner quote with some other text</blockquote>Outer again</blockquote>\n"
        ),
        ("```\nCode block 1\n```",
        "<div class=\"language-plaintext highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>Code block 1\n</code></pre></div></div>"
        ),
        ("```python\nCode block 2\n```",
        "<div class=\"language-python highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>Code block 2\n</code></pre></div></div>"
        ),
        ("```\nMulti\nLine\nCode block\n```",
        "<div class=\"language-plaintext highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>Multi\nLine\nCode block\n</code></pre></div></div>"
        ),
        ("> Outer quote with some text.\nNon-quoted text\n> Quote with some other text",
        "<blockquote>Outer quote with some text.</blockquote><p>Non-quoted text\n</p><blockquote>Quote with some other text</blockquote>\n"
        ),
        ("> Outer quote with some other text.\nNon-quoted text\nMore non-quoted\n> Quote with some other text",
        "<blockquote>Outer quote with some other text.</blockquote><p>Non-quoted text\nMore non-quoted\n</p><blockquote>Quote with some other text</blockquote>\n"
        ),
        ("Don't -> quote",
        "<p>Don&apos;t -&gt; quote</p>\n"
        ),
        ("Don't -> quote\n> Do Quote\nDon't quote this either",
        "<p>Don&apos;t -&gt; quote\n</p><blockquote>Do Quote</blockquote><p>Don&apos;t quote this either</p>\n"
        ),
        ("Testing an inline link [Link title](http://google.com)",
        "<p>Testing an inline link <a href=\"http://google.com\" referrerpolicy=\"no-referrer\">Link title</a></p>\n"
        ),
        ("Testing an inline link to a header id [Link title](#some-header)",
        "<p>Testing an inline link to a header id <a href=\"#some-header\" referrerpolicy=\"no-referrer\">Link title</a></p>\n"
        ),
        ("Testing some details <details>\n<summary markdown=\"span\">Summary text goes here</summary>\nSome text goes here\n</details>",
        "<p>Testing some details <details>\n<summary>Summary text goes here</summary>\n<p>Some text goes here\n</p>\n\n</details></p>\n"
        ),
        ("Testing some nested details <details>\n<summary markdown=\"span\">Outer summary</summary>\nOuter text<details>\n<summary markdown=\"span\">Inner Summary</summary>\nInner text\n</details>\n</details>",
        "<p>Testing some nested details <details>\n<summary>Outer summary</summary>\n<p>Outer text<details>\n<summary>Inner Summary</summary>\n<p>Inner text\n</p>\n\n</details></p>\n\n</details></p>\n"
        ),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("Test failing\n{:?}\n{:?}", html, test.1);
            println!("{:?}", lex(test.0));
            for (c1, c2) in test.1.chars().zip(html.chars()) {
                if c1 != c2 {
                    println!("Difference in {:?} {:?}", c1, c2);
                }
            }
        }
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_table_render() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("| Syntax      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| Body      | Text       | Here's this   |\n| Paragraph   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\">Body</td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">Here&apos;s this</td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\">Paragraph</td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
        ("| Syntax2      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| *Body*      | **Text**       | ***Here's this***   |\n| `Paragraph`   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax2</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><code>Paragraph</code></td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
        ("| Syntax3      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| *Body*      | **Text**       | ***Here's this***   |\n| `Paragraph <script=foo.js>test</script>`   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax3</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><code>Paragraph &lt;script=foo.js&gt;test&lt;/script&gt;</code></td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("Test failing\n{:?}\n{:?}", html, test.1);
            println!("> {:?}", lex(test.0));
            for (c1, c2) in test.1.chars().zip(html.chars()) {
                if c1 != c2 {
                    println!("Difference in {:?} {:?}", c1, c2);
                }
            }
        }
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_images(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("![Alt text](foo.jpeg)", "<img src=\"foo.jpeg\" alt=\"Alt text\" referrerpolicy=\"no-referrer\">"),
        ("![Alt text]()", "<img src=\"data:,\" alt=\"Alt text\">"),
        ("![Alt text](   )", "<img src=\"data:,\" alt=\"Alt text\">"),
        ("![Alt text](https://example.com/my/cool/image.png)", "<img src=\"https://example.com/my/cool/image.png\" alt=\"Alt text\" referrerpolicy=\"no-referrer\">"),
        ("![Red dot](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==)", "<img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==\" alt=\"Red dot\" referrerpolicy=\"no-referrer\">"),
        ("![Red dot](data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==)", "<img src=\"data:,\" alt=\"Red dot\">"),

    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_tasklists(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("- [ ] One task",
        "<ul class=\"contains-task-list\"><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">One task</li></ul>"),
        ("- [x] One other task",
        "<ul class=\"contains-task-list\"><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\" checked=\"\">One other task</li></ul>"),
        ("- [x] One other task\n- [ ] One task\n- [ ] One last task",
        "<ul class=\"contains-task-list\"><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\" checked=\"\">One other task</li><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">One task</li><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">One last task</li></ul>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_lists(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("* One entry",
        "<ul><li>One entry</li></ul>"),
        ("1. One entry",
        "<ol><li>One entry</li></ol>"),
        ("* an item\n* another item\n* a third item",
        "<ul><li>an item</li><li>another item</li><li>a third item</li></ul>"),
        (" * an item\n * another item\n * a third item",
        "<ul><li>an item</li><li>another item</li><li>a third item</li></ul>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("> {:?}", lex(test.0));
        }
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_blockquotes(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("> Outer quote with some text 1.\n> \n>> Inner quote with some other text\n> Outer again",
        "<blockquote>Outer quote with some text 1.<blockquote>Inner quote with some other text</blockquote>Outer again</blockquote>\n"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_references(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("Here's some text. And a ref [^1]\n [^1]: Reference text",
        "<p>Here's some text. And a ref <sup id=\"fnref:1\" role=\"doc-noteref\"><a href=\"#fn:1\" class=\"footnote\" rel=\"footnote\">1</a></sup></p>\n<div class=\"footnotes\" role=\"doc-endnotes\">\n\t<ol>\n\t\t<li id=\"fn:1\" role=\"doc-endnote\">\t\t\t<p>Reference text<a href=\"#fnref:1\" class=\"reversefootnote\" role=\"doc-backlink\">↩</a></p>\t\t</li>\t</ol>\n</div>\n"),
        ("Here's some text. And a ref [^1]\n [^1]: Reference text\n\twith multiple\n    lines\n  to ensure those work",
        "<p>Here's some text. And a ref <sup id=\"fnref:1\" role=\"doc-noteref\"><a href=\"#fn:1\" class=\"footnote\" rel=\"footnote\">1</a></sup></p>\n<div class=\"footnotes\" role=\"doc-endnotes\">\n\t<ol>\n\t\t<li id=\"fn:1\" role=\"doc-endnote\">\t\t\t<p>Reference text\nwith multiple\nlines\nto ensure those work<a href=\"#fnref:1\" class=\"reversefootnote\" role=\"doc-backlink\">↩</a></p>\t\t</li>\t</ol>\n</div>\n"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_paragraphs(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("Paragraph 1.\n\n```\nBlock text should end a paragraph.\n```\n\nThis is paragraph two.\n\n## Heading\n\nParagraph the third.",
        "<p>Paragraph 1.</p>\n<div class=\"language-plaintext highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>Block text should end a paragraph.\n</code></pre></div></div>\n<p>This is paragraph two.</p>\n<h2 id=\"heading\">Heading</h2>\n\n<p>Paragraph the third.</p>\n"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
    
}

// use std::fs;

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