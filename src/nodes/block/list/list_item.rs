use serde::{Deserialize, Serialize};

use crate::ToHtml;

use super::{
    bullet_list::BulletList, 
    ordered_list::OrderedList, 
    super::{
        code_block::CodeBlock,
        media::media_single::MediaSingle,
        paragraph::Paragraph,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub content: Vec<Content>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    BulletList(BulletList),
    CodeBlock(CodeBlock),
    MediaSingle(MediaSingle),
    OrderedList(OrderedList),
    Paragraph(Paragraph),
}

impl ListItem {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let mut content = String::new();

        for node in &self.content {
            let html = match node {
                Content::BulletList(bullet_list) => bullet_list.to_html(issue_or_comment_link),
                Content::CodeBlock(code_block) => format!("<li>{}</li>", code_block.to_html()),
                Content::MediaSingle(media_single) => format!("<li>{}</li>", media_single.to_html()),
                Content::OrderedList(ordered_list) => ordered_list.to_html(issue_or_comment_link),
                Content::Paragraph(paragraph) => format!("<li>{}</li>", paragraph.to_html(issue_or_comment_link)),
            };

            content.push_str(&html);
        }

        content
    }
}

impl ListItem {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}

impl Content {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        match self {
            Content::BulletList(bullet_list) => bullet_list.replace_media_urls(urls),
            Content::CodeBlock(_code_block) => (),
            Content::MediaSingle(media_single) => media_single.replace_media_urls(urls),
            Content::OrderedList(ordered_list) => ordered_list.replace_media_urls(urls),
            Content::Paragraph(_paragraph) => (),
    }
    }
}
