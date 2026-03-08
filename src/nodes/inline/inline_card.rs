use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineCard {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub id: Option<String>,
    pub data: Option<String>,
    pub url: Option<String>,
    pub collection: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

impl InlineCard {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        use html_escape::{encode_quoted_attribute, encode_text};
        let url = self.attributes.url.as_deref().unwrap_or(issue_or_comment_link);
        format!(
            r#"<a href="{}">{}</a>"#,
            encode_quoted_attribute(url),
            encode_text(url)
        )
    }
}
