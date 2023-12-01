use super::{Poly1D, XYZ};
pub use crate::dep::v0_5_0::image_formation::{
    AzAutofocus, Distortion, ImageBeamComp, ImageFormAlgo, Processing, RMAlgoType, RMAlgoTypeEnum,
    RcvChanProc, RgAutofocus, RgAutofocusEnum, STBeamComp, TxFrequencyProc, INCA,
};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {
    #[serde(rename = "RcvChanProc")]
    pub rcv_chan_proc: RcvChanProc,
    #[serde(rename = "TxRcvPolarizationProc")]
    pub tx_rcv_polarization_proc: TxRcvPolarizationProc,
    #[serde(rename = "TStartProc")]
    pub t_start_proc: f64,
    #[serde(rename = "TEndProc")]
    pub t_end_proc: f64,
    #[serde(rename = "TxFrequencyProc")]
    pub tx_frequency_proc: TxFrequencyProc,
    #[serde(rename = "SegmentIdentifier")]
    pub segment_identifier: Option<String>,
    #[serde(rename = "ImageFormAlgo")]
    pub image_form_algo: ImageFormAlgo,
    #[serde(rename = "STBeamComp")]
    pub st_beam_comp: STBeamComp,
    #[serde(rename = "ImageBeamComp")]
    pub image_beam_comp: ImageBeamComp,
    #[serde(rename = "AzAutofocus")]
    pub az_autofocus: AzAutofocus,
    #[serde(rename = "RgAutofocus")]
    pub rg_autofocus: RgAutofocus,
    #[serde(rename = "Processing")]
    pub processing: Option<Vec<Processing>>,
    #[serde(rename = "PolarizationCalibration")]
    pub polarization_calibration: Option<PolCal>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxRcvPolarizationProc {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PolCal {
    #[serde(rename = "DistortCorrectionApplied")]
    pub distort_correction_applied: bool,
    #[serde(rename = "Distortion")]
    pub distortion: Distortion,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzComp {
    #[serde(rename = "AzSF")]
    pub az_sf: f64,
    #[serde(rename = "KazPoly")]
    pub kaz_poly: Poly1D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMA {
    #[serde(rename = "RMAlgoType")]
    pub rm_algo_type: RMAlgoType,
    #[serde(rename = "ImageType")]
    pub image_type: ImageType,
    #[serde(rename = "RMAT")]
    pub rmat: Option<RMAlgo>,
    #[serde(rename = "RMCR")]
    pub rmcr: Option<RMAlgo>,
    #[serde(rename = "INCA")]
    pub inca: Option<INCA>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageType {
    #[serde(rename = "$text")]
    pub value: ImageTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageTypeEnum {
    RMAT,
    RMCR,
    INCA,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgo {
    #[serde(rename = "PosRef")]
    pub pos_ref: XYZ,
    #[serde(rename = "VelRef")]
    pub vel_ref: XYZ,
    #[serde(rename = "DopConeAngRef")]
    pub dop_cone_ang_ref: f64,
}

#[cfg(test)]
mod tests {
    use super::ImageFormation;
    use quick_xml::de::from_str;

    #[test]
    fn test_image_formation() {
        let xml_str = r#"<ImageFormation><RcvChanProc><NumChanProc>1
            </NumChanProc><ChanIndex>1</ChanIndex></RcvChanProc>
            <TxRcvPolarizationProc>V:V</TxRcvPolarizationProc><TStartProc>0
            </TStartProc><TEndProc>0</TEndProc><TxFrequencyProc><MinProc>0
            </MinProc><MaxProc>0</MaxProc></TxFrequencyProc><ImageFormAlgo>
            PFA</ImageFormAlgo><STBeamComp>NO</STBeamComp><ImageBeamComp>NO
            </ImageBeamComp><AzAutofocus>NO</AzAutofocus><RgAutofocus>NO
            </RgAutofocus><Processing><Type>Processing</Type><Applied>true
            </Applied><Parameter name="param">true</Parameter></Processing>
            </ImageFormation>"#;
        assert!(match from_str::<ImageFormation>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
    // TODO: Test RgAzComp, RMA
}
