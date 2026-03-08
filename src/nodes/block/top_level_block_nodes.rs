use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::{
    blockquote::Blockquote, 
    list::{bullet_list::BulletList, ordered_list::OrderedList},
    code_block::CodeBlock, 
    expand::Expand,
    heading::Heading, 
    media::{media_group::MediaGroup, media_single::MediaSingle},
    panel::Panel, 
    paragraph::Paragraph, 
    table::Table,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TopLevelBlockNode {
    Blockquote(Blockquote),
    BulletList(BulletList),
    CodeBlock(CodeBlock),
    Expand(Expand),
    Heading(Heading),
    MediaGroup(MediaGroup),
    MediaSingle(MediaSingle),
    OrderedList(OrderedList),
    Panel(Panel),
    Paragraph(Paragraph),
    Rule,
    Table(Table),
}

impl TopLevelBlockNode {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        match self {
            TopLevelBlockNode::Blockquote(blockquote) => blockquote.to_html(issue_or_comment_link),
            TopLevelBlockNode::BulletList(bullet_list) => bullet_list.to_html(issue_or_comment_link),
            TopLevelBlockNode::CodeBlock(code_block) => code_block.to_html(),
            TopLevelBlockNode::Expand(expand) => expand.to_html(issue_or_comment_link),
            TopLevelBlockNode::Heading(heading) => heading.to_html(issue_or_comment_link),
            TopLevelBlockNode::MediaGroup(media_group) => media_group.to_html(),
            TopLevelBlockNode::MediaSingle(media_single) => media_single.to_html(),
            TopLevelBlockNode::OrderedList(ordered_list) => ordered_list.to_html(issue_or_comment_link),
            TopLevelBlockNode::Panel(panel) => panel.to_html(issue_or_comment_link),
            TopLevelBlockNode::Paragraph(paragraph) => paragraph.to_html(issue_or_comment_link),
            TopLevelBlockNode::Rule => String::from("<hr/>"),
            TopLevelBlockNode::Table(table) => table.to_html(issue_or_comment_link),
        }
    }
}

impl TopLevelBlockNode {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        match self {
            TopLevelBlockNode::Blockquote(blockquote) => blockquote.replace_media_urls(urls),
            TopLevelBlockNode::BulletList(bullet_list) => bullet_list.replace_media_urls(urls),
            TopLevelBlockNode::CodeBlock(_code_block) => (),
            TopLevelBlockNode::Expand(expand) => expand.replace_media_urls(urls),
            TopLevelBlockNode::Heading(_heading) => (),
            TopLevelBlockNode::MediaGroup(media_group) => media_group.replace_media_urls(urls),
            TopLevelBlockNode::MediaSingle(media_single) => media_single.replace_media_urls(urls),
            TopLevelBlockNode::OrderedList(ordered_list) => ordered_list.replace_media_urls(urls),
            TopLevelBlockNode::Panel(panel) => panel.replace_media_urls(urls),
            TopLevelBlockNode::Paragraph(_paragraph) => (),
            TopLevelBlockNode::Rule => (),
            TopLevelBlockNode::Table(table) => table.replace_media_urls(urls),
        }
    }
}
