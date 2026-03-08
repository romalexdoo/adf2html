use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::Media;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSingle {
    pub content: Vec<Media>,
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub layout: Layout,
    pub width: Option<f32>,
    pub width_type: Option<WidthType>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    AlignEnd,
    AlignStart,
    Center,
    FullWidth,
    WrapLeft,
    WrapRight,
    Wide,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WidthType {
    Pixel,
    Percentage,
}

impl ToHtml for MediaSingle {
    fn to_html(&self) -> String {
        self.content
            .iter()
            .map(|m| format!(r#"<p style = "padding: 4px;">{}</p>"#, m.to_html()))
            .collect()
    }
}

impl MediaSingle {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}
