pub mod nested_expand;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    marks::mark::Mark,
    nodes::inline::text::Text,
    ToHtml
};

use {
    nested_expand::NestedExpand, 
    super::{
        blockquote::Blockquote, 
        list::{bullet_list::BulletList, ordered_list::OrderedList},
        code_block::CodeBlock, 
        heading::Heading, 
        media::{media_group::MediaGroup, media_single::MediaSingle},
        panel::Panel, 
        paragraph::Paragraph, 
        table::Table, 
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expand {
    pub content: Vec<Content>,
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
    pub marks: Option<Vec<Mark>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    Blockquote(Blockquote),
    BulletList(BulletList),
    CodeBlock(CodeBlock),
    Heading(Heading),
    MediaGroup(MediaGroup),
    MediaSingle(MediaSingle),
    OrderedList(OrderedList),
    Panel(Panel),
    Paragraph(Paragraph),
    Rule,
    Table(Table),
    NestedExpand(NestedExpand),
}

impl Expand {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let title = Text::new(self.attributes.title.clone(), self.marks.clone()).to_html();
        let content = self.content
            .iter()
            .map(|n| n.to_html(issue_or_comment_link))
            .collect::<String>();

        expand_html_formatting(title, content)
    }
}

impl Expand {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}

pub(crate) fn expand_html_formatting(title: String, content: String) -> String {
    let arrow_id = Uuid::new_v4().to_string();
    let expandable_content_id = Uuid::new_v4().to_string();
    
    format!(
        r#"
            <p 
                style="display: flex; align-items: center; cursor: pointer; padding: 4px;" 
                onclick="
                    var content = document.getElementById('{expandable_content_id}');
                    var arrow = document.getElementById('{arrow_id}');
                    content.style.display = (content.style.display === 'none' || content.style.display === '') ? 'block' : 'none';
                    arrow.style.transform = (arrow.style.transform === 'rotate(90deg)') ? 'rotate(0deg)' : 'rotate(90deg)';
                "
            >
                <span style="display: inline-flex; align-items: center;">
                    <svg width="24" height="24" id="{arrow_id}" style="transition: transform 0.3s ease; vertical-align: middle;">
                    <path d="M10.294 9.698a.99.99 0 0 1 0-1.407 1.01 1.01 0 0 1 1.419 0l2.965 2.94a1.09 1.09 0 0 1 0 1.548l-2.955 2.93a1.01 1.01 0 0 1-1.42 0 .99.99 0 0 1 0-1.407l2.318-2.297z"></path>
                    </svg>
                </span>
                {title}
            </p>
            <div style="display: none; margin-left: 32px;" id="{expandable_content_id}">
                {content}
            </div>
        "#
    )
}

impl Content {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        match self {
            Content::Blockquote(blockquote) => blockquote.to_html(issue_or_comment_link),
            Content::BulletList(bullet_list) => bullet_list.to_html(issue_or_comment_link),
            Content::CodeBlock(code_block) => code_block.to_html(),
            Content::Heading(heading) => heading.to_html(issue_or_comment_link),
            Content::MediaGroup(media_group) => media_group.to_html(),
            Content::MediaSingle(media_single) => media_single.to_html(),
            Content::OrderedList(ordered_list) => ordered_list.to_html(issue_or_comment_link),
            Content::Panel(panel) => panel.to_html(issue_or_comment_link),
            Content::Paragraph(paragraph) => paragraph.to_html(issue_or_comment_link),
            Content::Rule => String::from("<hr/>"),
            Content::Table(table) => table.to_html(issue_or_comment_link),
            Content::NestedExpand(nested_expand) => nested_expand.to_html(issue_or_comment_link),
        }
    }
}

impl Content {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        match self {
            Content::Blockquote(blockquote) => blockquote.replace_media_urls(urls),
            Content::BulletList(bullet_list) => bullet_list.replace_media_urls(urls),
            Content::CodeBlock(_code_block) => (),
            Content::Heading(_heading) => (),
            Content::MediaGroup(media_group) => media_group.replace_media_urls(urls),
            Content::MediaSingle(media_single) => media_single.replace_media_urls(urls),
            Content::OrderedList(ordered_list) => ordered_list.replace_media_urls(urls),
            Content::Panel(panel) => panel.replace_media_urls(urls),
            Content::Paragraph(_paragraph) => (),
            Content::Rule => (),
            Content::Table(table) => table.replace_media_urls(urls),
            Content::NestedExpand(nested_expand) => nested_expand.replace_media_urls(urls),
        }
    }
}
