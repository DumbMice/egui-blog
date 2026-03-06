---
title: "Blockquote Alignment Test"
date: "2026-03-06"
tags: ["test", "blockquote", "alignment"]
---

## Blockquote Alignment Test

This post tests the fixed blockquote rendering with GitHub-style borders.

### Single Line Blockquote
> This is a single line blockquote.

### Multi-Line Blockquote
> This is a multi-line blockquote that spans multiple lines to test if the vertical border scales correctly with the text height. The border should extend the full height of the blockquote content.

### Blockquote with Math
> This blockquote contains math: $E = mc^2$ and another formula $x^2 + y^2 = z^2$.

### Nested Elements in Blockquote
> This blockquote has **bold text**, *italic text*, and `inline code`.
> 
> It also has a second paragraph within the same blockquote.

### Very Long Blockquote
> Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
> 
> Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.

### Mixed Content
Normal text before blockquote.

> Blockquote with multiple elements:
> 1. First item
> 2. Second item
> 3. Third item
> 
> And a code block:
> ```rust
> fn main() {
>     println!("Hello, world!");
> }
> ```

Normal text after blockquote.

## Expected Behavior
1. Vertical border should align perfectly with text (not higher or lower)
2. Border should extend full height of blockquote content
3. Multi-line blockquotes should have continuous border
4. Border color should match weak text color
5. Proper padding around text