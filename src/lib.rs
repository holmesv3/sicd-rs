//! Sensor Independent Complex Data support
//!
//! The primary interface for general sicd reading is `read_sicd`.
//!
//! It is a future goal to have functions for each version, but for now a single
//! function call and `match` statement are used.
use memmap2::Mmap;
use ndarray::{azip, par_azip, Array2, ArrayView2};
use quick_xml::DeError;
use std::fs::File;
use std::path::Path;
use std::slice::from_raw_parts;
use std::str::{from_utf8, FromStr, Utf8Error};
use zerocopy::{BE, F32};

use num_complex::{Complex, Complex32};
use quick_xml::de::from_str;
use serde::Deserialize;
use thiserror::Error;

use nitf_rs::{Nitf, NitfError};

pub mod dep;
pub mod v1_3_0;

/// Construct a [Sicd] object from a file `path`.
///
/// This is specifically for cases where the version of the Sicd is not known
/// and makes use of several `enums` to parse the data.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// use sicd_rs::SicdVersion;
///
/// let sicd_path = Path::new("../example.nitf");
/// let sicd = sicd_rs::read_sicd(sicd_path).unwrap();
/// // Then use convenience methods provided by SicdMeta enum, or match off of version
/// let meta = sicd.meta.get_v1_meta();
///
/// ```
///
pub fn read_sicd(path: &Path) -> Result<Sicd, SicdError> {
    let file = File::open(path)?;
    Sicd::from_file(file)
}

#[derive(Error, Debug)]
pub enum SicdError {
    /// "unknown sicd version {}"
    #[error("unknown sicd version {0}")]
    VersionError(String),
    /// "metadata for version {} is not implemented"
    #[error("metadata for version {0} is not implemented")]
    Unimpl(String),
    // Wrappers for built in errors
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    NitfError(#[from] NitfError),
    #[error(transparent)]
    UTF8(#[from] Utf8Error),
    #[error(transparent)]
    DESER(#[from] DeError),
}

/// SICD file structure
///
// TODO: Implement printing (Debug, Display?)
pub struct Sicd<'a> {
    /// Nitf file object and associated metadata
    pub nitf: Nitf,
    /// Parsed SICD xml metadata
    pub meta: SicdMeta,
    /// SICD Version
    pub version: SicdVersion,
    /// Image data from Nitf Image segements
    pub image_data: Vec<ImageData<'a>>,
}

/// Image data structure. Currently only implements Complex<F32<BE>> (e.g. big-endian complex float) data type
#[derive(Debug)]
pub struct ImageData<'a> {
    /// Raw byte array
    pub array: ArrayView2<'a, Complex<F32<BE>>>,
    /// Need to hold onto this to access data
    _mmap: Mmap,
}

impl<'a> ImageData<'a> {
    fn initialize(mmap: Mmap, n_rows: usize, n_cols: usize) -> Self {
        let byte_slice_len = mmap.len();
        let new_size = byte_slice_len / std::mem::size_of::<Complex<F32<BE>>>();
        dbg!(byte_slice_len);
        dbg!(new_size);
        let f32_ptr = mmap.as_ptr() as *const Complex<F32<BE>>;
        let float_slice = unsafe { from_raw_parts(f32_ptr, new_size) };
        let array = ArrayView2::from_shape((n_rows, n_cols), float_slice).unwrap();
        Self { array, _mmap: mmap }
    }
}
pub trait ToNative {
    /// Performs allocation of a native-aligned Complex32 array
    fn to_native(&self) -> Array2<Complex32>;
    /// Performs allocation of a native-aligned Complex32 array using `rayon`
    fn par_to_native(&self) -> Array2<Complex32>;
}

impl<'a> ToNative for ArrayView2<'a, Complex<F32<BE>>> {
    fn to_native(&self) -> Array2<Complex32> {
        let mut out = Array2::from_elem((self.nrows(), self.ncols()), Complex32::default());
        azip!((be in self, ne in &mut out) {
           ne.re = be.re.get();
           ne.im = be.im.get();
        });
        out
    }

