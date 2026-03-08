use serde::{Deserialize, Serialize};

use crate::{
    nodes::inline::text::Text, 
    ToHtml
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeBlock {
    pub content: Option<Vec<Text>>,
    #[serde(rename = "attrs")]
    pub attributes: Option<Attributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub language: String,
}

impl ToHtml for CodeBlock {
    fn to_html(&self) -> String {
        match &self.content {
            Some(content) => {
                let html = content.iter().map(|t| t.to_html()).collect::<String>();
                format!(r#"<pre style="padding: 4px; background-color: #F4F5F7; margin-top: 12px; font-family: 'Courier New', Courier, monospace; white-space: pre-wrap; overflow-x: auto"><code>{html}</code></pre>"#)
            },
            None => String::new(),
        }
    }
}
