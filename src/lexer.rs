use crate::MiniIter;
use crate::ValidURL;
use crate::validate_link;

/// Tokens are the intermediate representation format in the markdown to html conversion
#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a>{
    /// String: Body of unstructured text
    Plaintext(String),
    /// u8: Header level (1..=6). str: Header text. Option<str>: html label
    Header(usize, String, Option<String>),
    /// str: Text for list entry
    UnorderedListEntry(Vec<&'a str>),
    /// str: Text for list entry
    OrderedListEntry(String),
    /// str: Text to be italicized
    Italic(String),
    /// str: Text to be bolded
    Bold(String),
    /// str: Text to be bolded and italicized
    BoldItalic(String),
    /// Corresponds to a </br> html tag
    LineBreak,
    /// Corresponds to a newline character
    Newline,
    /// Corresponds to a <hr /> html tag
    HorizontalRule,
    /// Used for control flow. Not directly rendered
    Tab,
    /// Used for control flow. Not directly rendered
    DoubleTab,
    /// str: Text to be struck through
    Strikethrough(String),
    /// str: Text to be placed within an inline code tag. eg. <code>str</code>
    Code(String),
    /// First str: Text to be placed within a multi-line code tag. Second str: Language
    CodeBlock(String, String),
    /// u8: Block quote level. str: Block quote text
    BlockQuote(u8, String),
    /// str: Link. Option<str>: Title for link.
    Image(String, Option<String>),
    /// str: Link. First Option<str>: Title for link. Second Option<str>: Hover text
    Link(ValidURL<'a>, Option<String>, Option<String>),
    /// str: Summary. Vec<Token>: Tokens to be rendered in the collapsable section
    Detail(String, Vec<Token<'a>>),
    /// Tuple of Vec<(Alignment, str)>: Which defines the table header and Vec<Vec<(Alignment, Vec<Token>)>> which defines the rows
    Table(Vec<(Alignment, String)>, Vec<Vec<(Alignment, Vec<Token<'a>>)>>),
    /// TaskBox: Boolean state of the checked or unchecked box. str: List item text
    TaskListItem(TaskBox, String),
    /// First str: Reference id. Second str: Reference text
    Footnote(String, String),
}

/// Holds the possible states of a taskbox in a task list
#[derive(Debug, PartialEq, Eq)]
pub enum TaskBox {
    Checked,
    Unchecked,
}

impl <'a> Token<'a>{
    fn is_usable_in_table(&self) -> bool {
        match self {
            Token::Code(_) => true,
            Token::Link(_, _, _) => true,
            Token::Bold(_) => true,
            Token::Italic(_) => true,
            Token::BoldItalic(_) => true,
            Token::Plaintext(_) => true,
            _ => false
        }
    }
}

/// Holds the alignment states for the table token
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center
}

use std::fmt;
impl fmt::Display for Alignment{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Alignment::Left => return write!(f, "left"),
            Alignment::Right => return write!(f, "right"),
            Alignment::Center => return write!(f, "center"),
        }
    }
}


#[derive(Debug)]
pub(crate) struct ParseError<'a>{
    pub(crate) content: &'a str,
}

pub(crate) fn push_str<'a>(t: &mut Vec<Token>, s: &'a str) {
    match t.last_mut() {
        Some(Token::Plaintext(token)) => {
            token.push_str(s)
        }
        _ => t.push(Token::Plaintext(s.to_string())),
    }
}

