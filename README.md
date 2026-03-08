# adf2html

A Rust library for converting [Atlassian Document Format (ADF)](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure/) documents into HTML. ADF is the rich-text format used by Atlassian products (Jira, Confluence) and is returned by the Atlassian v3 REST API.

## Features

Supports the following ADF node types:

**Block nodes:** `blockquote`, `bulletList`, `codeBlock`, `expand`, `heading`, `mediaGroup`, `mediaSingle`, `orderedList`, `panel`, `paragraph`, `rule`, `table`

**Inline nodes:** `date`, `emoji`, `hardBreak`, `inlineCard`, `mention`, `status`, `text`

**Marks:** `code`, `em`, `link`, `strike`, `strong`, `subsup`, `textColor`, `underline`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
adf2html = "1.0"
```

## Usage

### Basic conversion

Deserialize an ADF JSON payload into a `Document` and call `to_html`:

```rust
use adf2html::document::Document;

let adf_json = r#"{
    "version": 1,
    "type": "doc",
    "content": [
        {
            "type": "paragraph",
            "content": [
                {
                    "type": "text",
                    "text": "Hello, world!",
                    "marks": [{ "type": "strong" }]
                }
            ]
        }
    ]
}"#;

let document: Document = serde_json::from_str(adf_json).unwrap();

// Pass an optional timezone for rendering date nodes, and a base URL used
// for rendering inline mention/card links.
let html = document.to_html(None, "https://your-jira-instance.atlassian.net");

println!("{}", html);
// <p><strong>Hello, world!</strong></p>
```

### Specifying a timezone for date nodes

ADF `date` nodes store a UTC timestamp. Pass a [`chrono_tz::Tz`](https://docs.rs/chrono-tz) value to render dates in a specific timezone:

```rust
use adf2html::document::Document;
use adf2html::America::New_York;

let document: Document = serde_json::from_str(adf_json).unwrap();
let html = document.to_html(Some(New_York), "https://your-jira-instance.atlassian.net");
```

### Replacing media URLs

When an ADF document contains media attachments, the Atlassian API returns relative `/rest/api/3/attachment/content/<id>` paths. After fetching the rendered HTML you can rewrite those paths to absolute URLs pointing to your instance:

```rust
use adf2html::document::Document;

let mut document: Document = serde_json::from_str(adf_json).unwrap();
let html = document.to_html(None, "https://your-jira-instance.atlassian.net");

// Rewrites all attachment paths found in `html` to absolute URLs.
document.replace_media_urls("https://your-jira-instance.atlassian.net", &html);

// Render again with the updated URLs.
let html_with_media = document.to_html(None, "https://your-jira-instance.atlassian.net");
```

## License

MIT