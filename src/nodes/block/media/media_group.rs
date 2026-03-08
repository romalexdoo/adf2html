use serde::{Deserialize, Serialize};

use crate::ToHtml;use super::Media;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaGroup {
    pub content: Vec<Media>,
}

impl ToHtml for MediaGroup {
    fn to_html(&self) -> String {
        self.content
            .iter()
            .map(|m| format!(r#"<p style = "padding: 4px;">{}</p>"#, m.to_html()))
            .collect()
    }
}

impl MediaGroup {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}
