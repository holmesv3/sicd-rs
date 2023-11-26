use super::Parameter;
pub use crate::dep::v0_4_0::radar_collection::{Area, TxFrequency, TxSequence, Waveform};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {
    #[serde(rename = "TxFrequency")]
    pub tx_frequency: TxFrequency,
    #[serde(rename = "RefFreqIndex")]
    pub ref_freq_index: Option<i32>,
    #[serde(rename = "Waveform")]
    pub waveform: Option<Waveform>,
    #[serde(rename = "TxPolarization")]
    pub tx_polarization: TxPolarization,
    #[serde(rename = "TxSequence")]
    pub tx_sequence: Option<TxSequence>,
    #[serde(rename = "RcvChannels")]
    pub rcv_channels: RcvChannels,
    #[serde(rename = "Area")]
    pub area: Option<Area>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxPolarization {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChannels {
    #[serde(rename = "@size")]
    pub size: i32,
    #[serde(rename = "ChanParameters")]
    pub chan_parameters: Vec<ChanParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ChanParameters {
    #[serde(rename = "@index")]
    pub index: i32,
    #[serde(rename = "TxRcvPolarization")]
    pub tx_rcv_polarization: TxRcvPolarization,
    #[serde(rename = "RcvAPCIndex")]
    pub rcv_apc_index: Option<i32>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxRcvPolarization {
    #[serde(rename = "$text")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::RadarCollection;
    use quick_xml::de::from_str;

    #[test]
    fn test_radar_collection() {
        let xml_str = r#"<RadarCollection><TxFrequency><Min>0</Min><Max>0</Max>
            </TxFrequency><Waveform size="1"><WFParameters index="1">
            <TxPulseLength>0</TxPulseLength><TxRFBandwidth>0</TxRFBandwidth>
            <TxFreqStart>0</TxFreqStart><TxFMRate>0</TxFMRate><RcvDemodType>
            CHIRP</RcvDemodType><RcvWindowLength>0</RcvWindowLength>
            <ADCSampleRate>0</ADCSampleRate><RcvIFBandwidth>0</RcvIFBandwidth>
            <RcvFreqStart>0</RcvFreqStart><RcvFMRate>0</RcvFMRate>
            </WFParameters></Waveform><TxPolarization>V</TxPolarization>
            <RcvChannels size="1"><ChanParameters index="1"><TxRcvPolarization>
            OTHER</TxRcvPolarization><RcvAPCIndex>1</RcvAPCIndex></ChanParameters>
            </RcvChannels><Area><Corner><ACP index="1"><Lat>0</Lat><Lon>0</Lon>
            <HAE>0</HAE></ACP></Corner><Plane><RefPt><ECF><X>0</X><Y>0</Y><Z>0
            </Z></ECF><Line>0</Line><Sample>0</Sample></RefPt><XDir><UVectECF>
            <X>0</X><Y>0</Y><Z>0</Z></UVectECF><LineSpacing>0</LineSpacing>
            <NumLines>0</NumLines><FirstLine>0</FirstLine></XDir><YDir>
            <UVectECF><X>0</X><Y>0</Y><Z>0</Z></UVectECF><SampleSpacing>0
            </SampleSpacing><NumSamples>0</NumSamples><FirstSample>0
            </FirstSample></YDir></Plane></Area></RadarCollection>"#;
        assert!(match from_str::<RadarCollection>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
