use mini_markdown::render;


#[test]
fn commonmark_test_148_html_blocks() {
    let test_html = render("<table><tr><td>\n<pre>\n**Hello**,\n\n_world_.\n</pre>\n</td></tr></table>\n");
    let reference_html = "<table><tr><td>\n<pre>\n**Hello**,\n<p><em>world</em>.\n</pre></p>\n</td></tr></table>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_149_html_blocks() {
    let test_html = render("<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n\nokay.\n");
    let reference_html = "<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n<p>okay.</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_150_html_blocks() {
    let test_html = render(" <div>\n  *hello*\n         <foo><a>\n");
    let reference_html = " <div>\n  *hello*\n         <foo><a>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_151_html_blocks() {
    let test_html = render("</div>\n*foo*\n");
    let reference_html = "</div>\n*foo*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_152_html_blocks() {
    let test_html = render("<DIV CLASS=\"foo\">\n\n*Markdown*\n\n</DIV>\n");
    let reference_html = "<DIV CLASS=\"foo\">\n<p><em>Markdown</em></p>\n</DIV>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_153_html_blocks() {
    let test_html = render("<div id=\"foo\"\n  class=\"bar\">\n</div>\n");
    let reference_html = "<div id=\"foo\"\n  class=\"bar\">\n</div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_154_html_blocks() {
    let test_html = render("<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n");
    let reference_html = "<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_155_html_blocks() {
    let test_html = render("<div>\n*foo*\n\n*bar*\n");
    let reference_html = "<div>\n*foo*\n<p><em>bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_156_html_blocks() {
    let test_html = render("<div id=\"foo\"\n*hi*\n");
    let reference_html = "<div id=\"foo\"\n*hi*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_157_html_blocks() {
    let test_html = render("<div class\nfoo\n");
    let reference_html = "<div class\nfoo\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_158_html_blocks() {
    let test_html = render("<div *???-&&&-<---\n*foo*\n");
    let reference_html = "<div *???-&&&-<---\n*foo*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_159_html_blocks() {
    let test_html = render("<div><a href=\"bar\">*foo*</a></div>\n");
    let reference_html = "<div><a href=\"bar\">*foo*</a></div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_160_html_blocks() {
    let test_html = render("<table><tr><td>\nfoo\n</td></tr></table>\n");
    let reference_html = "<table><tr><td>\nfoo\n</td></tr></table>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_161_html_blocks() {
    let test_html = render("<div></div>\n``` c\nint x = 33;\n```\n");
    let reference_html = "<div></div>\n``` c\nint x = 33;\n```\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_162_html_blocks() {
    let test_html = render("<a href=\"foo\">\n*bar*\n</a>\n");
    let reference_html = "<a href=\"foo\">\n*bar*\n</a>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_163_html_blocks() {
    let test_html = render("<Warning>\n*bar*\n</Warning>\n");
    let reference_html = "<Warning>\n*bar*\n</Warning>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_164_html_blocks() {
    let test_html = render("<i class=\"foo\">\n*bar*\n</i>\n");
    let reference_html = "<i class=\"foo\">\n*bar*\n</i>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_165_html_blocks() {
    let test_html = render("</ins>\n*bar*\n");
    let reference_html = "</ins>\n*bar*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_166_html_blocks() {
    let test_html = render("<del>\n*foo*\n</del>\n");
    let reference_html = "<del>\n*foo*\n</del>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_167_html_blocks() {
    let test_html = render("<del>\n\n*foo*\n\n</del>\n");
    let reference_html = "<del>\n<p><em>foo</em></p>\n</del>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_168_html_blocks() {
    let test_html = render("<del>*foo*</del>\n");
    let reference_html = "<p><del><em>foo</em></del></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_169_html_blocks() {
    let test_html = render("<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\nokay\n");
    let reference_html = "<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_170_html_blocks() {
    let test_html = render("<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\nokay\n");
    let reference_html = "<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_171_html_blocks() {
    let test_html = render("<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n");
    let reference_html = "<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_172_html_blocks() {
    let test_html = render("<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\nokay\n");
    let reference_html = "<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_173_html_blocks() {
    let test_html = render("<style\n  type=\"text/css\">\n\nfoo\n");
    let reference_html = "<style\n  type=\"text/css\">\n\nfoo\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_174_html_blocks() {
    let test_html = render("> <div>\n> foo\n\nbar\n");
    let reference_html = "<blockquote>\n<div>\nfoo\n</blockquote>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_175_html_blocks() {
    let test_html = render("- <div>\n- foo\n");
    let reference_html = "<ul>\n<li>\n<div>\n</li>\n<li>foo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_176_html_blocks() {
    let test_html = render("<style>p{color:red;}</style>\n*foo*\n");
    let reference_html = "<style>p{color:red;}</style>\n<p><em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_177_html_blocks() {
    let test_html = render("<!-- foo -->*bar*\n*baz*\n");
    let reference_html = "<!-- foo -->*bar*\n<p><em>baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_178_html_blocks() {
    let test_html = render("<script>\nfoo\n</script>1. *bar*\n");
    let reference_html = "<script>\nfoo\n</script>1. *bar*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_179_html_blocks() {
    let test_html = render("<!-- Foo\n\nbar\n   baz -->\nokay\n");
    let reference_html = "<!-- Foo\n\nbar\n   baz -->\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_180_html_blocks() {
    let test_html = render("<?php\n\n  echo '>';\n\n?>\nokay\n");
    let reference_html = "<?php\n\n  echo '>';\n\n?>\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_181_html_blocks() {
    let test_html = render("<!DOCTYPE html>\n");
    let reference_html = "<!DOCTYPE html>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_182_html_blocks() {
    let test_html = render("<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\nokay\n");
    let reference_html = "<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\n<p>okay</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_183_html_blocks() {
    let test_html = render("  <!-- foo -->\n\n    <!-- foo -->\n");
    let reference_html = "  <!-- foo -->\n<pre><code>&lt;!-- foo --&gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_184_html_blocks() {
    let test_html = render("  <div>\n\n    <div>\n");
    let reference_html = "  <div>\n<pre><code>&lt;div&gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_185_html_blocks() {
    let test_html = render("Foo\n<div>\nbar\n</div>\n");
    let reference_html = "<p>Foo</p>\n<div>\nbar\n</div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_186_html_blocks() {
    let test_html = render("<div>\nbar\n</div>\n*foo*\n");
    let reference_html = "<div>\nbar\n</div>\n*foo*\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_187_html_blocks() {
    let test_html = render("Foo\n<a href=\"bar\">\nbaz\n");
    let reference_html = "<p>Foo\n<a href=\"bar\">\nbaz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_188_html_blocks() {
    let test_html = render("<div>\n\n*Emphasized* text.\n\n</div>\n");
    let reference_html = "<div>\n<p><em>Emphasized</em> text.</p>\n</div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_189_html_blocks() {
    let test_html = render("<div>\n*Emphasized* text.\n</div>\n");
    let reference_html = "<div>\n*Emphasized* text.\n</div>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_190_html_blocks() {
    let test_html = render("<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>\n");
    let reference_html = "<table>\n<tr>\n<td>\nHi\n</td>\n</tr>\n</table>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_191_html_blocks() {
    let test_html = render("<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>\n");
    let reference_html = "<table>\n  <tr>\n<pre><code>&lt;td&gt;\n  Hi\n&lt;/td&gt;\n</code></pre>\n  </tr>\n</table>\n";
    assert_eq!(test_html, reference_html);
}