    fn par_to_native(&self) -> Array2<Complex32> {
        let mut out = Array2::from_elem((self.nrows(), self.ncols()), Complex32::default());
        par_azip!((be in self, ne in &mut out) {
           ne.re = be.re.get();
           ne.im = be.im.get();
        });
        out
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum SicdVersion {
    V0_3_1,
    V0_4_0,
    V0_4_1,
    V0_5_0,
    V1_0_0,
    V1_0_1,
    V1_1_0,
    V1_2_0,
    V1_2_1,
    V1_3_0,
}

impl FromStr for SicdVersion {
    type Err = SicdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("urn:SICD:").collect::<String>().as_str() {
            "0.3.1" => Ok(SicdVersion::V0_3_1),
            "0.4.0" => Ok(SicdVersion::V0_4_0),
            "0.4.1" => Ok(SicdVersion::V0_4_1),
            "0.5.0" => Ok(SicdVersion::V0_5_0),
            "1.0.0" => Ok(SicdVersion::V1_0_0),
            "1.0.1" => Ok(SicdVersion::V1_0_1),
            "1.1.0" => Ok(SicdVersion::V1_1_0),
            "1.2.0" => Ok(SicdVersion::V1_2_0),
            "1.2.1" => Ok(SicdVersion::V1_2_1),
            "1.3.0" => Ok(SicdVersion::V1_3_0),
            _ => Err(SicdError::VersionError(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum SicdMeta {
    V0_3_1, // Not implemented
    V0_4_0(dep::v0_4_0::SicdMeta),
    V0_4_1, // Not implemented
    V0_5_0(dep::v0_5_0::SicdMeta),
    V1(v1_3_0::SicdMeta),
}

impl SicdMeta {
    pub fn get_v0_3_1_meta(self) -> SicdError {
        SicdError::Unimpl("0.3.1".to_string())
    }
    pub fn get_v0_4_0_meta(self) -> Option<dep::v0_4_0::SicdMeta> {
        match self {
            Self::V0_4_0(meta) => Some(meta),
            _ => None,
        }
    }
    pub fn get_v0_4_1_meta(self) -> SicdError {
        SicdError::Unimpl("0.4.1".to_string())
    }
    pub fn get_v0_5_0_meta(self) -> Option<dep::v0_5_0::SicdMeta> {
        match self {
            Self::V0_5_0(meta) => Some(meta),
            _ => None,
        }
    }
    pub fn get_v1_meta(self) -> Option<v1_3_0::SicdMeta> {
        match self {
            Self::V1(meta) => Some(meta),
            _ => None,
        }
    }
}
impl<'a> Sicd<'a> {
    pub fn from_file(mut file: File) -> Result<Self, SicdError> {
        let nitf = Nitf::from_reader(&mut file)?;
        let dex_data = nitf.data_extension_segments[0].get_data_map(&mut file)?;
        let sicd_str = from_utf8(&dex_data[..])?;
        let (version, meta) = parse_sicd(sicd_str)?;

        let image_data: Vec<_> = nitf
            .image_segments
            .iter()
            .map(|seg| {
                ImageData::initialize(
                    seg.get_data_map(&mut file).unwrap(),
                    seg.header.nrows.val as usize,
                    seg.header.ncols.val as usize,
                )
            })
            .collect();

        Ok(Self {
            nitf,
            meta,
            version,
            image_data,
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VersionGetter {
    #[serde(rename = "@xmlns")]
    pub version: String,
}

fn parse_sicd(sicd_str: &str) -> Result<(SicdVersion, SicdMeta), SicdError> {
    // This feels bad
    let tmp: VersionGetter = from_str(sicd_str)?;
    let sicd_version = SicdVersion::from_str(&tmp.version)?;
    use SicdError::Unimpl;
    match sicd_version {
        SicdVersion::V0_3_1 => Err(Unimpl("V0_3_1".to_string())),
        SicdVersion::V0_4_0 => Ok((SicdVersion::V0_4_0, SicdMeta::V0_4_0(from_str(sicd_str)?))),
        SicdVersion::V0_4_1 => Err(Unimpl("V0_4_1".to_string())),
        SicdVersion::V0_5_0 => Ok((SicdVersion::V0_5_0, SicdMeta::V0_5_0(from_str(sicd_str)?))),
        // Don't need to worry about anything else, all versions past 1.0 are backwards compatible
        other => Ok((other, SicdMeta::V1(from_str(sicd_str)?))),
    }
}
