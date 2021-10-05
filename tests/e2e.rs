use mini_markdown::render;
// use mini_markdown::lex;


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
        "<p>Horizontal rule:\n<hr />\nStrikethrough:\n<strike>strikethrough</strike>\n</p>"
        ),
        ("> Outer quote with some text 1.\n> \n>> Inner quote with some other text\n> Outer again",
        "<blockquote>Outer quote with some text 1.\n\n<blockquote>Inner quote with some other text\n</blockquote>Outer again</blockquote>"
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
        "<blockquote>Outer quote with some text.\n</blockquote><p>Non-quoted text\n</p><blockquote>Quote with some other text</blockquote>"
        ),
        ("> Outer quote with some text.\nNon-quoted text\nMore non-quoted\n> Quote with some other text",
        "<blockquote>Outer quote with some text.\n</blockquote><p>Non-quoted text\nMore non-quoted\n</p><blockquote>Quote with some other text</blockquote>"
        ),
        ("Don't -> quote",
        "<p>Don&apos;t -&gt; quote</p>"
        ),
        ("Don't -> quote\n> Do Quote\nDon't quote this either",
        "<p>Don&apos;t -&gt; quote\n</p><blockquote>Do Quote\n</blockquote><p>Don&apos;t quote this either</p>"
        ),
        ("Testing an inline link [Link title](http://google.com)",
        "<p>Testing an inline link <a href=\"http://google.com\">Link title</a></p>"
        ),
        ("Testing an inline link to a header id [Link title](#some-header)",
        "<p>Testing an inline link to a header id <a href=\"#some-header\">Link title</a></p>"
        ),
        ("Testing some details <details>\n<summary markdown=\"span\">Summary text goes here</summary>\nSome text goes here\n</details>",
        "<p>Testing some details <details>\n<summary>Summary text goes here</summary>\n\n<p>Some text goes here\n</p>\n</details></p>"
        ),
        ("Testing some nested details <details>\n<summary markdown=\"span\">Outer summary</summary>\nOuter text<details>\n<summary markdown=\"span\">Inner Summary</summary>\nInner text\n</details>\n</details>",
        "<p>Testing some nested details <details>\n<summary>Outer summary</summary>\n\n<p>Outer text<details>\n<summary>Inner Summary</summary>\n\n<p>Inner text\n</p>\n</details>\n</p>\n</details></p>"
        ),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("Test failing\n{:?}\n{:?}", html, test.1);
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
        ("| Syntax      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| *Body*      | **Text**       | ***Here's this***   |\n| `Paragraph`   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><code>Paragraph</code></td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
        ("| Syntax      | Description | Test Text     |\n| :---        |    :----:   |          ---: |\n| *Body*      | **Text**       | ***Here's this***   |\n| `Paragraph <script=foo.js>test</script>`   | Text        | And more      |", 
        "<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n\t\t<th style=\"text-align: left\">Syntax</th>\t\t<th style=\"text-align: center\">Description</th>\t\t<th style=\"text-align: right\">Test Text</th>\t</tr>\n\t</thead>\n\t<tbody>\n\t<tr>\n\t\t<td style=\"text-align: left\"><em>Body</em></td>\n\t\t<td style=\"text-align: center\"><strong>Text</strong></td>\n\t\t<td style=\"text-align: right\"><strong><em>Here&apos;s this</em></strong></td>\n\t</tr>\n\t<tr>\n\t\t<td style=\"text-align: left\"><code>Paragraph &lt;script=foo.js&gt;test&lt;/script&gt;</code></td>\n\t\t<td style=\"text-align: center\">Text</td>\n\t\t<td style=\"text-align: right\">And more</td>\n\t</tr>\n\t</tbody>\n</table>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        if html != test.1 {
            println!("Test failing\n{:?}\n{:?}", html, test.1);
            for (c1, c2) in test.1.chars().zip(html.chars()) {
                if c1 != c2 {
                    println!("Difference in {:?} {:?}", c1, c2);
                }
            }
        }
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