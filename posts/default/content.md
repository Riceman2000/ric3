You have landed on the default blog page, not accessable by anything other than snooping around. So congrats for doing that. 

The following is a test of my markdown rendering.

# h1 Heading 
## h2 Heading
### h3 Heading
#### h4 Heading
##### h5 Heading
###### h6 Heading


## Horizontal Rules

___

---

***


## Typographic replacements

(c) (C) (r) (R) (tm) (TM) (p) (P) +-

test.. test... test..... test?..... test!....

!!!!!! ???? ,,  -- ---

"Smartypants, double quotes" and 'single quotes'

## Emphasis

**This is bold text**

__This is bold text__

*This is italic text*

_This is italic text_

~Strikethrough~~

## Blockquotes

> Nesting?
>> With more arrows?
> > > Mayhaps with spaces?

## Lists

Unordered

+ Mixing symbols +, -, or *
+ 2 space indented sublist
  - Marker character change forces new list start:
    * Ac tristique libero volutpat at
    + Facilisis in pretium nisl aliquet
    - Nulla volutpat aliquam velit
+ Back to the first level

Ordered

1. Lorem ipsum dolor sit amet
2. Consectetur adipiscing elit
3. Integer molestie lorem at massa


1. Keeping all numbers ... 
1. ... the same

Start numbering with offset:

57. foo
1. bar


## Code

Inline `code` with text after

Indented code

    // Some comments
    line 1 of code
    line 2 of code
    line 3 of code


Block code "fences"

```
Sample text here...
```

Syntax highlighting

```rust
fn test_func(var: String) -> u32 {
    // do something 
    10
}
```

## Tables

| Option | Description |
| ------ | ----------- |
| data   | path to data files to supply the data that will be passed into templates. |
| engine | engine to be used for processing templates. Handlebars is the default. |
| ext    | extension to be used for dest files. |

Right aligned columns

| Option | Description |
| ------:| -----------:|
| data   | path to data files to supply the data that will be passed into templates. |
| engine | engine to be used for processing templates. Handlebars is the default. |
| ext    | extension to be used for dest files. |


## Links

[link text](https://github.com/Riceman2000)

[link with title](https://github.com/Riceman2000 "title text!")

Autoconverted link https://github.com/Riceman2000


## Images

![Minion](https://octodex.github.com/images/original.png)
![Stormtroopocat](https://octodex.github.com/images/original.png "Some text here")

Footnote style syntax

![Alt text][id]

With a reference later in the document defining the URL location:

[id]: https://octodex.github.com/images/original.png  "Some more text"

### Emojis

Classic markup: :wink: :cry: :laughing: :yum:

Shortcuts (emoticons): :-) :-( 8-) ;)

### Subscript / Superscript

- 19^th^
- H~2~O

### Footnotes

Footnote 1 link[^first].

Footnote 2 link[^second].

Inline footnote^[Text of inline footnote] definition.

Duplicated footnote reference[^second].

[^first]: Footnote **can have markup**

    and multiple paragraphs.

[^second]: Footnote text.


### Definition lists

Term 1

:   Definition 1
with lazy continuation.

Term 2 with *inline markup*

:   Definition 2

        { some code, part of Definition 2 }

    Third paragraph of definition 2.

_Compact style:_

Term 1
  ~ Definition 1

Term 2
  ~ Definition 2a
  ~ Definition 2b


### Abbreviations

This is HTML abbreviation example.

It converts "HTML", but keep intact partial entries like "xxxHTMLyyy" and so on.

*[HTML]: Hyper Text Markup Language
