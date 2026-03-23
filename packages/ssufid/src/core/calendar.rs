use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SsufidCalendar {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub starts_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub ends_at: Option<time::OffsetDateTime>,
    pub location: Option<String>,
    pub url: Option<String>,
}
