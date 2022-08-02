use mini_markdown::render;
use mini_markdown::{lex, parse};
use mini_markdown::lexer::Token;


#[test]
fn test_simple_render() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", "<h1>Heading level 1</h1>"),
        ("## Heading level 2", "<h2>Heading level 2</h2>"),
        ("### Heading level 3", "<h3>Heading level 3</h3>"),
        ("#### Heading level 4", "<h4>Heading level 4</h4>"),
        ("##### Heading level 5", "<h5>Heading level 5</h5>"),
        ("###### Heading level 6", "<h6>Heading level 6</h6>"),
        ("####### Invalid Heading level 7", "<h6>Invalid Heading level 7</h6>"),
        ("Some text _with italics_ in the same paragraph\n", "<p>Some text <em>with italics</em> in the same paragraph</p>\n"),
        ("Some text! With exclamations!", "<p>Some text! With exclamations!</p>\n"),

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
        "<p>Text attributes <em>italic</em>, </p>\n<p><strong>bold</strong>, <code>monospace</code>. Some implementations may use <em>single-asterisks</em> for italic text.</p>\n"),
        ("Horizontal rule:\n\n---\n\nStrikethrough:\n\n~~strikethrough~~\n\n",
        "<p>Horizontal rule:</p>\n<hr />\n<p>Strikethrough:</p>\n<p><strike>strikethrough</strike></p>\n"
        ),
        ("> Outer quote with some text 1.\n> \n>> Inner quote with some other text\n> Outer again",
        "<blockquote>Outer quote with some text 1.<blockquote>Inner quote with some other text</blockquote>Outer again</blockquote>\n"
        ),
        ("```\nCode block 1\n```",
        "<pre><code>Code block 1\n</code></pre>"
        ),
        ("```python\nCode block 2\n```",
        "<pre><div class=\"language-python highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>Code block 2\n</code></div></div></pre>"
        ),
        ("```\nMulti\nLine\nCode block\n```",
        "<pre><code>Multi\nLine\nCode block\n</code></pre>"
        ),
        ("> Outer quote with some text.\nNon-quoted text\n> Quote with some other text",
        "<blockquote>Outer quote with some text.</blockquote><p>Non-quoted text</p>\n<blockquote>Quote with some other text</blockquote>\n"
        ),
        ("> Outer quote with some other text.\nNon-quoted text\nMore non-quoted\n> Quote with some other text",
        "<blockquote>Outer quote with some other text.</blockquote><p>Non-quoted text</p>\n<p>More non-quoted</p>\n<blockquote>Quote with some other text</blockquote>\n"
        ),
        ("Don't -> quote",
        "<p>Don&apos;t -&gt; quote</p>\n"
        ),
        ("Don't -> quote\n> Do Quote\nDon't quote this either",
        "<p>Don&apos;t -&gt; quote</p>\n<blockquote>Do Quote</blockquote><p>Don&apos;t quote this either</p>\n"
        ),
        ("Testing an inline link [Link title](http://google.com)",
        "<p>Testing an inline link <a href=\"http://google.com\" referrerpolicy=\"no-referrer\">Link title</a></p>\n"
        ),
        ("Testing an inline link to a header id [Link title](#some-header)",
        "<p>Testing an inline link to a header id <a href=\"#some-header\" referrerpolicy=\"no-referrer\">Link title</a></p>\n"
        ),
        ("Testing some details\n<details>\n<summary>Summary text goes here</summary>\nSome text goes here\n</details>",
        "<p>Testing some details</p>\n<details>\n<summary>Summary text goes here</summary>\n\n<p>Some text goes here</p>\n\n</details>"
        ),
        ("Testing some nested details <details>\n<summary>Outer summary</summary>\nOuter text<details>\n<summary>Inner Summary</summary>\nInner text\n</details>\n</details>",
        "<p>Testing some nested details </p>\n<details>\n<summary>Outer summary</summary>\n\n<p>Outer text</p>\n<details>\n<summary>Inner Summary</summary>\n\n<p>Inner text</p>\n\n</details>\n\n</details>"
        ),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("?? {:?}", lex(test.0));
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
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax2</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><p><code>Paragraph</code></p>\n</td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
        ("| Syntax3      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| *Body*      | **Text**       | ***Here's this***   |\n| `Paragraph <script=foo.js>test</script>`   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax3</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><p><code>Paragraph &lt;script=foo.js&gt;test&lt;/script&gt;</code></p>\n</td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
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
        ("![Alt text](foo.jpeg)", "<p><img src=\"foo.jpeg\" alt=\"Alt text\" referrerpolicy=\"no-referrer\"></p>"),
        ("![Alt text]()", "<p><img src=\"data:,\" alt=\"Alt text\"></p>"),
        ("![Alt text](   )", "<p><img src=\"data:,\" alt=\"Alt text\"></p>"),
        ("![Alt text](https://example.com/my/cool/image.png)", "<p><img src=\"https://example.com/my/cool/image.png\" alt=\"Alt text\" referrerpolicy=\"no-referrer\"></p>"),
        ("![Red dot](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==)", "<p><img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==\" alt=\"Red dot\" referrerpolicy=\"no-referrer\"></p>"),
        ("![Red dot](data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==)", "<p><img src=\"data:,\" alt=\"Red dot\"></p>"),

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
        "<ul class=\"contains-task-list\"><li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\" checked=\"\">One other task</li>\n<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">One task</li>\n<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">One last task</li></ul>"),
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
        ("lead text\n\n- entry 1\n- entry 2\n- entry 3\n- entry 4\n\nMore text",
        "<p>lead text</p>\n<ul><li>entry 1</li>\n<li>entry 2</li>\n<li>entry 3</li>\n<li>entry 4</li>\n</ul>\n<p>More text</p>\n"),
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
        "<p>Here's some text. And a ref <sup id=\"fnref:1\" role=\"doc-noteref\"><a href=\"#fn:1\" class=\"footnote\" rel=\"footnote\">1</a></sup></p>\n<div class=\"footnotes\" role=\"doc-endnotes\">\n\t<ol>\n\t\t<li id=\"fn:1\" role=\"doc-endnote\">\t\t\t<p>Reference text\n\twith multiple\n    lines\n  to ensure those work<a href=\"#fnref:1\" class=\"reversefootnote\" role=\"doc-backlink\">↩</a></p>\t\t</li>\t</ol>\n</div>\n"),
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
        "<p>Paragraph 1.</p>\n<pre><code>Block text should end a paragraph.\n</code></pre>\n<p>This is paragraph two.</p>\n<h2>Heading</h2>\n<p>Paragraph the third.</p>\n"),
        ("# Post title\nSection text\n# Second section\nGood content",
        "<h1>Post title</h1>\n<p>Section text</p>\n<h1>Second section</h1>\n<p>Good content</p>\n")
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
    
}

