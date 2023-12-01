use super::{Poly1D, Poly2D, XYZ};
pub use crate::dep::v0_4_0::image_formation::{
    AzAutofocus, Distortion, ImageBeamComp, ImageFormAlgo, ImageFormation, ImageType, Processing,
    RMAlgoType, RMAlgoTypeEnum, RcvChanProc, RgAutofocus, RgAutofocusEnum, RGAZCOMP, STBeamComp,
    TxFrequencyProc, INCA,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Rma {
    #[serde(rename = "RMAlgoType")]
    pub rm_algo_type: RMAlgoType,
    #[serde(rename = "ImageType")]
    pub image_type: ImageType,
    #[serde(rename = "RMAT")]
    pub rmat: Option<RMAlgo>,
    #[serde(rename = "INCA")]
    pub inca: Option<INCA>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgo {
    #[serde(rename = "RefTime")]
    pub ref_time: f64,
    #[serde(rename = "PosRef")]
    pub pos_ref: XYZ,
    #[serde(rename = "UnitVelRef")]
    pub unit_vel_ref: XYZ,
    #[serde(rename = "DistRLPoly")]
    pub dist_rl_poly: Poly1D,
    #[serde(rename = "CosDCACOAPoly")]
    pub cos_dcacoa_poly: Poly2D,
    #[serde(rename = "Kx1")]
    pub kx1: f64,
    #[serde(rename = "Kx2")]
    pub kx2: f64,
    #[serde(rename = "Ky1")]
    pub ky1: f64,
    #[serde(rename = "Ky2")]
    pub ky2: f64,
}
