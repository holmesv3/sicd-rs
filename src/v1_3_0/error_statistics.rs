use super::Parameter;
use serde::Deserialize;

pub use crate::dep::v0_4_0::error_statistics::{
    Decorr, IonoError, PosVelErr, RadarSensor, TropoError,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorStatistics {
    #[serde(rename = "CompositeSCP")]
    pub composite_scp: Option<CompositeSCP>,
    #[serde(rename = "Components")]
    pub components: Option<Components>,
    #[serde(rename = "Unmodeled")]
    pub unmodeled: Option<Unmodeled>,
    #[serde(rename = "AdditionalParams")]
    pub additional_params: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CompositeSCP {
    #[serde(rename = "Rg")]
    pub rg: f64,
    #[serde(rename = "Az")]
    pub az: f64,
    #[serde(rename = "RgAz")]
    pub rg_az: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Components {
    #[serde(rename = "PosVelErr")]
    pub pos_vel_err: PosVelErr,
    #[serde(rename = "RadarSensor")]
    pub radar_sensor: RadarSensor,
    #[serde(rename = "TropoErro")]
    pub tropo_erro: Option<TropoError>,
    #[serde(rename = "IonoError")]
    pub iono_error: Option<IonoError>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Unmodeled {
    #[serde(rename = "Xrow")]
    pub xrow: f64,
    #[serde(rename = "Ycol")]
    pub ycol: f64,
    #[serde(rename = "XrowYcol")]
    pub xrow_ycol: f64,
    #[serde(rename = "UnmodeledDecorr")]
    pub unmodeled_decorr: Option<UnmodeledDecorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct UnmodeledDecorr {
    #[serde(rename = "Xrow")]
    pub xrow: Decorr,
    #[serde(rename = "Ycol")]
    pub ycol: Decorr,
}

#[cfg(test)]
mod tests {
    use super::ErrorStatistics;
    use quick_xml::de::from_str;

    #[test]
    fn test_errror_statistics() {
        let xml_str = r#"<ErrorStatistics><Components><PosVelErr><Frame>RIC_ECI
            </Frame><P1>0</P1><P2>0</P2><P3>0</P3><V1>0</V1><V2>0</V2><V3>0</V3>
            <CorrCoefs><P1P2>0</P1P2><P1P3>0</P1P3><P1V1>0</P1V1><P1V2>0</P1V2>
            <P1V3>0</P1V3><P2P3>0</P2P3><P2V1>0</P2V1><P2V2>0</P2V2><P2V3>0
            </P2V3><P3V1>0</P3V1><P3V2>0</P3V2><P3V3>0</P3V3><V1V2>0</V1V2>
            <V1V3>0</V1V3><V2V3>0</V2V3></CorrCoefs></PosVelErr><RadarSensor>
            <RangeBias>0</RangeBias><ClockFreqSF>0</ClockFreqSF><TransmitFreqSF>
            0</TransmitFreqSF></RadarSensor><TropoError><TropoRangeVertical>0
            </TropoRangeVertical><TropoRangeSlant>0</TropoRangeSlant>
            </TropoError></Components></ErrorStatistics>"#;
        assert!(match from_str::<ErrorStatistics>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