#[test]
fn test_links(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("another (See [Sewer Shark](https://en.wikipedia.org/wiki/Sewer_Shark)). Video playback",
        "<p>another &#40;See <a href=\"https://en.wikipedia.org/wiki/Sewer_Shark\" referrerpolicy=\"no-referrer\">Sewer Shark</a>&#41;. Video playback</p>\n"),
        ("r [Distant Worlds](https://www.youtube.com/watch?v=yd3KYOei8o4) a", 
        "<p>r <a href=\"https://www.youtube.com/watch?v=yd3KYOei8o4\" referrerpolicy=\"no-referrer\">Distant Worlds</a> a</p>\n"),
        ("Foo\n```\nbattle\nenemy1\n```\nSome text [ddh](https://g.com/d/ddh/t/m)\n\nMore text following",
         "<p>Foo</p>\n<pre><code>battle\nenemy1\n</code></pre>\n<p>Some text <a href=\"https://g.com/d/ddh/t/m\" referrerpolicy=\"no-referrer\">ddh</a></p>\n<p>More text following</p>\n"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

#[test]
fn test_details(){
    let mut tests = Vec::new();
    tests.extend(vec![
        ("<details>\n<summary>Summary</summary>\n\n```\nFoo\n```\n</details>",
         "<details>\n<summary>Summary</summary>\n\n<pre><code>Foo\n</code></pre>\n\n</details>"),
         ("<details>\n<summary>Summary but with spaces</summary>\n\n```\nFoo\n```\n</details>",
         "<details>\n<summary>Summary but with spaces</summary>\n\n<pre><code>Foo\n</code></pre>\n\n</details>"),
         ("<details>\r\n<summary>testing right now</summary>\r\ninner test\r\n</details>",
         "<details>\n<summary>testing right now</summary>\n\n<p>inner test\r</p>\n\n</details>"),
        ("Here's some lead text\n <details>\n<summary>Summary</summary>\n\n```\nFoo\n```\n</details>",
         "<p>Here&apos;s some lead text</p>\n<details>\n<summary>Summary</summary>\n\n<pre><code>Foo\n</code></pre>\n\n</details>")
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
    
}