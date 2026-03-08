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
        if let Some(url) = &self.attributes.url {
            format!(r#"<a style = "padding: 4px;" href = "{0}">{0}</a>"#, url)
        } else {
            format!(r#"<a style = "padding: 4px;" href = "{0}">{0}</a>"#, issue_or_comment_link)
        }
    }
}
