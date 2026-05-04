use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, LinkType, Tag};

use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Codeblock kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnedCodeBlockKind {
    Indented,
    /// The value contained in the tag describes the language of the code, which may be empty.
    Fenced(String),
}

impl OwnedCodeBlockKind {
    pub fn is_indented(&self) -> bool {
        matches!(*self, Self::Indented)
    }

    pub fn is_fenced(&self) -> bool {
        matches!(*self, Self::Fenced(_))
    }
}

impl From<CodeBlockKind<'_>> for OwnedCodeBlockKind {
    fn from(kind: CodeBlockKind<'_>) -> Self {
        match kind {
            CodeBlockKind::Indented => Self::Indented,
            CodeBlockKind::Fenced(text) => Self::Fenced(text.into_string()),
        }
    }
}

/// ``OwnedTags`` for elements that can contain other elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OwnedTag {
    /// A paragraph of text and other inline elements.
    Paragraph,

    /// A heading. The first field indicates the level of the heading,
    /// the second the fragment identifier, and the third the classes.
    Heading(HeadingLevel, Option<String>, Vec<String>),

    BlockQuote,
    /// A code block.
    CodeBlock(OwnedCodeBlockKind),

    /// A list. If the list is ordered the field indicates the number of the first item.
    /// Contains only list items.
    List(Option<u64>), // TODO: add delim and tight for ast (not needed for html)
    /// A list item.
    Item,
    /// A footnote definition. The value contained is the footnote's label by which it can
    /// be referred to.
    FootnoteDefinition(String),

    /// A table. Contains a vector describing the text-alignment for each of its columns.
    Table(Vec<Alignment>),
    /// A table header. Contains only `TableCell`s. Note that the table body starts immediately
    /// after the closure of the `TableHead` tag. There is no `TableBody` tag.
    TableHead,
    /// A table row. Is used both for header rows as body rows. Contains only `TableCell`s.
    TableRow,
    TableCell,

    // span-level tags
    Emphasis,
    Strong,
    Strikethrough,

    /// A link. The first field is the link type, the second the destination URL and the third is a title.
    Link(LinkType, String, String),

    /// An image. The first field is the link type, the second the destination URL and the third is a title.
    Image(LinkType, String, String),
}

impl From<Tag<'_>> for OwnedTag {
    fn from(kind: Tag<'_>) -> Self {
        match kind {
            Tag::Paragraph => Self::Paragraph,
            Tag::Heading(level, id, classes) => Self::Heading(
                level,
                id.map(|txt| txt.to_owned()),
                classes.into_iter().map(|txt| txt.to_owned()).collect(),
            ),
            Tag::BlockQuote => Self::BlockQuote,
            Tag::CodeBlock(kind) => Self::CodeBlock(kind.into()),
            Tag::List(content) => Self::List(content),
            Tag::Item => Self::Item,
            Tag::FootnoteDefinition(txt) => Self::FootnoteDefinition(txt.into_string()),
            Tag::Table(aligns) => Self::Table(aligns),
            Tag::TableHead => Self::TableHead,
            Tag::TableRow => Self::TableRow,
            Tag::TableCell => Self::TableCell,
            Tag::Emphasis => Self::Emphasis,
            Tag::Strong => Self::Strong,
            Tag::Strikethrough => Self::Strikethrough,
            Tag::Link(kind, url, title) => Self::Link(kind, url.into_string(), title.into_string()),
            Tag::Image(kind, url, title) => {
                Self::Image(kind, url.into_string(), title.into_string())
            }
        }
    }
}

/// Markdown events that are generated in a preorder traversal of the document
/// tree, with additional `End` events whenever all of an inner node's children
/// have been visited.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OwnedEvent {
    /// Start of a tagged element. Events that are yielded after this event
    /// and before its corresponding `End` event are inside this element.
    /// Start and end events are guaranteed to be balanced.
    Start(OwnedTag),
    /// End of a tagged element.
    End(OwnedTag),
    /// A text node.
    Text(String),
    /// An inline code node.
    Code(String),
    /// An HTML node.
    Html(String),
    /// A reference to a footnote with given label, which may or may not be defined
    /// by an event with a `OwnedTag::FootnoteDefinition` tag. Definitions and references to them may
    /// occur in any order.
    FootnoteReference(String),
    /// A soft line break.
    SoftBreak,
    /// A hard line break.
    HardBreak,
    /// A horizontal ruler.
    Rule,
    /// A task list marker, rendered as a checkbox in HTML. Contains a true when it is checked.
    TaskListMarker(bool),
}

impl From<Event<'_>> for OwnedEvent {
    fn from(e: Event<'_>) -> Self {
        match e {
            Event::Start(tag) => Self::Start(tag.into()),
            Event::End(tag) => Self::End(tag.into()),
            Event::Text(txt) => Self::Text(txt.into_string()),
            Event::Code(txt) => Self::Code(txt.into_string()),
            Event::Html(txt) => Self::Html(txt.into_string()),
            Event::FootnoteReference(txt) => Self::FootnoteReference(txt.into_string()),
            Event::SoftBreak => Self::SoftBreak,
            Event::HardBreak => Self::HardBreak,
            Event::Rule => Self::Rule,
            Event::TaskListMarker(marker) => Self::TaskListMarker(marker),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpannedEvent {
    pub span: Range<usize>,
    pub event: OwnedEvent,
}

