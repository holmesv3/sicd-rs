//! Common types and metadata definition for SICD Version 0.5.0 [2011-01-12]
use serde::Deserialize;

pub mod grid;
pub mod image_formation;
pub mod match_info;

pub use crate::dep::v0_4_0::antenna::Antenna;
pub use crate::dep::v0_4_0::collection_info::CollectionInfo;
pub use crate::dep::v0_4_0::error_statistics::ErrorStatistics;
pub use crate::dep::v0_4_0::geo_data::GeoData;
pub use crate::dep::v0_4_0::image_creation::ImageCreation;
pub use crate::dep::v0_4_0::image_data::ImageData;
pub use crate::dep::v0_4_0::pfa::PFA;
pub use crate::dep::v0_4_0::position::Position;
pub use crate::dep::v0_4_0::radar_collection::RadarCollection;
pub use crate::dep::v0_4_0::radiometric::Radiometric;
pub use crate::dep::v0_4_0::scpcoa::SCPCOA;
pub use crate::dep::v0_4_0::timeline::Timeline;
pub use crate::dep::v0_4_0::{
    Coef1D, Coef2D, IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Parameter, Poly1D, Poly2D, RowCol,
    XyzPoly, CMPLX, LL, LLH, XYZ,
};
pub use grid::Grid;
pub use image_formation::{ImageFormation, RGAZCOMP, RMA};
pub use match_info::MatchInfo;

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
    pub scpcoa: SCPCOA,
    #[serde(rename = "Radiometric")]
    pub radiometric: Option<Radiometric>,
    #[serde(rename = "Antenna")]
    pub antenna: Option<Antenna>,
    #[serde(rename = "ErrorStatistics")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "MatchInfo")]
    pub match_info: Option<MatchInfo>,
    #[serde(rename = "RGAZCOMP")]
    pub rg_az_comp: Option<RGAZCOMP>,
    #[serde(rename = "PFA")]
    pub pfa: Option<PFA>,
    #[serde(rename = "RMA")]
    pub rma: Option<RMA>,
}
