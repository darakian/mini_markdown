use mini_markdown::render;


#[test]
fn commonmark_test_119_fenced_code_blocks() {
    let test_html = render("```\n<\n >\n```\n");
    let reference_html = "<pre><code>&lt;\n &gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_120_fenced_code_blocks() {
    let test_html = render("~~~\n<\n >\n~~~\n");
    let reference_html = "<pre><code>&lt;\n &gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_121_fenced_code_blocks() {
    let test_html = render("``\nfoo\n``\n");
    let reference_html = "<p><code>foo</code></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_122_fenced_code_blocks() {
    let test_html = render("```\naaa\n~~~\n```\n");
    let reference_html = "<pre><code>aaa\n~~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_123_fenced_code_blocks() {
    let test_html = render("~~~\naaa\n```\n~~~\n");
    let reference_html = "<pre><code>aaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_124_fenced_code_blocks() {
    let test_html = render("````\naaa\n```\n``````\n");
    let reference_html = "<pre><code>aaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_125_fenced_code_blocks() {
    let test_html = render("~~~~\naaa\n~~~\n~~~~\n");
    let reference_html = "<pre><code>aaa\n~~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_126_fenced_code_blocks() {
    let test_html = render("```\n");
    let reference_html = "<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_127_fenced_code_blocks() {
    let test_html = render("`````\n\n```\naaa\n");
    let reference_html = "<pre><code>\n```\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_128_fenced_code_blocks() {
    let test_html = render("> ```\n> aaa\n\nbbb\n");
    let reference_html = "<blockquote>\n<pre><code>aaa\n</code></pre>\n</blockquote>\n<p>bbb</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_129_fenced_code_blocks() {
    let test_html = render("```\n\n  \n```\n");
    let reference_html = "<pre><code>\n  \n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_130_fenced_code_blocks() {
    let test_html = render("```\n```\n");
    let reference_html = "<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_131_fenced_code_blocks() {
    let test_html = render(" ```\n aaa\naaa\n```\n");
    let reference_html = "<pre><code>aaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_132_fenced_code_blocks() {
    let test_html = render("  ```\naaa\n  aaa\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\naaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_133_fenced_code_blocks() {
    let test_html = render("   ```\n   aaa\n    aaa\n  aaa\n   ```\n");
    let reference_html = "<pre><code>aaa\n aaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_134_fenced_code_blocks() {
    let test_html = render("    ```\n    aaa\n    ```\n");
    let reference_html = "<pre><code>```\naaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_135_fenced_code_blocks() {
    let test_html = render("```\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_136_fenced_code_blocks() {
    let test_html = render("   ```\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_137_fenced_code_blocks() {
    let test_html = render("```\naaa\n    ```\n");
    let reference_html = "<pre><code>aaa\n    ```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_138_fenced_code_blocks() {
    let test_html = render("``` ```\naaa\n");
    let reference_html = "<p><code> </code>\naaa</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_139_fenced_code_blocks() {
    let test_html = render("~~~~~~\naaa\n~~~ ~~\n");
    let reference_html = "<pre><code>aaa\n~~~ ~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_140_fenced_code_blocks() {
    let test_html = render("foo\n```\nbar\n```\nbaz\n");
    let reference_html = "<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_141_fenced_code_blocks() {
    let test_html = render("foo\n---\n~~~\nbar\n~~~\n# baz\n");
    let reference_html = "<h2>foo</h2>\n<pre><code>bar\n</code></pre>\n<h1>baz</h1>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_142_fenced_code_blocks() {
    let test_html = render("```ruby\ndef foo(x)\n  return 3\nend\n```\n");
    let reference_html = "<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_143_fenced_code_blocks() {
    let test_html = render("~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~\n");
    let reference_html = "<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_144_fenced_code_blocks() {
    let test_html = render("````;\n````\n");
    let reference_html = "<pre><code class=\"language-;\"></code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_145_fenced_code_blocks() {
    let test_html = render("``` aa ```\nfoo\n");
    let reference_html = "<p><code>aa</code>\nfoo</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_146_fenced_code_blocks() {
    let test_html = render("~~~ aa ``` ~~~\nfoo\n~~~\n");
    let reference_html = "<pre><code class=\"language-aa\">foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_147_fenced_code_blocks() {
    let test_html = render("```\n``` aaa\n```\n");
    let reference_html = "<pre><code>``` aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


