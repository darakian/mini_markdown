pub mod lexer;
use crate::lexer::*;

pub fn lex(source: &str) -> Vec<Token>{
    let mut char_iter = source.chars().peekable();
    let mut tokens = Vec::new();
    while char_iter.peek().is_some(){
        match char_iter.peek().unwrap(){
            '#' => {
                let token = lex_heading(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '*' | '_' => {
                let token = lex_asterisk_underscore(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '~' => {
                let token = lex_tilde(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '-' | '+' => {
                let token = lex_plus_minus(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            ' ' => {
                let token = lex_spaces(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '`' => {
                let token = lex_backticks(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '\n' => {
                let token = lex_newlines(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '>' if (tokens.last() == Some(&Token::Newline) || tokens.len() == 0) => {
                let token = lex_blockquotes(&mut char_iter);
                match token {
                    Ok(t) => {
                        tokens.push(t);
                        },
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '!' => {
                let token = lex_images(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '[' => {
                let token = lex_links(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '<' => {
                let token = lex_side_carrot(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '|' => {
                let token = lex_pipes(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                let token = lex_numbers(&mut char_iter);
                match token {
                    Ok(t) => tokens.push(t),
                    Err(e) => push_str(&mut tokens, e.content),
                }
            }
            // Parse '\' to escape a markdown control character
            '\\' => {
                char_iter.next();
                if char_iter.peek().is_some(){
                    let c = char_iter.next().unwrap();
                    push_char(&mut tokens, c);
                }
            }
            _ => {
                let c = char_iter.next().unwrap();
                push_char(&mut tokens, c);
            },
        }
    }
    tokens
}

pub fn parse(tokens: &Vec<Token>) -> String {
    let mut html = String::new();
    let mut in_ordered_list = false;
    let mut in_unordered_list = false;
    let mut in_paragraph = false;
    let mut quote_level = 0;
    for token in tokens.iter(){
        // Close multi-liners
        if in_ordered_list {
            match token {
                Token::OrderedListEntry(_) | Token::UnorderedListEntry(_) => {},
                Token::Tab | Token::DoubleTab => {},
                _ => {
                    in_ordered_list = false;
                    html.push_str(format!("</ol>").as_str())
                }
            }
        }
        if in_unordered_list {
            match token {
                Token::OrderedListEntry(_) | Token::UnorderedListEntry(_) => {},
                Token::Tab | Token::DoubleTab => {},
                _ => {
                    in_unordered_list = false;
                    html.push_str(format!("</ul>").as_str())
                }
            }
        }
        if quote_level > 0 {
            match token {
                Token::BlockQuote(_l, _s) => {},
                Token::Newline => {},
                _ => {
                    for _i in 0..quote_level {
                        html.push_str(format!("</blockquote>").as_str());
                        quote_level-=1;
                    }
                }
            }
        }

        // Add content
        match token {
            Token::Plaintext(t) => {
                if !in_paragraph {
                    html.push_str(format!("<p>").as_str());
                    in_paragraph = true;
                }
                html.push_str(format!("{}", sanitize(t)).as_str())
            },
            Token::Header(l, t, lbl) => {
                let mut id;
                match lbl {
                    Some(_t) => id = lbl.as_ref().unwrap().to_string(),
                    None => id = t.to_string(),
                };
                id.make_ascii_lowercase();
                html.push_str(format!("<h{level} id=\"{id}\">{text}</h{level}>\n", 
                    level=l, 
                    text=sanitize(t), 
                    id=id.replace(" ", "-"))
                .as_str())
            },
            Token::UnorderedListEntry(t) => {
                if in_unordered_list == false {
                    in_unordered_list = true;
                    html.push_str(format!("<ul>").as_str())
                }
                html.push_str(format!("<li>{}</li>", sanitize(t)).as_str())
            },
            Token::OrderedListEntry(t) => {
                if in_ordered_list == false {
                    in_ordered_list = true;
                    html.push_str(format!("<ol>").as_str())
                }
                html.push_str(format!("<li>{}</li>", sanitize(t)).as_str())
            },
            Token::Newline => {html.push('\n')},
            Token::ParagraphBreak => {
                if in_paragraph {
                    html.push_str(format!("</p>").as_str());
                    in_paragraph = false;
                }
            },
            Token::Italic(t) => {html.push_str(format!("<em>{}</em>", sanitize(t)).as_str())},
            Token::Bold(t) => {html.push_str(format!("<strong>{}</strong>", sanitize(t)).as_str())},
            Token::BoldItalic(t) => {html.push_str(format!("<strong><em>{}</em></strong>", sanitize(t)).as_str())},
            Token::LineBreak => {html.push_str("<br>")},
            Token::HorizontalRule => {html.push_str("<hr />")},
            Token::Strikethrough(t) => {html.push_str(format!("<strike>{}</strike>", sanitize(t)).as_str())},
            // Token::Tab => {},
            // Token::DoubleTab => {},
            Token::Code(t) => {html.push_str(format!("<code>{}</code>", sanitize(t)).as_str())},
            Token::CodeBlock(t, lang) => {
                html.push_str(format!(
                "<div class=\"language-{} highlighter-rouge\"><div class=\"highlight\"><pre class=\"highlight\"><code>{}</code></pre></div></div>",
                sanitize(lang), 
                sanitize(t)
                ).as_str())
            },
            Token::BlockQuote(l, t) => {
                if in_paragraph {
                    html.push_str(format!("</p>").as_str());
                    in_paragraph = false;
                }
                match quote_level {
                    _ if l == &quote_level => {},
                    _ if l < &quote_level => {
                        let diff = quote_level - l;
                        quote_level = *l;
                        for _i in 0..diff {
                            html.push_str(format!("</blockquote>").as_str());
                        }
                    },
                    _ if l > &quote_level => {
                        let diff = l - quote_level;
                        quote_level = *l;
                        for _i in 0..diff {
                            html.push_str(format!("<blockquote>").as_str());
                        }
                    },
                    _ => {},
                }
                if !t.is_empty(){
                    html.push_str(format!("{}", sanitize(t)).as_str());
                }
            },
            Token::Image(l, t) => html.push_str(format!("<img src=\"{link}\" alt=\"{text}\"", link=l, text=sanitize(t)).as_str()),
            Token::Link(l, t, ht) => {
                match (t, ht){
                    (Some(t), Some(ht)) => html.push_str(format!("<a href=>\"{link}\" title=\"{hover}\">{text}", link=l, text=sanitize(t), hover=ht).as_str()),
                    (Some(t), None) => html.push_str(format!("<a href=\"{link}\">{text}</a>", link=l, text=sanitize(t)).as_str()),
                    (None, Some(ht)) => html.push_str(format!("<a href=\"{link}\" title=\"{hover}\">{link}</a>", link=l, hover=sanitize(ht)).as_str()),
                    (None, None) => html.push_str(format!("<a href=\"{link}\">{link}</a>", link=l).as_str()),
                }
            },
            Token::Detail(summary, inner_tokens) => {
                let inner_html = parse(inner_tokens);
                html.push_str(format!("<details>\n<summary>{sum}</summary>\n{in_html}\n</details>", sum=sanitize(summary), in_html=inner_html).as_str());
            },
            Token::Table(headings, rows) => {
                //Assert headings.len() == rows.width()
                html.push_str("<table class=\"table table-bordered\">\n\t<thead>\n\t<tr>\n");
                for h in headings.into_iter() {
                    html.push_str(format!("\t\t<th style=\"text-align: {align}\">{heading}</th>", heading=h.1, align=h.0).as_str());
                }
                html.push_str("\t</tr>\n\t</thead>\n\t<tbody>");
                for row in rows.iter(){
                    html.push_str("\n\t<tr>");
                    for elem in row.iter(){
                        html.push_str(format!("\n\t\t<td style=\"text-align: {align}\">{row_text}</td>", align=elem.0, row_text=elem.1).as_str());
                    }
                    html.push_str("\n\t</tr>");
                }
                html.push_str("\n\t</tbody>\n</table>");
            },
            _ => {},
        }
    }
    if in_paragraph {
        html.push_str(format!("</p>").as_str());
        // in_paragraph = true;
    }
    if quote_level > 0 {
        for _i in (0..quote_level).rev(){
            html.push_str(format!("</blockquote>").as_str());
        }
    }
    html
}

pub fn render(source: &str) -> String {
    parse(&lex(source))
}

pub fn sanitize(source: &String) -> String {
    source.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}