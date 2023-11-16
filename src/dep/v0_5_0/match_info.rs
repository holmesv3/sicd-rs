pub use crate::dep::v0_4_0::match_info::Collect;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    #[serde(rename = "@size")]
    pub size: usize,
    #[serde(rename = "Collect")]
    pub collect: Vec<Collect>,
}
