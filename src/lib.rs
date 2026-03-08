pub mod document;
pub mod marks;
pub mod nodes;

pub use chrono_tz::*;
use std::cell::RefCell;

thread_local! {
    pub(crate) static TIMEZONE: RefCell<Tz> = const { RefCell::new(UTC) };  // Default to UTC
}

pub(crate) trait ToHtml {
    fn to_html(&self) -> String;
}
