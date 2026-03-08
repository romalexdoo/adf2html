use html_escape::encode_text;
use serde::{Deserialize, Serialize};

use crate::ToHtml;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub id: Option<String>,
    pub short_name: String,
    pub text: Option<String>,
}

impl ToHtml for Emoji {
    fn to_html(&self) -> String {
        // Prefer the Unicode text field; skip rendering if only a raw short-name is available
        if let Some(text) = &self.attributes.text {
            encode_text(text).into_owned()
        } else {
            String::new()
        }
    }
}
