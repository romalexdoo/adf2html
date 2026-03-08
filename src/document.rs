use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    nodes::block::top_level_block_nodes::TopLevelBlockNode, 
    TIMEZONE
};

static ATTACHMENT_URL_REGEX: OnceLock<Regex> = OnceLock::new();

fn attachment_url_regex() -> &'static Regex {
    ATTACHMENT_URL_REGEX.get_or_init(|| {
        Regex::new(r"/rest/api/3/attachment/content/\d+").unwrap()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub version: u8,
    #[serde(rename = "type")]
    pub kind: String,
    pub content: Vec<TopLevelBlockNode>,
}

impl Document {
    pub fn to_html(&self, timezone: Option<chrono_tz::Tz>, issue_or_comment_link: &str) -> String {
        TIMEZONE.with(|tz| {
            *tz.borrow_mut() = timezone.unwrap_or(chrono_tz::UTC);
        });
        
        let mut html = String::new();

        for node in &self.content {
            html.push_str(&node.to_html(issue_or_comment_link));
        }

        html
    }

    pub fn replace_media_urls(&mut self, api_domain_name: &str, html_rendered: &str) {
        let regex = attachment_url_regex();

        let mut urls: Vec<String> = regex
            .find_iter(html_rendered)
            .map(|mat| format!("{}{}", api_domain_name, mat.as_str()))
            .collect();

        urls.reverse();

        for node in self.content.iter_mut() {
            node.replace_media_urls(&mut urls);
        }
    }
}