pub(crate) fn lex_heading<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let hashes = char_iter.consume_while_case_holds(&|c| c == "#").unwrap_or("");
    if hashes.len() > 6 {
        return Err(ParseError{content: hashes});
    }
    if char_iter.next_if_eq(&" ").is_none() && char_iter.next_if_eq(&"\t").is_none() && char_iter.peek() != Some(&"\n"){
        return Err(ParseError{content: hashes});
    }
    let line = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    let mut heading = "";
    if line.contains("{#") && 
        line.contains('}') {
            let (heading, _title) = line.split_once("{").unwrap_or(("",""));
            let line = line.strip_prefix(&heading).unwrap()
                .strip_prefix("{#").unwrap()
                .strip_suffix("}").unwrap();
        }
    let line_without_optional_trailing_hash_sequence = match line.trim_end().rsplit_once(' ') {
        Some((left, right)) => {
            match right.chars().all(|c| c == '#') {
                true => left,
                false => line,
            }
        },
        None => line,
    };
    if line.chars().all(|c| c == '#') {
        return Ok(Token::Header(hashes.len(), "".to_string(), None));
    }
    let parsed_line = crate::render_ignore(line_without_optional_trailing_hash_sequence.trim_end_matches(&[' ', '\t']), &['#'])
        .strip_prefix("<p>").unwrap_or("")
        .strip_suffix("</p>\n").unwrap_or("").trim().to_string();
    if heading != "" {
        return Ok(Token::Header(hashes.len(), heading.trim().to_string(), Some(parsed_line)));
    }
    return Ok(Token::Header(hashes.len(), parsed_line, None));
}

pub(crate) fn lex_asterisk_underscore<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let asterunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_" || c == "\t").unwrap_or("");
    if asterunds.len() == 1 && char_iter.next_if_eq(&" ").is_some(){
        let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
        char_iter.next();
        return Ok(Token::UnorderedListEntry(vec![s]))
    }
    if asterunds.chars().all(|x| x == '*') && char_iter.peek() == Some(&"\n"){
        return Ok(Token::HorizontalRule)
    }
    match asterunds.len() {
        1 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            if char_iter.peek() != Some("*") || char_iter.peek() != Some(&"_"){
                char_iter.next();
                return Ok(Token::Italic(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        2 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 2 {
                return Ok(Token::Bold(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        3 => {
            let s = char_iter.consume_while_case_holds(&|c| c != "*" && c != "_").unwrap_or("");
            let trailing_astunds = char_iter.consume_while_case_holds(&|c| c == "*" || c == "_").unwrap_or("");
            if trailing_astunds.len() == 3 {
                return Ok(Token::BoldItalic(s.to_string()))
            } else {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")});
            }
        },
        _ => {
            if asterunds.replace("\t", "").chars().all(|x| x == '*') || asterunds.chars().all(|x| x == '_'){
                return Ok(Token::HorizontalRule)
            } else {
                return Err(ParseError{content: asterunds})
            }
        }
    }
}

pub(crate) fn lex_tabs_spaces<'a>(char_iter: &mut MiniIter<'a>, tokens: &Vec<Token>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let whitespace = char_iter.consume_while_case_holds(&|c| c == "\t" || c == " ");
    match whitespace {
        None => return Err(ParseError{content: ""}),
        Some(s) if (1..=3).contains(&s.len()) && !s.contains("\t")  => return Err(ParseError{content: s}),
        Some(s) if s.len() >= 2 && 
                !s.contains("\t") && 
                char_iter.peek() == Some("\n")  => return Ok(Token::LineBreak),
        Some(_s) => {},
    }
    let whitespace = whitespace.unwrap_or("");
    let line = char_iter.consume_until_tail_is("\n").unwrap_or("");
    match whitespace {
        "    " if (matches!(tokens.last(), Some(Token::Plaintext(_))) && line.contains('#')) => return Err(ParseError{content: line}),
        "    " if (matches!(tokens.last(), Some(Token::Newline)) && line.contains('#')) => return Err(ParseError{content: line}),
        "\t" if  matches!(tokens.last(), Some(Token::Code(_))) => return Ok(Token::Code(line.to_string())),
        "\t" | "    " | "  \t" => return Ok(Token::Code(line.to_string())),
        "\t\t" => return Ok(Token::Code("\t".to_owned()+&line.to_string())),
        _ => {},
    }
    if char_iter.peek() == Some("\t") || char_iter.peek() ==  Some(" ") {
        match lex_tabs_spaces(char_iter, tokens) {
            Ok(Token::CodeBlock(_content, _lang)) => {
                return Ok(Token::CodeBlock(char_iter.get_substring_from(start_index).unwrap_or("").to_string(),"".to_string()))},
            Err(e) => return Err(e),
            Ok(_) => return Err(ParseError{content: ""}), 
        }
    }
    return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
}

pub(crate) fn lex_backticks<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let leading_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
    let mut lang = "";
    if leading_ticks.len() == 3 {
        if char_iter.next_if_eq("\n") != Some(&"\n") {
            lang = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
            char_iter.next();
        }
        let s = char_iter.consume_while_case_holds(&|c| c != "`").unwrap_or("");
        let trailing_ticks = char_iter.consume_while_case_holds(&|c| c == "`").unwrap_or("");
        if leading_ticks.len() != trailing_ticks.len() {
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
        } else {
            return Ok(Token::CodeBlock(s.to_string(), lang.to_string()))
        }
    }

    // let s = char_iter.consume_while_case_holds(&|c| c != "`" && c!= "\n").unwrap_or("");
    let tail = &(0..leading_ticks.len() as u64).map(|_| "`").collect::<String>();
    let s = char_iter.consume_until_tail_is(tail).unwrap_or("");
    if !s.ends_with(tail) {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}) 
    } else {
        let s = s.trim_end_matches(tail);
        if s.starts_with(' ') && s.ends_with(' ') {
            return Ok(Token::Code(s.trim_start_matches(' ').trim_end_matches(' ').to_string()))
        }
        return Ok(Token::Code(s.to_string()))
    }

    // leading_ticks.len() == 3. Check for lang

}

pub(crate) fn lex_newlines<'a>(char_iter: &mut MiniIter<'a>, tokens: &Vec<Token>) -> Result<Token<'a>, ParseError<'a>> {
    match char_iter.consume_while_case_holds(&|c| c == "\n") {
        Some(s) if s.len() >= 2 => return Ok(Token::Newline),
        Some(s) if s.len() < 2 => return Err(ParseError{content: s}),
        _ => return Err(ParseError{content: ""}),
    }
}

