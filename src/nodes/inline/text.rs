use html_escape::{encode_quoted_attribute, encode_text};
use serde::{Deserialize, Serialize};

use crate::{
    marks::mark::{Mark, MarkVecToHtml as _}, 
    ToHtml
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub marks: Option<Vec<Mark>>,
}

impl Text {
    pub fn new(text: String, marks: Option<Vec<Mark>>) -> Self {
        Self { text, marks }
    }
}

impl ToHtml for Text {
    fn to_html(&self) -> String {
        let mut style = String::new();
        let mut tag = "span";
        let mut tag_a_attributes = String::new();
        let mut has_marks = false;

        if let Some(marks) = &self.marks {
            style = marks.get_styles();
            has_marks = !marks.is_empty();

            if let Some(Mark::Link(link)) = marks.iter().find(|m| matches!(m, Mark::Link(_))) {
                tag = "a";
                tag_a_attributes = link.html_a_tag_attributes();
            }
        }

        let escaped_text = encode_text(&self.text);

        if style.is_empty() && !has_marks {
            return escaped_text.into_owned();
        }

        let style_attr = if style.is_empty() {
            String::new()
        } else {
            format!(r#" style="{}""#, encode_quoted_attribute(&style))
        };

        let space = if tag_a_attributes.is_empty() { "" } else { " " };

        format!(r#"<{tag}{space}{tag_a_attributes}{style_attr}>{escaped_text}</{tag}>"#)
    }
}
