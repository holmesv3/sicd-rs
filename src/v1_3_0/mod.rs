//! Common types and metadata definition for SICD Version 1.3.0 [2021-11-30]
//!
//! Backwards compatible with version 1, 1.1, 1.2.1
use serde::Deserialize;

pub mod antenna;
pub mod error_statistics;
pub mod geo_data;
pub mod image_data;
pub mod image_formation;
pub mod match_info;
pub mod pfa;
pub mod position;
pub mod radar_collection;
pub mod radiometric;
pub mod scpcoa;
pub mod timeline;

pub use crate::dep::v0_4_0::collection_info::CollectionInfo;
pub use crate::dep::v0_4_0::image_creation::ImageCreation;
pub use crate::dep::v0_4_0::{
    IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Poly1D, Poly2D, RowCol, XyzPoly, CMPLX, LL, LLH, XYZ,
};
pub use crate::dep::v0_5_0::grid::Grid;
use antenna::Antenna;
use error_statistics::ErrorStatistics;
use geo_data::GeoData;
use image_data::ImageData;
use image_formation::{ImageFormation, RgAzComp, Rma};
use match_info::MatchInfo;
use pfa::Pfa;
use position::Position;
use radar_collection::RadarCollection;
use radiometric::Radiometric;
use scpcoa::ScpCoa;
use timeline::Timeline;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SicdMeta {
    #[serde(rename = "CollectionInfo")]
    pub collection_info: CollectionInfo,
    #[serde(rename = "ImageCreation")]
    pub image_creation: Option<ImageCreation>,
    #[serde(rename = "ImageData")]
    pub image_data: ImageData,
    #[serde(rename = "GeoData")]
    pub geo_data: GeoData,
    #[serde(rename = "Grid")]
    pub grid: Grid,
    #[serde(rename = "Timeline")]
    pub timeline: Timeline,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "RadarCollection")]
    pub radar_collection: RadarCollection,
    #[serde(rename = "ImageFormation")]
    pub image_formation: ImageFormation,
    #[serde(rename = "SCPCOA")]
    pub scpcoa: ScpCoa,
    #[serde(rename = "Radiometric")]
    pub radiometric: Option<Radiometric>,
    #[serde(rename = "Antenna")]
    pub antenna: Option<Antenna>,
    #[serde(rename = "ErrorStatistics")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "MatchInfo")]
    pub match_info: Option<MatchInfo>,
    #[serde(rename = "RgAzComp")]
    pub rg_az_comp: Option<RgAzComp>,
    #[serde(rename = "PFA")]
    pub pfa: Option<Pfa>,
    #[serde(rename = "RMA")]
    pub rma: Option<Rma>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Parameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SinglePolarization {
    V,
    H,
    X,
    Y,
    S,
    E,
    RHC,
    LHC,
    OTHER,
    UNKNOWN,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum DualPolarization {
    #[serde(rename = "V:V")]
    V_V,
    #[serde(rename = "V:H")]
    V_H,
    #[serde(rename = "V:X")]
    V_X,
    #[serde(rename = "V:Y")]
    V_Y,
    #[serde(rename = "V:S")]
    V_S,
    #[serde(rename = "V:E")]
    V_E,
    #[serde(rename = "V:RHC")]
    V_RHC,
    #[serde(rename = "V:LHC")]
    V_LHC,
    #[serde(rename = "V:OTHER")]
    V_OTHER,
    #[serde(rename = "H:V")]
    H_V,
    #[serde(rename = "H:H")]
    H_H,
    #[serde(rename = "H:X")]
    H_X,
    #[serde(rename = "H:Y")]
    H_Y,
    #[serde(rename = "H:S")]
    H_S,
    #[serde(rename = "H:E")]
    H_E,
    #[serde(rename = "H:RHC")]
    H_RHC,
    #[serde(rename = "H:LHC")]
    H_LHC,
    #[serde(rename = "H:OTHER")]
    H_OTHER,
    #[serde(rename = "X:V")]
    X_V,
    #[serde(rename = "X:H")]
    X_H,
    #[serde(rename = "X:X")]
    X_X,
    #[serde(rename = "X:Y")]
    X_Y,
    #[serde(rename = "X:S")]
    X_S,
    #[serde(rename = "X:E")]
    X_E,
    #[serde(rename = "X:RHC")]
    X_RHC,
    #[serde(rename = "X:LHC")]
    X_LHC,
    #[serde(rename = "X:OTHER")]
    X_OTHER,
    #[serde(rename = "Y:V")]
    Y_V,
    #[serde(rename = "Y:H")]
    Y_H,
    #[serde(rename = "Y:X")]
    Y_X,
    #[serde(rename = "Y:Y")]
    Y_Y,
    #[serde(rename = "Y:S")]
    Y_S,
    #[serde(rename = "Y:E")]
    Y_E,
    #[serde(rename = "Y:RHC")]
    Y_RHC,
    #[serde(rename = "Y:LHC")]
    Y_LHC,
    #[serde(rename = "Y:OTHER")]
    Y_OTHER,
    #[serde(rename = "S:V")]
    S_V,
    #[serde(rename = "S:H")]
    S_H,
    #[serde(rename = "S:X")]
    S_X,
    #[serde(rename = "S:Y")]
    S_Y,
    #[serde(rename = "S:S")]
    S_S,
    #[serde(rename = "S:E")]
    S_E,
    #[serde(rename = "S:RHC")]
    S_RHC,
    #[serde(rename = "S:LHC")]
    S_LHC,
    #[serde(rename = "S:OTHER")]
    S_OTHER,
    #[serde(rename = "E:V")]
    E_V,
    #[serde(rename = "E:H")]
    E_H,
    #[serde(rename = "E:X")]
    E_X,
    #[serde(rename = "E:Y")]
    E_Y,
    #[serde(rename = "E:S")]
    E_S,
    #[serde(rename = "E:E")]
    E_E,
    #[serde(rename = "E:RHC")]
    E_RHC,
    #[serde(rename = "E:LHC")]
    E_LHC,
    #[serde(rename = "E:OTHER")]
    E_OTHER,
    #[serde(rename = "RHC:V")]
    RHC_V,
    #[serde(rename = "RHC:H")]
    RHC_H,
    #[serde(rename = "RHC:X")]
    RHC_X,
    #[serde(rename = "RHC:Y")]
    RHC_Y,
    #[serde(rename = "RHC:S")]
    RHC_S,
    #[serde(rename = "RHC:E")]
    RHC_E,
    #[serde(rename = "RHC:RHC")]
    RHC_RHC,
    #[serde(rename = "RHC:LHC")]
    RHC_LHC,
    #[serde(rename = "RHC:OTHER")]
    RHC_OTHER,
    #[serde(rename = "LHC:V")]
    LHC_V,
    #[serde(rename = "LHC:H")]
    LHC_H,
    #[serde(rename = "LHC:X")]
    LHC_X,
    #[serde(rename = "LHC:Y")]
    LHC_Y,
    #[serde(rename = "LHC:S")]
    LHC_S,
    #[serde(rename = "LHC:E")]
    LHC_E,
    #[serde(rename = "LHC:RHC")]
    LHC_RHC,
    #[serde(rename = "LHC:LHC")]
    LHC_LHC,
    #[serde(rename = "LHC:OTHER")]
    LHC_OTHER,
    #[serde(rename = "OTHER:V")]
    OTHER_V,
    #[serde(rename = "OTHER:H")]
    OTHER_H,
    #[serde(rename = "OTHER:X")]
    OTHER_X,
    #[serde(rename = "OTHER:Y")]
    OTHER_Y,
    #[serde(rename = "OTHER:S")]
    OTHER_S,
    #[serde(rename = "OTHER:E")]
    OTHER_E,
    #[serde(rename = "OTHER:RHC")]
    OTHER_RHC,
    #[serde(rename = "OTHER:LHC")]
    OTHER_LHC,
    #[serde(rename = "OTHER:OTHER")]
    OTHER_OTHER,
    OTHER,
    UNKNOWN,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_sicd_types() {
        let xml = r#"<RowCol><Row>0</Row><Col>0</Col></RowCol>"#;
        assert!(match from_str::<RowCol>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<IdxRowCol index="0"><Row>0</Row><Col>0</Col></IdxRowCol>
        "#;
        assert!(match from_str::<IdxRowCol>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<CMPLX><Real>0</Real><Imag>0</Imag></CMPLX>"#;
        assert!(match from_str::<CMPLX>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<XYZ><X>0</X><Y>0</Y><Z>0</Z></XYZ>"#;
        assert!(match from_str::<XYZ>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<LLH><Lat>0</Lat><Lon>0</Lon><HAE>0</HAE></LLH>"#;
        assert!(match from_str::<LLH>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"
            <IdxLLH index="0"><Lat>0</Lat><Lon>0</Lon><HAE>0</HAE></IdxLLH>"#;
        assert!(match from_str::<IdxLLH>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<LL><Lat>0</Lat><Lon>0</Lon></LL>"#;
        assert!(match from_str::<LL>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<IdxLL index="0"><Lat>0</Lat><Lon>0</Lon></IdxLL>"#;
        assert!(match from_str::<IdxLL>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<Poly1d order1="1"><Coef1d exponent1="0">0</Coef1d>
            <Coef1d exponent1="1">0</Coef1d></Poly1d>"#;
        assert!(match from_str::<Poly1D>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<Poly2d order1 = "1" order2 = "1">
            <Coef2d exponent1="0" exponent2="0">0</Coef2d>
            <Coef2d exponent1="1" exponent2="0">0</Coef2d>
            <Coef2d exponent1="0" exponent2="1">0</Coef2d>
            <Coef2d exponent1="1" exponent2="1">0</Coef2d></Poly2d>"#;
        assert!(match from_str::<Poly2D>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<XyzPoly>
            <X order1="0"><Coef1d exponent1="0">0</Coef1d></X>
            <Y order1="0"><Coef1d exponent1="0">0</Coef1d></Y>
            <Z order1="0"><Coef1d exponent1="0">0</Coef1d></Z></XyzPoly>"#;
        assert!(match from_str::<XyzPoly>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"<IdxXyzPoly index="0">
            <X order1="0"><Coef1d exponent1="0">0</Coef1d></X>
            <Y order1="0"><Coef1d exponent1="0">0</Coef1d></Y>
            <Z order1="0"><Coef1d exponent1="0">0</Coef1d></Z></IdxXyzPoly>"#;
        assert!(match from_str::<IdxXyzPoly>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });

        let xml = r#"
            <Parameter name="Param0">TestP0</Parameter>
            <Parameter name="Param1">TestP1</Parameter>"#;
        assert!(match from_str::<Parameter>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn test_empty_parameter() {
        let xml = r#"
            <Parameter name="Param0">      </Parameter>
            <Parameter name="Param1">TestP1</Parameter>"#;
        assert!(match from_str::<Parameter>(xml) {
            Ok(_) => true,
            Err(_) => false,
        });
    }
}