pub(crate) fn lex_blockquotes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let right_arrows = char_iter.consume_while_case_holds(&|c| c == ">").unwrap_or("");
    match char_iter.peek() {
        Some(" ") | Some("\t") => {},
        _ => {return Err(ParseError{content: right_arrows})}
    }
    let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
    char_iter.next_if_eq(&"\n");
    Ok(Token::BlockQuote(right_arrows.len() as u8, s.to_string()))
}

pub(crate) fn lex_images<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.next_if_eq("!") != Some(&"!"){
        return Err(ParseError{content: ""})
    }
    let link_result = lex_links(char_iter);
    match link_result {
        Err(_e) => return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")}),
        Ok(Token::Link(link, title, _)) => return Ok(Token::Image(link.content.to_string(), title)),
        _ => return Err(ParseError{content: "Non link token returned from lex_links"})
    }
}

pub(crate) fn lex_links<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.next_if_eq("[") != Some(&"[") {
        return Err(ParseError{content: ""})
    }
    let title = char_iter.consume_while_case_holds(&|c| c != "]").unwrap_or("");
    if char_iter.next_if_eq("]") != Some(&"]") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    // Parse footnotes big and small
    if title.starts_with("^") && char_iter.next_if_eq(":") == Some(&":") {
        let ref_id = title.strip_prefix("^").unwrap_or("");
        let note_index = char_iter.get_index();
        loop {
           char_iter.consume_while_case_holds(&|c| c != "\n");
           char_iter.next();
           if char_iter.peek() != Some(&" ") && char_iter.peek() != Some(&"\t") {
            break;
           }
           if char_iter.next_if_eq("\t") == Some(&"\t") {
            continue;
           }
           if char_iter.peek() == Some(&" ") {
            let spaces = char_iter.consume_while_case_holds(&|c| c == " ").unwrap_or("");
            match spaces.len() {
                2 | 4 => {},
                _ => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})},
            }
            continue
           }
           break;
        }
        if ref_id.contains(char::is_whitespace){
            return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
        }
        return Ok(Token::Footnote(ref_id.to_string(), char_iter.get_substring_from(note_index).unwrap_or("").trim().to_string()));
    }
    if char_iter.next_if_eq("(") != Some(&"(") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    let link = char_iter.consume_while_case_holds(&|c| c != ")" && c != " ").unwrap_or("");
    if char_iter.peek() != Some(&")") && char_iter.peek() != Some(&" ") {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    if char_iter.next_if_eq(")") == Some(&")") {
        match validate_link(link) {
            Ok(vl) => return Ok(Token::Link(vl, Some(title.to_string()), None)),
            Err(se) => return Err(ParseError{content: &se.content}),
        }
        
    }
    if char_iter.peek() == Some(&" ") {
        let hover = char_iter.consume_while_case_holds(&|c| c != ")").unwrap_or("");
        char_iter.skip_while(|c| c != &"\n").next();
        match validate_link(link) {
           Ok(vl) => return Ok(Token::Link(vl, Some(title.to_string()), Some(hover.to_string()))),
           Err(se) => return Err(ParseError{content: &se.content}),
        }
    }
    Err(ParseError{content: ""})
}

