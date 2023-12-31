//! Common types and metadata definition for SICD Version 0.4.0 [2010-02-12]
use ndarray::{Array1, Array2};
use serde::Deserialize;

pub mod antenna;
pub mod collection_info;
pub mod error_statistics;
pub mod geo_data;
pub mod grid;
pub mod image_creation;
pub mod image_data;
pub mod image_formation;
pub mod match_info;
pub mod pfa;
pub mod position;
pub mod radar_collection;
pub mod radiometric;
pub mod scpcoa;
pub mod timeline;

pub use antenna::Antenna;
pub use collection_info::CollectionInfo;
pub use error_statistics::ErrorStatistics;
pub use geo_data::GeoData;
pub use grid::Grid;
pub use image_creation::ImageCreation;
pub use image_data::ImageData;
pub use image_formation::{ImageFormation, RGAZCOMP, RMA};
pub use match_info::MatchInfo;
pub use pfa::PFA;
pub use position::Position;
pub use radar_collection::RadarCollection;
pub use radiometric::Radiometric;
pub use scpcoa::SCPCOA;
pub use timeline::Timeline;

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

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RowCol {
    #[serde(rename = "Row")]
    pub row: i64,
    #[serde(rename = "Col")]
    pub col: i64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxRowCol {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "Row")]
    pub row: i64,
    #[serde(rename = "Col")]
    pub col: i64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CMPLX {
    #[serde(rename = "Real")]
    pub real: f64,
    #[serde(rename = "Imag")]
    pub imag: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XYZ {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LLH {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
    #[serde(rename = "HAE")]
    pub hae: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxLLH {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
    #[serde(rename = "HAE")]
    pub hae: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LL {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxLL {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1D {
    #[serde(rename = "@exponent1")]
    pub exponent1: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1D {
    #[serde(rename = "@order1")]
    pub order1: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef1D>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2D {
    #[serde(rename = "@exponent1")]
    pub exponent1: usize,
    #[serde(rename = "@exponent2")]
    pub exponent2: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2D {
    #[serde(rename = "@order1")]
    pub order1: usize,
    #[serde(rename = "@order2")]
    pub order2: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef2D>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    #[serde(rename = "X")]
    pub x: Poly1D,
    #[serde(rename = "Y")]
    pub y: Poly1D,
    #[serde(rename = "Z")]
    pub z: Poly1D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxXyzPoly {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "X")]
    pub x: Poly1D,
    #[serde(rename = "Y")]
    pub y: Poly1D,
    #[serde(rename = "Z")]
    pub z: Poly1D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Parameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Poly1D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array1<f64> {
        let mut poly = Array1::zeros(self.order1 + 1);
        for coef in &self.coefs {
            let term: usize = coef.exponent1;
            poly[term] = coef.value;
        }
        poly
    }

    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            let exp = i32::try_from(coef.exponent1).unwrap();
            res += coef.value * x.powi(exp);
        }
        res
    }
}

impl Poly2D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array2<f64> {
        let mut poly = Array2::zeros((self.order1 + 1, self.order2 + 1));
        for coef in &self.coefs {
            let term1 = coef.exponent1;
            let term2 = coef.exponent2;
            poly[[term1, term2]] = coef.value;
        }
        poly
    }
    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            res += coef.value
                * x.powi(i32::try_from(coef.exponent1).unwrap())
                * y.powi(i32::try_from(coef.exponent2).unwrap())
        }
        res
    }
}
impl XyzPoly {
    pub fn eval(&self, t: f64) -> Vec<f64> {
        let x_pos = self.x.eval(t);
        let y_pos = self.y.eval(t);
        let z_pos = self.z.eval(t);
        vec![x_pos, y_pos, z_pos]
    }
}
