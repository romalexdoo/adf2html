use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::list_item::ListItem;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletList {
    pub content: Vec<ListItem>,
}

impl ToHtml for BulletList {
    fn to_html(&self) -> String {
        let html = self.content
            .iter()
            .map(|item| item.to_html())
            .collect::<String>();

        format!(r#"<div style = "padding: 4px;"><ul>{html}</ul></div>"#)
    }
}

impl BulletList {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}
