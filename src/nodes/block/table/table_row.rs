use serde::{Deserialize, Serialize};

use super::table_cell::TableCell;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub content: Vec<Content>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Content {
    TableHeader(TableCell),
    TableCell(TableCell),
}

impl TableRow {
    pub(crate) fn to_html(&self, row_number: Option<u32>, issue_or_comment_link: &str) -> String {
        let mut html = String::from("<tr>");

        if let Some(i) = row_number {
            html.push_str(&format!(r#"<th style = "background-color: #F1F2F4;">{i}</th>"#));
        }

        for cell in &self.content {
            match cell {
                Content::TableHeader(table_cell) => html.push_str(&table_cell.to_html(true, issue_or_comment_link)),
                Content::TableCell(table_cell) => html.push_str(&table_cell.to_html(false, issue_or_comment_link)),
            };
        }

        html.push_str("</tr>");

        html
    }

    pub(crate) fn col_widths(&self) -> Vec<u32> {
        let mut widths = Vec::new();
        for cell in &self.content {
            let cw = match cell {
                Content::TableHeader(c) => c.col_widths(),
                Content::TableCell(c) => c.col_widths(),
            };
            if let Some(w) = cw {
                widths.extend_from_slice(w);
            }
        }
        widths
    }

    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            match content {
                Content::TableHeader(cell) => cell.replace_media_urls(urls),
                Content::TableCell(cell) => cell.replace_media_urls(urls),
            }
        }        
    }
}
