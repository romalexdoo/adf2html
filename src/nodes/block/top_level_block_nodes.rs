use serde::{Deserialize, Serialize};

use crate::ToHtml;
use super::{
    blockquote::Blockquote, 
    list::{bullet_list::BulletList, ordered_list::OrderedList},
    code_block::CodeBlock, 
    expand::expand::Expand,
    heading::Heading, 
    media::{media_group::MediaGroup, media_single::MediaSingle},
    panel::Panel, 
    paragraph::Paragraph, 
    table::table::Table,
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

impl ToHtml for TopLevelBlockNode {
    fn to_html(&self) -> String {
        match self {
            TopLevelBlockNode::Blockquote(blockquote) => blockquote.to_html(),
            TopLevelBlockNode::BulletList(bullet_list) => bullet_list.to_html(),
            TopLevelBlockNode::CodeBlock(code_block) => code_block.to_html(),
            TopLevelBlockNode::Expand(expand) => expand.to_html(),
            TopLevelBlockNode::Heading(heading) => heading.to_html(),
            TopLevelBlockNode::MediaGroup(media_group) => media_group.to_html(),
            TopLevelBlockNode::MediaSingle(media_single) => media_single.to_html(),
            TopLevelBlockNode::OrderedList(ordered_list) => ordered_list.to_html(),
            TopLevelBlockNode::Panel(panel) => panel.to_html(),
            TopLevelBlockNode::Paragraph(paragraph) => paragraph.to_html(),
            TopLevelBlockNode::Rule => String::from("<hr/>"),
            TopLevelBlockNode::Table(table) => table.to_html(),
        }
    }
}