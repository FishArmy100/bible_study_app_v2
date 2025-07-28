use serde::{Deserialize, Serialize};

pub mod bible;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JsonFormat
{
    Json,
    JsonLines,
}