pub(crate) fn lex_side_carrot<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    if char_iter.next_if_eq("<") !=  Some("<") {return Err(ParseError{content: ""})}

    let s = char_iter.consume_while_case_holds(&|c| c != ">").unwrap_or("");
    match (s, char_iter.next_if_eq(">")) {
        ("details", Some(">")) => {
            char_iter.next_if_eq(&"\r");
            if !char_iter.next_if_eq(&"\n").is_some(){
                return Err(ParseError{content: s});
            }
            return parse_details(char_iter)
        },
        (_, Some(">")) if s.len() >= 1 => {
            if s.contains(char::is_whitespace) {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})}
            match validate_link(s) {
                Ok(vl) if vl.scheme.is_some() => {return Ok(Token::Link(vl, None, None))} // Auto links require a scheme
                _ => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})}
            }
            
        },
        (_, Some(">")) if s.len() == 0 => {
            return Err(ParseError{content: "<>"})
        },
        (_, _) => return Err(ParseError{content: s}),
    }
}

pub(crate) fn lex_plus_minus<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let s = char_iter.consume_while_case_holds(&|c| c == "-" || c == "+").unwrap_or("");
    match s.len() {
        3..=usize::MAX => { return Ok(Token::HorizontalRule)},
        2 => {return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})},
        1 => {},
        _ => {return Err(ParseError{content: "string length error"})},
    }
    let line_index = char_iter.get_index();
    let mut lines = vec![char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("")];
    while char_iter.get_substring_ahead(3) == Some("\n\n\t") {
        char_iter.next();
        char_iter.next();
        char_iter.next();
        char_iter.next();
        lines.push(char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or(""));
    }
    let line = char_iter.get_substring_from(line_index).unwrap_or("");
    if line.starts_with(" [ ] "){return Ok(Token::TaskListItem(TaskBox::Unchecked, line[5..].to_string()))}
    else if line.starts_with(" [x] ") || line.starts_with(" [X] "){return Ok(Token::TaskListItem(TaskBox::Checked, line[5..].to_string()))}
    else if line.starts_with(" "){return Ok(Token::UnorderedListEntry(lines))}
    else {return Ok(Token::UnorderedListEntry(lines))}
}

pub(crate) fn lex_numbers<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let c = char_iter.next().unwrap();
    match char_iter.next_if_eq(".") {
        Some(".") => {
            if char_iter.next_if_eq(" ") != Some(&" "){
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
            }
            let s = char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or("");
            return Ok(Token::OrderedListEntry(s.to_string()))
        },
        _ => return Err(ParseError{content: c})
    }
}

