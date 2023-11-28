//! Common types and metadata definition for SICD Version 1.3.0 [2021-11-30]
//!
//! Backwards compatible with version 1, 1.1, 1.2.1
use serde::Deserialize;

pub mod antenna;
pub mod error_statistics;
pub mod geo_data;
pub mod image_formation;
pub mod match_info;
pub mod radar_collection;
pub mod radiometric;
pub mod scpcoa;

pub use crate::dep::v0_4_0::collection_info::CollectionInfo;
pub use crate::dep::v0_4_0::image_creation::ImageCreation;
pub use crate::dep::v0_4_0::image_data::ImageData;
pub use crate::dep::v0_4_0::pfa::Pfa;
pub use crate::dep::v0_4_0::position::Position;
pub use crate::dep::v0_4_0::timeline::Timeline;
pub use crate::dep::v0_4_0::{
    IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Poly1D, Poly2D, RowCol, XyzPoly, CMPLX, LL, LLH, XYZ,
};
pub use crate::dep::v0_5_0::grid::Grid;

use antenna::Antenna;
use error_statistics::ErrorStatistics;
use geo_data::GeoData;
use image_formation::{ImageFormation, RgAzComp, Rma};
use match_info::MatchInfo;
use radar_collection::RadarCollection;
use radiometric::Radiometric;
use scpcoa::ScpCoa;

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

#[cfg(test)]
mod tests {
    use super::{
        IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Parameter, Poly1D, Poly2D, RowCol, XyzPoly, CMPLX,
        LL, LLH, XYZ,
    };
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
