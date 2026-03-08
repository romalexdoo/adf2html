use serde::{Deserialize, Serialize};

use crate::ToHtml;

use super::super::{
    blockquote::Blockquote, 
    list::{bullet_list::BulletList, ordered_list::OrderedList},
    code_block::CodeBlock, 
    heading::Heading, 
    media::media_group::MediaGroup, 
    expand::nested_expand::NestedExpand, 
    panel::Panel, 
    paragraph::Paragraph, 
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub content: Vec<Content>,
    #[serde(rename = "attrs")]
    pub attributes: Option<Attributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub background: Option<String>,
    pub colspan: Option<u32>,
    pub col_width: Option<Vec<u32>>,
    pub row_span: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    Blockquote(Blockquote),
    BulletList(BulletList),
    CodeBlock(CodeBlock),
    Heading(Heading),
    MediaGroup(MediaGroup),
    OrderedList(OrderedList),
    Panel(Panel),
    Paragraph(Paragraph),
    Rule,
    NestedExpand(NestedExpand),
}

impl TableCell {
    pub(crate) fn to_html(&self, is_header_cell: bool, issue_or_comment_link: &str) -> String {
        let tag = match is_header_cell {
            false => "td",
            true => "th",
        };

        let mut tag_attributes = String::new();
        let mut col_widths = String::new();

        if let Some(attributes) = &self.attributes {
            if let Some(background) = &attributes.background {
                tag_attributes.push_str( &format!(r#" style = "background-color: {background};""#));
            }

            if let Some(colspan) = attributes.colspan {
                tag_attributes.push_str( &format!(r#" colspan="{colspan}""#));
            }
            
            if let Some(col_width) = &attributes.col_width {
                col_widths.push_str("<colgroup>");

                for width in col_width {
                    if *width == 0 {
                        col_widths.push_str(&format!(r#"<col style="width: {width}px;">"#));
                    } else {
                        col_widths.push_str(r#"<col style="width: auto;">"#);
                    }
                }

                col_widths.push_str("</colgroup>");
            }
            
            if let Some(row_span) = attributes.row_span {
                tag_attributes.push_str( &format!(r#" rowspan="{row_span}""#));
            }            
        }

        let content = self.content
            .iter()
            .map(|n| n.to_html(issue_or_comment_link))
            .collect::<String>();

        format!("{col_widths}<{tag}{tag_attributes}>{content}</{tag}>")
    }

    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}

impl Content {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        match self {
            Content::Blockquote(blockquote) => blockquote.to_html(issue_or_comment_link),
            Content::BulletList(bullet_list) => bullet_list.to_html(issue_or_comment_link),
            Content::CodeBlock(code_block) => code_block.to_html(),
            Content::Heading(heading) => heading.to_html(issue_or_comment_link),
            Content::MediaGroup(media_group) => media_group.to_html(),
            Content::OrderedList(ordered_list) => ordered_list.to_html(issue_or_comment_link),
            Content::Panel(panel) => panel.to_html(issue_or_comment_link),
            Content::Paragraph(paragraph) => paragraph.to_html(issue_or_comment_link),
            Content::Rule => String::from("<hr/>"),
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
            Content::OrderedList(ordered_list) => ordered_list.replace_media_urls(urls),
            Content::Panel(panel) => panel.replace_media_urls(urls),
            Content::Paragraph(_paragraph) => (),
            Content::Rule => (),
            Content::NestedExpand(nested_expand) => nested_expand.replace_media_urls(urls),
    }
    }
}
