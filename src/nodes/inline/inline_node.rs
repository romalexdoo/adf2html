use serde::{Deserialize, Serialize};

use crate::ToHtml;

use super::{
    date::Date, 
    emoji::Emoji, 
    inline_card::InlineCard, 
    mention::Mention, 
    status::Status, 
    text::Text, 
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum InlineNode {
    Date(Date),
    Emoji(Emoji),
    HardBreak,
    InlineCard(InlineCard),
    Mention(Mention),
    Status(Status),
    Text(Text),
}

impl InlineNode {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        match self {
            InlineNode::Date(date) => date.to_html(),
            InlineNode::Emoji(emoji) => emoji.to_html(),
            InlineNode::HardBreak => String::from("<br/>"),
            InlineNode::InlineCard(inline_card) => inline_card.to_html(issue_or_comment_link),
            InlineNode::Mention(mention) => mention.to_html(),
            InlineNode::Status(status) => status.to_html(),
            InlineNode::Text(text) => text.to_html(),
        }
    }
}
