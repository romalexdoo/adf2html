pub mod media_group;
pub mod media_single;

use serde::{Deserialize, Serialize};

use crate::{
    marks::link::{Attributes as LinkAttributes, Link},
    ToHtml,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
    pub marks: Option<Vec<Mark>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: Kind,
    pub collection: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub occurrence_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    File,
    Link,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Mark {
    Border,
    Link(Link),
}

impl ToHtml for Media {
    fn to_html(&self) -> String {
        let mut html = format!("{}_{}", self.attributes.collection, self.attributes.id);
        
        if let Some(marks) = &self.marks
            && let Some(Mark::Link(link)) = marks.iter().find(|m| matches!(m, Mark::Link(_))) 
        {
            html = format!(r#"<a {} style = "padding: 4px;">{}</a>"#, link.html_a_tag_attributes(), html);
        }

        html
    }
}

impl Media {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        if urls.is_empty() {
            return;
        }

        if matches!(self.attributes.kind, Kind::File) {
            self.attributes.kind = Kind::Link;

            if let Some(marks) = &self.marks
                && marks.iter().any(|m| matches!(m, Mark::Link(_))) 
            {
                return;
            }

            if let Some(url) = urls.pop() {
                let link_attributes = LinkAttributes {
                    href: url,
                    collection: None,
                    id: None,
                    occurrence_key: None,
                    title: None,
                };

                let link = Link {
                    attributes: link_attributes,
                };

                let mark = Mark::Link(link);

                self.marks
                    .get_or_insert_with(Vec::new)
                    .push(mark);
            }
        }
    }
}
