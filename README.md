# mini_markdown
A minimal, usable markdown renderer.
___
The design goal of this project is to provide a dependency free markdown to html renderer.

## Status
The basic markdown syntax is currently supported. Extended syntax is ongoing.   
Testing is also ongoing. Please report bugs as issues should you find any.  
Any output is considered unstable for the time being.

## Currently Supported Syntax
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