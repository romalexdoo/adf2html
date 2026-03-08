use serde::{Deserialize, Serialize};

use crate::nodes::inline::inline_node::InlineNode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionList {
    pub content: Vec<DecisionItem>,
    #[serde(rename = "attrs")]
    pub attributes: Option<Attributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub local_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionItem {
    pub content: Option<Vec<InlineNode>>,
    #[serde(rename = "attrs")]
    pub attributes: DecisionItemAttributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecisionItemAttributes {
    pub local_id: Option<String>,
    pub state: DecisionState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionState {
    Decided,
}

impl DecisionList {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let items: String = self.content
            .iter()
            .map(|item| item.to_html(issue_or_comment_link))
            .collect();
        format!(r#"<ul style="list-style: none; padding-left: 0;">{items}</ul>"#)
    }
}

impl DecisionItem {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        use html_escape::encode_quoted_attribute;
        let id_attr = self.attributes.local_id
            .as_ref()
            .map(|id| format!(r#" id="{}""#, encode_quoted_attribute(id)))
            .unwrap_or_default();

        let content: String = self.content
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|n| n.to_html(issue_or_comment_link))
            .collect();

        // Decision items are represented with a "✓" icon
        format!(
            r#"<li{id_attr} style="display: flex; align-items: baseline; gap: 6px;"><span style="color: #00875A;">&#10003;</span>{content}</li>"#
        )
    }
}
