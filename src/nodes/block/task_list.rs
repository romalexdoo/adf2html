use serde::{Deserialize, Serialize};

use crate::nodes::inline::inline_node::InlineNode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskList {
    pub content: Vec<TaskItem>,
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
pub struct TaskItem {
    pub content: Option<Vec<InlineNode>>,
    #[serde(rename = "attrs")]
    pub attributes: TaskItemAttributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskItemAttributes {
    pub local_id: Option<String>,
    pub state: TaskState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskState {
    Todo,
    Done,
}

impl TaskList {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let items: String = self.content
            .iter()
            .map(|item| item.to_html(issue_or_comment_link))
            .collect();
        format!(r#"<ul style="list-style: none; padding-left: 0;">{items}</ul>"#)
    }
}

impl TaskItem {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        use html_escape::encode_quoted_attribute;
        let checked = matches!(self.attributes.state, TaskState::Done);
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

        let checked_attr = if checked { r#" checked disabled"# } else { r#" disabled"# };
        format!(
            r#"<li{id_attr} style="display: flex; align-items: baseline; gap: 6px;"><input type="checkbox"{checked_attr}>{content}</li>"#
        )
    }
}
