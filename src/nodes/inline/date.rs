use chrono::{DateTime, Timelike};
use serde::{Deserialize, Serialize};

use crate::{ToHtml, TIMEZONE};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    #[serde(rename = "attrs")]
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub timestamp: String,
}

impl ToHtml for Date {
    fn to_html(&self) -> String {
        if let Ok(timestamp) = self.attributes.timestamp.parse::<i64>()
            && let Some(date) = DateTime::from_timestamp(timestamp, 0) 
        {
            let mut fmt = String::from("%d.%m.%Y");
            
            if date.hour() != 0 || date.minute() != 0 {
                fmt.push_str(" %H:%M %:z");
            }

            let datetime_with_timezone = TIMEZONE.with(|tz| {
                let tz = *tz.borrow();
                date.with_timezone(&tz)
            });
            
            let date_string = datetime_with_timezone
                .format(&fmt)
                .to_string();

            return format!(r#"<span style = "padding: 4px;">{date_string}</span>"#);
        };

        String::new()
    }
}
