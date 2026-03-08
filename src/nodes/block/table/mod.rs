pub mod table_cell;
pub mod table_row;

use serde::{Deserialize, Serialize};

use table_row::TableRow;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub content: Vec<TableRow>,
    #[serde(rename = "attrs")]
    pub attributes: Option<Attributes>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub is_number_column_enabled: Option<bool>,
    pub width: Option<u32>,
    pub layout: Option<Layout>,
    pub display_mode: Option<DisplayMode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layout {
    Center,
    AlignStart,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    Default,
    Fixed,
}

impl Table {
    pub fn to_html(&self, issue_or_comment_link: &str) -> String {
        let mut style = String::from(r#"padding: 4px;"#);
        let mut is_number_column_enabled = false;

        if let Some(attributes) = &self.attributes {
            is_number_column_enabled = attributes.is_number_column_enabled.unwrap_or_default();
            
            if let Some(width) = attributes.width {
                style.push_str(&format!("width: {width}px; "));
            } else if let Some(layout) = &attributes.layout {
                match layout {
                    Layout::Center => style.push_str("margin: 0 auto; "),
                    Layout::AlignStart => style.push_str("margin: 0; "),
                }
            }

            if let Some(display_mode) = &attributes.display_mode {
                match display_mode {
                    DisplayMode::Default => (),
                    DisplayMode::Fixed => style.push_str("table-layout: fixed; max-width: 100%; overflow: auto; "),
                }
            }
        }

        style = format!(r#" style = "{style}""#);

        // Build colgroup from the first row's cell col_width values
        let colgroup = self.content
            .first()
            .map(|first_row| {
                let widths = first_row.col_widths();
                if widths.is_empty() {
                    String::new()
                } else {
                    let cols: String = widths.iter().map(|&w| {
                        if w > 0 {
                            format!(r#"<col style="width: {w}px;">"#)
                        } else {
                            String::from("<col>")
                        }
                    }).collect();
                    format!("<colgroup>{cols}</colgroup>")
                }
            })
            .unwrap_or_default();

        let mut content = String::new();
        let mut row_number = match is_number_column_enabled {
            true => Some(0),
            false => None,
        };

        for row in &self.content {
            if let Some(ref mut num) = row_number {
                *num += 1;
            }

            content.push_str(&row.to_html(row_number, issue_or_comment_link));
        }
        
        format!("<table{style}>{colgroup}{content}</table>")
    }
}

impl Table {
    pub(crate) fn replace_media_urls(&mut self, urls: &mut Vec<String>) {
        for content in self.content.iter_mut() {
            content.replace_media_urls(urls);
        }        
    }
}
