use serde::{
    de::{self, IntoDeserializer as _, Unexpected}, 
    Deserialize, 
    Deserializer, 
    Serialize
};

use crate::ToHtml;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_access_level")]
    pub access_level: Option<AccessLevel>,
    pub text: Option<String>,
    pub user_type: Option<UserType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccessLevel {
    None,
    Site,
    Application, 
    Container,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserType {
    Default,
    Special,
    App,
}

impl ToHtml for Mention {
    fn to_html(&self) -> String {
        use html_escape::encode_text;
        let mention_string = self.attributes.text.as_deref().unwrap_or_default();
        if mention_string.is_empty() {
            return String::new();
        }
        format!(r#"<span style="padding: 2px 4px; background-color: #DEEBFF; border-radius: 3px;">{}</span>"#, encode_text(mention_string))
    }
}

fn deserialize_access_level<'de, D>(deserializer: D) -> Result<Option<AccessLevel>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;

    match opt.as_deref() {
        Some("") => Ok(Some(AccessLevel::None)),
        Some(s) => AccessLevel::deserialize(s.into_deserializer())
            .map(Some)
            .map_err(|_: D::Error| de::Error::invalid_value(
                Unexpected::Str(s),
                &"a valid access level string",
            )),
        None => Ok(None),
    }
}
