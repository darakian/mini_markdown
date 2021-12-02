# mini_markdown
A dependency free markdown renderer.
___
The design goal of this project is to provide a dependency free, feature complete markdown to html renderer.
All output is sanitized and any script injection vector is considered a bug.

## Status
Markdown support is now feature complete with testing ongoing.   
Please report bugs as issues should you find any.  
All output is considered unstable for the time being.


## Usage
The primary function in this library is the `render` function.
```
pub fn render(source: &str) -> String {
    parse(lex(source))
}
```
If all you want is to take markdown and get html `render` is your function. If you want to work with the internal structure the `parse` and `lex` functions are public.

## Syntax
### Headings
Headings are supported with the `#` syntax only. Up to six levels are supported
### Paragraphs
One or more lines separating text will begin a new paragraph
### Line breaks
Two or more spaces at the end of a line will add a `<br>`
### Italic text
Text surrounded by any pairing of `*` and `_` will be italicized
```
*text*, _text_, *text_ or _text*
```
### Bold text
Text surrounded by any pairing of two `*` and/or `_` will be bolded
### Bold Italic text
Text surrounded by any pairing of three `*` and/or `_` will be bolded and italicized
### Blockquotes and nested block quotes
Blockquotes and nested block quotes are supported using the `>` character
```
> Some quoted text
>> Some quoted text in a nested quote
> Some more text in the outer quote
```
### Ordered Lists
Ordered lists are supported by numbering new lines
```
1. One
2. Two
3. Three
```
Max length is 9 elements
### Unordered lists 
Unordered lists are supported by starting a line with `-`, `+`, or `*`
```
* Some
- list
+ text
```
### Inline code
Inline code is supported by encapsulating text in two \` characters
### Code blocks
Code blocks are supported by either indenting lines with four spaces or by encapsulating lines with three \` characters before and after.
### Images
Images are supported with the following syntax `![Hover text](link)`
### Links
Links are supported with the following syntax `[Text](link)`
### Quick links
Url or email text can be made into a link by enclosing the text in angle brackets `<`text`>`
### Horizontal rules
Horizontal rules are supported with three or more `*`, `-`, or `_` characters alone on a line
```
****
-----
or
______
```
### Escaping characters
Control characters can be escaped by preceding them with a backslash `\`

### Collapsable sections
Collapsable sections are supported and follow the html for a `details` section.
```
<details>
<summary>Summary text</summary>
Text
</details>
```
Will render as 
<details>
<summary>Summary text</summary>
Text
</details>

### Tables
HTML tables are supported with the following
```
| Syntax      | Description | Test Text     |
| :---        |    :----:   |          ---: |
| Header      | Title       | Here's this   |
| Paragraph   | Text        | And more      |
```

which renders as

| Syntax      | Description | Test Text     |
| :---        |    :----:   |          ---: |
| Header      | Title       | Here's this   |
| Paragraph   | Text        | And more      |

The `:` character will define the alignments as shown.

### Footnotes
References with text tags are supported for both inline footnotes and multiline big footnotes.  
For example
```
Here's a simple footnote,[^1] and here's a longer one.[^bignote]
[^1]: This is the first footnote.

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.

```
renders as  

Here's a simple footnote,[^1] and here's a longer one.[^bignote]
[^1]: This is the first footnote.

[^bignote]: Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.

    `{ my code }`

    Add as many paragraphs as you like.
