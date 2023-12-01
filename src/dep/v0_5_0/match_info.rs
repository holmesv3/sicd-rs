pub use crate::dep::v0_4_0::match_info::Collect;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    #[serde(rename = "@size")]
    pub size: i32,
    #[serde(rename = "Collect")]
    pub collect: Vec<Collect>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_match_info() {
        let xml = r#"
          <MatchInfo size="1686">
    <Collect index="-4879">
      <CollectorName>string</CollectorName>
      <IlluminatorName>string</IlluminatorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Collect>
    <Collect index="2689">
      <CollectorName>string</CollectorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <Parameter name="string">string</Parameter>
    </Collect>
  </MatchInfo>
        "#;
        assert!(match quick_xml::de::from_str::<super::MatchInfo>(xml) {
            Ok(_) => true,
            Err(e) => {
                dbg!(e);
                false
            }
        });
    }
}
