use super::{Poly2D, XYZ};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {
    #[serde(rename = "ImagePlane")]
    pub image_plane: ImagePlane,
    #[serde(rename = "Type")]
    pub type_grid: GridType,
    #[serde(rename = "TimeCOAPoly")]
    pub time_coa_poly: Poly2D,
    #[serde(rename = "Row")]
    pub row: DirectionParams,
    #[serde(rename = "Col")]
    pub col: DirectionParams,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImagePlane {
    #[serde(rename = "$text")]
    pub value: ImagePlaneEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImagePlaneEnum {
    GROUND,
    SLANT,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GridType {
    #[serde(rename = "$text")]
    pub value: GridTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum GridTypeEnum {
    RGAZIM,
    RGZERO,
    XRGYCR,
    XCTYAT,
    PLANE,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DirectionParams {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "SS")]
    pub ss: f64,
    #[serde(rename = "ImpRespWid")]
    pub imp_resp_wid: f64,
    #[serde(rename = "Sgn")]
    pub sgn: i32, // TODO: Maybe use an actual enum here
    #[serde(rename = "ImpRespBW")]
    pub imp_resp_bw: f64,
    #[serde(rename = "KCtr")]
    pub k_ctr: f64,
    #[serde(rename = "DeltaK1")]
    pub delta_k1: f64,
    #[serde(rename = "DeltaK2")]
    pub delta_k2: f64,
    #[serde(rename = "DeltaKCOAPoly")]
    pub delta_kcoa_poly: Option<Poly2D>,
    #[serde(rename = "WgtType")]
    pub wgt_type: Option<String>,
    #[serde(rename = "WgtFunct")]
    pub wgt_funct: Option<WgtFunct>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtFunct {
    #[serde(rename = "@size")]
    pub size: i32,
    #[serde(rename = "Wgt")]
    pub wgt: Vec<Wgt>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Wgt {
    #[serde(rename = "@index")]
    pub index: i32,
    #[serde(rename = "$value")]
    pub value: f64,
}

mod tests {
    use super::{DirectionParams, Grid, Poly2D};
    use quick_xml::de::from_str;

    #[test]
    fn test_gen_xml_grid() {
        let xml = r#"
  <Grid>
    <ImagePlane class="xs:string">GROUND</ImagePlane>
    <Type class="xs:string">RGAZIM</Type>
    <TimeCOAPoly order1="-1052" order2="-4622">
      <Coef exponent1="-4237" exponent2="2660" class="xs:double">-29866499999999.8</Coef>
      <Coef exponent1="1945" exponent2="-1595" class="xs:double">-4506299999999.76</Coef>
      <Coef exponent1="-4026" exponent2="-4356" class="xs:double">-20750399999999.8</Coef>
      <Coef exponent1="594" exponent2="-4020" class="xs:double">-33942899999999.8</Coef>
      <Coef exponent1="972" exponent2="1840" class="xs:double">-15645799999999.8</Coef>
    </TimeCOAPoly>
    <Row>
      <UVectECF>
        <X class="xs:double">-20727799999999.8</X>
        <Y class="xs:double">23855300000000.2</Y>
        <Z class="xs:double">29372000000000.2</Z>
      </UVectECF>
      <SS class="xs:double">21110500000000.2</SS>
      <ImpRespWid class="xs:double">19682000000000.2</ImpRespWid>
      <Sgn class="xs:int">2424</Sgn>
      <ImpRespBW class="xs:double">20014300000000.2</ImpRespBW>
      <KCtr class="xs:double">-12708799999999.8</KCtr>
      <DeltaK1 class="xs:double">-41078599999999.8</DeltaK1>
      <DeltaK2 class="xs:double">19824000000000.2</DeltaK2>
      <DeltaKCOAPoly order1="-1146" order2="2472">
        <Coef exponent1="-1708" exponent2="-2589" class="xs:double">26725500000000.2</Coef>
      </DeltaKCOAPoly>
      <WgtType class="xs:string">string</WgtType>
    </Row>
    <Col>
      <UVectECF>
        <X class="xs:double">15387100000000.2</X>
        <Y class="xs:double">-47256499999999.8</Y>
        <Z class="xs:double">40079100000000.2</Z>
      </UVectECF>
      <SS class="xs:double">24383100000000.2</SS>
      <ImpRespWid class="xs:double">-13241899999999.8</ImpRespWid>
      <Sgn class="xs:int">3169</Sgn>
      <ImpRespBW class="xs:double">-3957199999999.77</ImpRespBW>
      <KCtr class="xs:double">-18888599999999.8</KCtr>
      <DeltaK1 class="xs:double">34461900000000.2</DeltaK1>
      <DeltaK2 class="xs:double">41716700000000.2</DeltaK2>
      <WgtType class="xs:string">string</WgtType>
      <WgtFunct size="-1907">
        <Wgt index="527" class="xs:double">-11849799999999.8</Wgt>
        <Wgt index="-3877" class="xs:double">15082400000000.2</Wgt>
      </WgtFunct>
    </Col>
  </Grid>
        "#;
        assert!(match from_str::<Grid>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_time_coa_poly() {
        let xml = r#"
    <TimeCOAPoly order1="-1052" order2="-4622">
      <Coef exponent1="-4237" exponent2="2660" class="xs:double">-29866499999999.8</Coef>
      <Coef exponent1="1945" exponent2="-1595" class="xs:double">-4506299999999.76</Coef>
      <Coef exponent1="-4026" exponent2="-4356" class="xs:double">-20750399999999.8</Coef>
      <Coef exponent1="594" exponent2="-4020" class="xs:double">-33942899999999.8</Coef>
      <Coef exponent1="972" exponent2="1840" class="xs:double">-15645799999999.8</Coef>
    </TimeCOAPoly>
        "#;
        assert!(match from_str::<Poly2D>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_row() {
        let xml = r#"
    <Row>
      <UVectECF>
        <X class="xs:double">-20727799999999.8</X>
        <Y class="xs:double">23855300000000.2</Y>
        <Z class="xs:double">29372000000000.2</Z>
      </UVectECF>
      <SS class="xs:double">21110500000000.2</SS>
      <ImpRespWid class="xs:double">19682000000000.2</ImpRespWid>
      <Sgn class="xs:int">2424</Sgn>
      <ImpRespBW class="xs:double">20014300000000.2</ImpRespBW>
      <KCtr class="xs:double">-12708799999999.8</KCtr>
      <DeltaK1 class="xs:double">-41078599999999.8</DeltaK1>
      <DeltaK2 class="xs:double">19824000000000.2</DeltaK2>
      <DeltaKCOAPoly order1="-1146" order2="2472">
        <Coef exponent1="-1708" exponent2="-2589" class="xs:double">26725500000000.2</Coef>
      </DeltaKCOAPoly>
      <WgtType class="xs:string">string</WgtType>
    </Row>
        "#;
        assert!(match from_str::<DirectionParams>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    // #[test]
    // fn test_gen_xml_() {
    //     let xml = r#"
    //     "#;
    //     assert!(match from_str::<>(xml) {
    //         Ok(_) => true,
    //         Err(err) => {dbg!(err);false},
    //     });
    // }
}
