//! Common types and metadata definition for SICD Version 0.5.0 [2011-01-12]
use serde::Deserialize;

pub mod grid;
pub mod match_info;
pub mod radiometric;

pub use crate::dep::v0_4_0::antenna::Antenna;
pub use crate::dep::v0_4_0::collection_info::CollectionInfo;
pub use crate::dep::v0_4_0::error_statistics::ErrorStatistics;
pub use crate::dep::v0_4_0::geo_data::GeoData;
pub use crate::dep::v0_4_0::image_creation::ImageCreation;
pub use crate::dep::v0_4_0::image_data::ImageData;
pub use crate::dep::v0_4_0::image_formation::{ImageFormation, RgAzComp, Rma};
pub use crate::dep::v0_4_0::pfa::Pfa;
pub use crate::dep::v0_4_0::position::Position;
pub use crate::dep::v0_4_0::radar_collection::RadarCollection;
pub use crate::dep::v0_4_0::scpcoa::ScpCoa;
pub use crate::dep::v0_4_0::timeline::Timeline;
pub use crate::dep::v0_4_0::{
    Coef1D, Coef2D, IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Parameter, Poly1D, Poly2D, RowCol,
    XyzPoly, CMPLX, LL, LLH, XYZ,
};
use grid::Grid;
use match_info::MatchInfo;
use radiometric::Radiometric;

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
