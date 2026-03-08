use serde::{Deserialize, Serialize};

use crate::ToHtml;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub color: Color,
    pub text: String,
    pub local_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Neutral,
    Blue,
    Green,
    Purple,
    Red,
    Yellow,
}

impl ToHtml for Status {
    fn to_html(&self) -> String {
        use html_escape::{encode_quoted_attribute, encode_text};
        let local_id = self.attributes.local_id
            .as_ref()
            .map(|id| format!(r#" id="{}""#, encode_quoted_attribute(id)))
            .unwrap_or_default();
        
        let color = match self.attributes.color {
            Color::Neutral => "background-color: rgba(9, 30, 66, 0.08)",
            Color::Blue => "color: #0052CC; background-color: #DEEBFF",
            Color::Green => "color: #006644; background-color: #E3FCEF",
            Color::Purple => "color: #403294; background-color: #EAE6FF",
            Color::Red => "color: #DE350B; background-color: #FFEBE6",
            Color::Yellow => "color: rgb(151, 79, 12); background-color: #FFFAE6",
        };

        format!(
            r#"<span{local_id} style="{color}; padding-left: 4px; padding-right: 4px; font-weight: bold">{}</span>"#,
            encode_text(&self.attributes.text.to_uppercase())
        )
    }
}
