use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::list_item::ListItem;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedList {
    pub content: Vec<ListItem>,
    #[serde(rename = "attrs")]
    pub attributes: Option<Attributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub order: Option<u32>,
}

impl ToHtml for OrderedList {
    fn to_html(&self) -> String {
        let html = self.content
            .iter()
            .map(|item| item.to_html())
            .collect::<String>();

        format!(r#"<div style = "padding: 4px;"><ol>{html}</ol></div>"#)
    }
}

impl OrderedList {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}
