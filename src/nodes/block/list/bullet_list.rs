use serde::{Deserialize, Serialize};

use super::list_item::ListItem;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletList {
    pub content: Vec<ListItem>,
}

impl BulletList {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let html = self.content
            .iter()
            .map(|item| item.to_html(issue_or_comment_link))
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
