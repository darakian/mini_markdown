pub mod lexer;
pub mod iter;
use crate::lexer::*;
use crate::iter::MiniIter;

#[derive(Debug)]
pub(crate) struct SanitizationError{
    pub(crate) content: String,
}

fn on_newline(tokens: &Vec<Token>) -> bool {
    match tokens.last() {
        Some(Token::Plaintext(t)) => t.ends_with("\n"),
        _ => true,
    }
}

/// Convert source markdown to an ordered vector of tokens
pub fn lex(source: &str) -> Vec<Token>{
    let mut char_iter = MiniIter::new(source);
    let mut tokens = Vec::new();
    while char_iter.peek().is_some(){
        match char_iter.peek().unwrap(){
            "#" if on_newline(&tokens) => {
                match lex_heading(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "*" | "_" => {
                match lex_asterisk_underscore(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "~" => {
                match lex_tilde(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "-" | "+" => {
                match lex_plus_minus(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            " " | "\t" => {
                match lex_tabs_spaces(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "`" => {
                match lex_backticks(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "\n" => {
                match lex_newlines(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            ">" => {
                match lex_blockquotes(&mut char_iter) {
                    Ok(t) => {
                        tokens.push(t);
                        },
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "!" => {
                match lex_images(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "[" => {
                match lex_links(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "<" => {
                match lex_side_carrot(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "|" => {
                match lex_pipes(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => {
                match lex_numbers(&mut char_iter) {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            }
            // Parse "\" to escape a markdown control character
            "\\" => {
                char_iter.next();
                if char_iter.peek() == Some(&"#"){
                    let hashes = char_iter.consume_while_case_holds(&|c| c == "#").unwrap_or("");
                    push_str(&mut tokens, hashes);
                } else if char_iter.peek().is_some(){
                    push_str(&mut tokens, char_iter.next().unwrap());
                }
            }
            _ => {
                push_str(&mut tokens, char_iter.next().unwrap());
            },
        }
    }
    tokens
}

/// Parse tokens to produce safe html output
pub fn parse(tokens: &[Token]) -> String {
    let mut html = String::with_capacity(tokens.len()*100);
    let mut in_task_list = false;
    let mut in_ordered_list = false;
    let mut in_unordered_list = false;
    let mut in_paragraph = false;
    let mut quote_level = 0;
    let mut references = Vec::new();
    for token in tokens.iter(){
        // Handle multi-liners
        match token {
            Token::Plaintext(t) if t.trim().is_empty() => {}, //Ignore empty plaintext tokens 
            Token::Tab | Token::DoubleTab => {},
            Token::OrderedListEntry(_) | Token::UnorderedListEntry(_) | Token::Newline if in_ordered_list | in_unordered_list => {},
            Token::TaskListItem(_, _)  | Token::Newline if in_task_list => {},
            _ if in_ordered_list => {
                in_ordered_list = false;
                html.push_str("</ol>\n");
                if !in_paragraph {
                    in_paragraph = true;
                    html.push_str("<p>") 
                }
            },
            _ if in_unordered_list => {
                in_unordered_list = false;
                html.push_str("</ul>\n");
                if !in_paragraph {
                    in_paragraph = true;
                    html.push_str("<p>") 
                }
            },
            _ if in_task_list => {
                in_task_list = false;
                html.push_str("</ul>\n");
                if !in_paragraph {
                    in_paragraph = true;
                    html.push_str("<p>") 
                }
            },
            Token::BlockQuote(_, _) | Token::Newline if quote_level > 0 => {},
            Token::CodeBlock(_, _) | Token::Newline | Token::Header(_, _, _) if in_paragraph => {
                in_paragraph = false;
                html.push_str("</p>")
            },
            Token::Plaintext(_) | Token::Italic(_) | Token::Bold(_) | Token::BoldItalic(_) | Token::Strikethrough(_) if !in_paragraph => {
                for _i in 0..quote_level {
                        html.push_str("</blockquote>");
                        quote_level-=1;
                }
                in_paragraph = true;
                html.push_str("<p>")
            },
            _ => {}
        }

        // Add content
        match token {
            Token::Plaintext(t) => {
                if t.trim().is_empty() {continue}
                
                // Handle references
                if t.contains("[^") && t.contains("]") {
                    let plaintext_tokens = t.split("[^");
                    let mut s = String::new();
                    let mut count = 1;
                    for tok in plaintext_tokens {
                        if tok.trim_end().ends_with("]") {
                            let tok = tok.trim_end().trim_end_matches(']');
                            s.push_str(format!(
                                "<sup id=\"fnref:{reference}\" role=\"doc-noteref\"><a href=\"#fn:{reference}\" class=\"footnote\" rel=\"footnote\">{ref_count}</a></sup>", 
                                reference = sanitize_display_text(tok), 
                                ref_count = count).as_str());
                            count+=1;
                        } else {s.push_str(tok)}
                    }
                    html.push_str(&s);
                } else {
                    html.push_str(sanitize_display_text(t.trim_start_matches('\n')).as_str())
                }
            },
            Token::Header(l, t, lbl) => {
                match lbl {
                    Some(lbl_text) => html.push_str(format!("<h{level} id=\"{id}\">{text}</h{level}>", 
                        level=l, 
                        text=t, 
                        id=sanitize_display_text(&lbl_text.replace(" ", "-")))
                        .as_str()),
                    None => html.push_str(format!("<h{level}>{text}</h{level}>", 
                        level=l, 
                        text=t)
                        .as_str()),
                };
            },
            Token::TaskListItem(c,t) => {
                if in_task_list == false {
                    in_task_list = true;
                    html.push_str("<ul class=\"contains-task-list\">")
                }
                match c {
                    TaskBox::Checked => {
                        html.push_str(format!("<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\" checked=\"\">{}</li>", sanitize_display_text(t)).as_str())

                    },
                    TaskBox::Unchecked => {
                        html.push_str(format!("<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\">{}</li>", sanitize_display_text(t)).as_str())
                    }
                }
            },
            Token::UnorderedListEntry(t) => {
                if in_unordered_list == false {
                    in_unordered_list = true;
                    html.push_str("<ul>")
                }
                html.push_str(format!("<li>{}</li>", sanitize_display_text(t)).as_str())
            },
            Token::OrderedListEntry(t) => {
                if in_ordered_list == false {
                    in_ordered_list = true;
                    html.push_str(format!("<ol>").as_str())
                }
                html.push_str(format!("<li>{}</li>", sanitize_display_text(t)).as_str())
            },
            Token::Newline => {html.push('\n')},
            Token::Tab => {html.push('\t')},
            Token::DoubleTab => {html.push_str("\t\t")},
            Token::Italic(t) => {html.push_str(format!("<em>{}</em>", sanitize_display_text(t)).as_str())},
            Token::Bold(t) => {html.push_str(format!("<strong>{}</strong>", sanitize_display_text(t)).as_str())},
            Token::BoldItalic(t) => {html.push_str(format!("<strong><em>{}</em></strong>", sanitize_display_text(t)).as_str())},
            Token::LineBreak => {html.push_str("<br>")},
            Token::HorizontalRule => {html.push_str("<hr />")},
            Token::Strikethrough(t) => {html.push_str(format!("<strike>{}</strike>", sanitize_display_text(t)).as_str())},
            Token::Code(t) => {
                html.push_str(format!("<pre><code>{}</code></pre>", sanitize_display_text(t)).as_str())},
            Token::CodeBlock(t, lang) => {
                html.push_str("<pre>");
                match lang.as_str() {
                    "" => html.push_str(format!("<code>{}</code>", sanitize_display_text(t)).as_str()),
                    _ => html.push_str(format!(
                        "<div class=\"language-{} highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>{}</code></div></div>",
                        sanitize_display_text(lang), 
                        sanitize_display_text(t)
                        ).as_str()),
                };
                html.push_str("</pre>");
            },
            Token::BlockQuote(l, t) => {
                if in_paragraph {
                    html.push_str("</p>");
                    in_paragraph = false;
                }
                match quote_level {
                    _ if l == &quote_level => {},
                    _ if l < &quote_level => {
                        let diff = quote_level - l;
                        quote_level = *l;
                        for _i in 0..diff {
                            html.push_str("</blockquote>");
                        }
                    },
                    _ if l > &quote_level => {
                        let diff = l - quote_level;
                        quote_level = *l;
                        for _i in 0..diff {
                            html.push_str("<blockquote>");
                        }
                    },
                    _ => {},
                }
                if !t.is_empty(){
                    html.push_str(format!("{}", sanitize_display_text(t)).as_str());
                }
            },
            Token::Image(l, t) => {
                let l = match validate_url(l){
                    Ok(vl) => vl,
                    _ => "",
                };
                match (l, t) {
                    (l, None) if l.trim() == "" => {html.push_str("<p><img src=\"data:,\"></p>")}
                    (l, Some(t)) if l.trim() == "" => {html.push_str(format!("<p><img src=\"data:,\" alt=\"{text}\"></p>", text=sanitize_display_text(t)).as_str())}
                    (l, None) => {html.push_str(format!("<p><img src=\"{link}\"> referrerpolicy=\"no-referrer\"></p>", link=l).as_str())}
                    (l, Some(t)) => {html.push_str(format!("<p><img src=\"{link}\" alt=\"{text}\" referrerpolicy=\"no-referrer\"></p>", link=l, text=sanitize_display_text(t)).as_str())}
                }
                
            },
            Token::Link(l, t, ht) => {
                let l = match validate_url(l){
                    Ok(vl) => vl,
                    _ => "",
                };
                match (t, ht){
                    (Some(t), Some(ht)) => html.push_str(format!("<a href=>\"{link}\" title=\"{hover}\" referrerpolicy=\"no-referrer\">{text}</a>", link=l, text=sanitize_display_text(t), hover=ht).as_str()),
                    (Some(t), None) => html.push_str(format!("<a href=\"{link}\" referrerpolicy=\"no-referrer\">{text}</a>", link=l, text=sanitize_display_text(t)).as_str()),
                    (None, Some(ht)) => html.push_str(format!("<a href=\"{link}\" title=\"{hover}\" referrerpolicy=\"no-referrer\">{link}</a>", link=l, hover=sanitize_display_text(ht)).as_str()),
                    (None, None) => html.push_str(format!("<a href=\"{link}\" referrerpolicy=\"no-referrer\">{link}</a>", link=l).as_str()),
                }
            },
            Token::Detail(summary, inner_tokens) => {
                if in_paragraph {
                    html.push_str("</p>\n");
                    in_paragraph = false;
                }
                let inner_html = parse(inner_tokens);
                html.push_str(format!("<details>\n<summary>{sum}</summary>\n{in_html}\n</details>", sum=sanitize_display_text(summary), in_html=inner_html).as_str());
            },
            Token::Table(headings, rows) => {
                if  headings.len() != rows[0].len() {continue}
                html.push_str("<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n");
                for h in headings.into_iter() {
                    html.push_str(format!("\t\t<th style=\"text-align: {align}\">{heading}</th>", heading=sanitize_display_text(&h.1), align=h.0).as_str());
                }
                html.push_str("\t</tr>\n\t</thead>\n\t<tbody>");
                for row in rows.iter(){
                    html.push_str("\n\t<tr>");
                    for elem in row.iter(){
                        let mut row_string = String::new();
                        for token in elem.1.iter() {
                           match token {
                            Token::Plaintext(s) => row_string.push_str(&sanitize_display_text(&s)),
                            Token::Italic(t) => {row_string.push_str(format!("<em>{}</em>", sanitize_display_text(t)).as_str())},
                            Token::Bold(t) => {row_string.push_str(format!("<strong>{}</strong>", sanitize_display_text(t)).as_str())},
                            Token::BoldItalic(t) => {row_string.push_str(format!("<strong><em>{}</em></strong>", sanitize_display_text(t)).as_str())},
                            Token::LineBreak => {row_string.push_str("<br>")},
                            Token::HorizontalRule => {row_string.push_str("<hr />")},
                            Token::Strikethrough(t) => {row_string.push_str(format!("<strike>{}</strike>", sanitize_display_text(t)).as_str())},
                            _ => row_string.push_str(&parse(&elem.1))
                            } 
                        }
                        html.push_str(format!("\n\t\t<td style=\"text-align: {align}\">{row_text}</td>", align=elem.0, row_text=row_string).as_str());
                    }
                    html.push_str("\n\t</tr>");
                }
                html.push_str("\n\t</tbody>\n</table>");
            },
            Token::Footnote(ref_id, text) => {
                references.push((ref_id, text));
            },
        }
    }

    // Close out any open tags
    if in_paragraph {
        html.push_str("</p>\n");
    }
    if in_task_list | in_unordered_list {
        html.push_str("</ul>");
    }
    if in_ordered_list {
        html.push_str("</ol>");
    }
    if quote_level > 0 {
        for _i in (0..quote_level).rev(){
            html.push_str("</blockquote>\n");
        }
    }


    // Add references
    if references.len() > 0{
        html.push_str("<div class=\"footnotes\" role=\"doc-endnotes\">\n");
        html.push_str("\t<ol>\n");
        for reference in references.iter(){
            html.push_str("\t\t<li id=\"fn:1\" role=\"doc-endnote\">");
            html.push_str(format!("\t\t\t<p>{ref_text}<a href=\"#fnref:{ref_count}\" class=\"reversefootnote\" role=\"doc-backlink\">↩</a></p>", 
                ref_count=sanitize_display_text(reference.0), 
                ref_text=sanitize_display_text(reference.1)).as_str());
            html.push_str("\t\t</li>");
        }
        html.push_str("\t</ol>\n");
        html.push_str("</div>\n");
    }
    if html.chars().last().unwrap() != '\n' {
        html.push('\n');
    }
    html
}

/// Render HTML from a source markdown string
/// Output is sanitized to prevent script injection
pub fn render(source: &str) -> String {
    parse(&lex(source))
}

pub fn render_ignore(source: &str, ignore: &[char]) -> String {
    parse(&lex(source))
}

/// Replace potentially unsafe characters with html entities
pub(crate) fn sanitize_display_text(source: &str) -> String {
    source.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('[', "&lbrack;")
        .replace(']', "&rbrack;")
        .replace('{', "&lbrace;")
        .replace('}', "&rbrace;")
        .replace('|', "&mid;")
        .replace('\\', "&backslash;")
        .replace('~', "&tilde;")
        .replace(')', "&#41;")
        .replace('(', "&#40;")
}

/// Basic url schema validation
pub(crate) fn validate_url(source: &str) -> Result<&str, SanitizationError> {
    if source.contains("\"") || !source.is_ascii() || source.contains(char::is_whitespace) { // https://www.rfc-editor.org/rfc/rfc3986#section-2
        return Err(SanitizationError{content: "Unsupported characters".to_string()})
    }
    let (schema, path) = source.split_at(source.find(':').unwrap_or(0));
    if schema.to_lowercase() == "javascript" || !schema.is_ascii() {
        return Err(SanitizationError{content: "Unsupported Schema".to_string()})
    }
    if schema.to_lowercase() == "data" && !path.starts_with(":image/"){
        return Err(SanitizationError{content: "Unsupported Data URL".to_string()})
    }
    Ok(source)
}