pub(crate) fn lex_tilde<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let lead_tildes = match char_iter.consume_while_case_holds(&|s| s == "~") {
        Some(s) => s,
        None => return Err(ParseError{content: "Failure to parse ~"}),
    };
    match lead_tildes.len() {
        1 => return Err(ParseError{content: lead_tildes}),
        2 => {
            let line = char_iter.consume_while_case_holds(&|s| s != "~").unwrap_or("");
            let tail_tildes = char_iter.consume_while_case_holds(&|s| s == "~").unwrap_or("");
            if lead_tildes.len() != tail_tildes.len() {
                return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
            }
            return Ok(Token::Strikethrough(line.to_string()));
        }
        _ => return Err(ParseError{content: lead_tildes}),
    }
}

fn parse_details<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>>{
    let mut summary_line = char_iter.consume_while_case_holds(&|c| c != "\n" && c != "\r").unwrap_or("");
    char_iter.next_if_eq("\r");
    if (!summary_line.starts_with("<summary>") || !summary_line.ends_with("</summary>")) && !summary_line.len() >= 20 {
        return Err(ParseError{content: summary_line});
    }
    summary_line = summary_line.strip_prefix("<summary>").unwrap()
                    .strip_suffix("</summary>").unwrap_or("");
    let remaining_text_index = char_iter.get_index();
    let mut remaining_text = char_iter.consume_until_tail_is("</details>").unwrap_or("");
    if remaining_text.contains("<details>") {
        let mut opens = remaining_text.matches("<details>").count();
        let mut closes = remaining_text.matches("</details>").count();
        while opens == closes {
            char_iter.consume_until_tail_is("</details>").unwrap_or("");
            remaining_text = char_iter.get_substring_from(remaining_text_index).unwrap_or("");
            opens = remaining_text.matches("<details>").count();
            closes = remaining_text.matches("</details>").count();
        }
    }
    let inner_tokens = crate::lex(remaining_text.strip_suffix("</details>").unwrap_or(""), &[]);
    Ok(Token::Detail(summary_line.to_string(), inner_tokens))
}

pub(crate) fn lex_pipes<'a>(char_iter: &mut MiniIter<'a>) -> Result<Token<'a>, ParseError<'a>> {
    let start_index = char_iter.get_index();
    let mut lines = Vec::new();
    while char_iter.next_if_eq(&"|") == Some("|") {
        lines.push(char_iter.consume_while_case_holds(&|c| c != "\n").unwrap_or(""));
        char_iter.next();
    }
    if lines.len() < 3 {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    if !lines.iter().all(|l| l.matches("|").count() == lines[0].matches("|").count()) {
        return Err(ParseError{content: char_iter.get_substring_from(start_index).unwrap_or("")})
    }
    let headings: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim().to_string())
        .collect();
    let alignments: Vec<_> = lines.remove(0).split("|")
        .filter(|&x| x != "")
        .map(|x| 
            {
            match (x.trim().starts_with(":"), x.trim().ends_with(":")) {
                (true, false) => Alignment::Left,
                (true, true) => Alignment::Center,
                (false, true) => Alignment::Right,
                _ => Alignment::Left,
            }}
        )
        .collect();
    let mut rows = Vec::new();
    for l in lines.into_iter() {
        let elements: Vec<&str> = l.split("|")
        .filter(|&x| x != "")
        .map(|x| x.trim())
        .collect();
        let mut r = Vec::new();
        for e in elements.into_iter() {
            let mut inner_tokens = crate::lex(&e, &[]);
            inner_tokens.retain(|token| token.is_usable_in_table());
            r.push(inner_tokens);
        }
        rows.push(alignments.clone().into_iter().zip(r.into_iter()).collect());
    }
    Ok(Token::Table(alignments.into_iter().zip(headings.into_iter()).collect(), rows))
}