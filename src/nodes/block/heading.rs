use serde::{Deserialize, Serialize};

use crate::nodes::inline::inline_node::InlineNode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heading {
    pub content: Option<Vec<InlineNode>>,
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub level: i8,
    pub local_id: Option<String>,
}

impl Heading {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        use html_escape::encode_quoted_attribute;
        let tag = format!("h{}", self.attributes.level);
        let id = self.attributes.local_id
            .as_ref()
            .map(|id| format!(r#" id="{}""#, encode_quoted_attribute(id)))
            .unwrap_or_default();

        let mut html = String::new();

        if let Some(content) = &self.content {
            html = content
                .iter()
                .map(|n| n.to_html(issue_or_comment_link))
                .collect();
        }

        format!(r#"<{tag}{id}>{html}</{tag}>"#)
    }
}
