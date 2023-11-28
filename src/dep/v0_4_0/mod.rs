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

use crate::to_usize;
pub use antenna::Antenna;
pub use collection_info::CollectionInfo;
pub use error_statistics::ErrorStatistics;
pub use geo_data::GeoData;
pub use grid::Grid;
pub use image_creation::ImageCreation;
pub use image_data::ImageData;
pub use image_formation::{ImageFormation, RgAzComp, Rma};
pub use match_info::MatchInfo;
pub use pfa::Pfa;
pub use position::Position;
pub use radar_collection::RadarCollection;
pub use radiometric::Radiometric;
pub use scpcoa::ScpCoa;
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
pub struct RowCol {
    #[serde(rename = "Row")]
    pub row: i64,
    #[serde(rename = "Col")]
    pub col: i64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxRowCol {
    #[serde(rename = "@index")]
    pub index: i32,
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
    pub index: i32,
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
    pub index: i32,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1D {
    #[serde(rename = "@exponent1")]
    pub exponent1: i32,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1D {
    #[serde(rename = "@order1")]
    pub order1: i32,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef1D>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2D {
    #[serde(rename = "@exponent1")]
    pub exponent1: i32,
    #[serde(rename = "@exponent2")]
    pub exponent2: i32,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2D {
    #[serde(rename = "@order1")]
    pub order1: i32,
    #[serde(rename = "@order2")]
    pub order2: i32,
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
    pub index: i32,
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
        let mut poly = Array1::zeros(to_usize(self.order1) + 1);
        for coef in &self.coefs {
            let term: usize = to_usize(coef.exponent1);
            poly[term] = coef.value;
        }
        poly
    }

    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            res += coef.value * x.powi(coef.exponent1)
        }
        res
    }
}

impl Poly2D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array2<f64> {
        let mut poly = Array2::zeros((to_usize(self.order1) + 1, to_usize(self.order2) + 1));
        for coef in &self.coefs {
            let term1 = to_usize(coef.exponent1);
            let term2 = to_usize(coef.exponent2);
            poly[[term1, term2]] = coef.value;
        }
        poly
    }
    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            res += coef.value * x.powi(coef.exponent1) * y.powi(coef.exponent2)
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

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::{CollectionInfo, GeoData, Grid, ImageCreation, ImageData, SicdMeta};

    #[test]
    fn test_gen_xml() {
        let xml = r#"
        <?xml version="1.0" encoding="utf-8"?>
<!-- Created with Liquid Technologies Online Tools 1.0 (https://www.liquid-technologies.com) -->
<SICD xmlns="urn:SICD:0.4.0" xsi:schemaLocation="urn:SICD:0.4.0 schema.xsd" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <CollectionInfo>
    <CollectorName class="xs:string">string</CollectorName>
    <IlluminatorName class="xs:string">string</IlluminatorName>
    <CoreName class="xs:string">string</CoreName>
    <CollectType class="xs:string">MONOSTATIC</CollectType>
    <RadarMode>
      <ModeType class="xs:string">SPOTLIGHT</ModeType>
    </RadarMode>
    <Classification class="xs:string">string</Classification>
    <CountryCode class="xs:string">string</CountryCode>
    <CountryCode class="xs:string">string</CountryCode>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
  </CollectionInfo>
  <ImageCreation>
    <Application class="xs:string">string</Application>
    <DateTime class="xs:dateTime">1984-08-28T05:27:48.21</DateTime>
    <Site class="xs:string">string</Site>
    <Profile class="xs:string">string</Profile>
  </ImageCreation>
  <ImageData>
    <PixelType class="xs:string">RE32F_IM32F</PixelType>
    <AmpTable size="-2707">
      <Amplitude index="3686" class="xs:double">-7078599999999.76</Amplitude>
      <Amplitude index="1025" class="xs:double">21028400000000.2</Amplitude>
    </AmpTable>
    <NumRows class="xs:int">2816</NumRows>
    <NumCols class="xs:int">1313</NumCols>
    <FirstRow class="xs:int">-2905</FirstRow>
    <FirstCol class="xs:int">-3467</FirstCol>
    <FullImage>
      <NumRows class="xs:int">135</NumRows>
      <NumCols class="xs:int">-1971</NumCols>
    </FullImage>
    <SCPPixel>
      <Row class="xs:int">1978</Row>
      <Col class="xs:int">1467</Col>
    </SCPPixel>
    <ValidData size="545">
      <Vertex index="-1561">
        <Row class="xs:int">-3455</Row>
        <Col class="xs:int">2979</Col>
      </Vertex>
      <Vertex index="-4756">
        <Row class="xs:int">-108</Row>
        <Col class="xs:int">3828</Col>
      </Vertex>
      <Vertex index="-1004">
        <Row class="xs:int">1031</Row>
        <Col class="xs:int">2580</Col>
      </Vertex>
    </ValidData>
  </ImageData>
  <GeoData>
    <EarthModel class="xs:string">WGS_84</EarthModel>
    <SCP>
      <ECF>
        <X class="xs:double">-32922499999999.8</X>
        <Y class="xs:double">45571300000000.2</Y>
        <Z class="xs:double">-28286899999999.8</Z>
      </ECF>
      <LLH>
        <Lat class="xs:double">24698700000000.2</Lat>
        <Lon class="xs:double">-6815199999999.76</Lon>
        <HAE class="xs:double">15901200000000.2</HAE>
      </LLH>
    </SCP>
    <ImageCorners>
      <ICP index="3:LRLC">
        <Lat class="xs:double">-16505799999999.8</Lat>
        <Lon class="xs:double">15760600000000.2</Lon>
      </ICP>
      <ICP index="2:FRLC">
        <Lat class="xs:double">-30138199999999.8</Lat>
        <Lon class="xs:double">18215300000000.2</Lon>
      </ICP>
      <ICP index="1:FRFC">
        <Lat class="xs:double">7613900000000.24</Lat>
        <Lon class="xs:double">38986600000000.2</Lon>
      </ICP>
      <ICP index="4:LRFC">
        <Lat class="xs:double">19679800000000.2</Lat>
        <Lon class="xs:double">19660400000000.2</Lon>
      </ICP>
    </ImageCorners>
    <ValidData size="-1889">
      <Vertex index="-3007">
        <Lat class="xs:double">15413400000000.2</Lat>
        <Lon class="xs:double">18658000000000.2</Lon>
      </Vertex>
      <Vertex index="-3956">
        <Lat class="xs:double">-45829899999999.8</Lat>
        <Lon class="xs:double">1316900000000.23</Lon>
      </Vertex>
      <Vertex index="4940">
        <Lat class="xs:double">-48148799999999.8</Lat>
        <Lon class="xs:double">30975500000000.2</Lon>
      </Vertex>
    </ValidData>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
  </GeoData>
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
  <Timeline>
    <CollectStart class="xs:dateTime">1992-02-22T01:30:10.50</CollectStart>
    <CollectDuration class="xs:double">-8831099999999.77</CollectDuration>
    <IPP size="-1044">
      <Set index="2806">
        <TStart class="xs:double">-28097199999999.8</TStart>
        <TEnd class="xs:double">22082200000000.2</TEnd>
        <IPPStart class="xs:int">-2173</IPPStart>
        <IPPEnd class="xs:int">-3916</IPPEnd>
        <IPPPoly order1="3011">
          <Coef exponent1="3102" class="xs:double">13155900000000.2</Coef>
        </IPPPoly>
      </Set>
    </IPP>
  </Timeline>
  <Position>
    <ARPPoly>
      <X order1="2521">
        <Coef exponent1="-1577" class="xs:double">17928400000000.2</Coef>
      </X>
      <Y order1="-1905">
        <Coef exponent1="4137" class="xs:double">3104600000000.23</Coef>
      </Y>
      <Z order1="2720">
        <Coef exponent1="-682" class="xs:double">3795500000000.23</Coef>
      </Z>
    </ARPPoly>
    <GRPPoly>
      <X order1="-4748">
        <Coef exponent1="-2388" class="xs:double">-29925299999999.8</Coef>
      </X>
      <Y order1="1963">
        <Coef exponent1="2071" class="xs:double">6272200000000.24</Coef>
      </Y>
      <Z order1="2262">
        <Coef exponent1="2538" class="xs:double">-47250899999999.8</Coef>
      </Z>
    </GRPPoly>
    <TxAPCPoly>
      <X order1="4732">
        <Coef exponent1="-1273" class="xs:double">-22993999999999.8</Coef>
      </X>
      <Y order1="761">
        <Coef exponent1="3911" class="xs:double">-36545199999999.8</Coef>
      </Y>
      <Z order1="2531">
        <Coef exponent1="2650" class="xs:double">-45678499999999.8</Coef>
      </Z>
    </TxAPCPoly>
    <RcvAPC size="-3585">
      <RcvAPCPoly index="3103">
        <X order1="-1490">
          <Coef exponent1="4983" class="xs:double">-20544099999999.8</Coef>
        </X>
        <Y order1="-957">
          <Coef exponent1="1492" class="xs:double">29301400000000.2</Coef>
        </Y>
        <Z order1="-4553">
          <Coef exponent1="-3189" class="xs:double">-25040099999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-1520">
        <X order1="726">
          <Coef exponent1="-2846" class="xs:double">29194600000000.2</Coef>
        </X>
        <Y order1="2106">
          <Coef exponent1="-3120" class="xs:double">23720400000000.2</Coef>
        </Y>
        <Z order1="2039">
          <Coef exponent1="804" class="xs:double">-3619899999999.77</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="778">
        <X order1="-132">
          <Coef exponent1="-3961" class="xs:double">21776700000000.2</Coef>
        </X>
        <Y order1="2203">
          <Coef exponent1="4604" class="xs:double">10397500000000.2</Coef>
        </Y>
        <Z order1="-3819">
          <Coef exponent1="-1009" class="xs:double">10143900000000.2</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-1596">
        <X order1="-1398">
          <Coef exponent1="4925" class="xs:double">41201500000000.2</Coef>
        </X>
        <Y order1="-2004">
          <Coef exponent1="-893" class="xs:double">21733600000000.2</Coef>
        </Y>
        <Z order1="-2057">
          <Coef exponent1="-3389" class="xs:double">2158100000000.23</Coef>
        </Z>
      </RcvAPCPoly>
    </RcvAPC>
  </Position>
  <RadarCollection>
    <RefFreqIndex class="xs:int">2854</RefFreqIndex>
    <TxFrequency>
      <Min class="xs:double">-12914799999999.8</Min>
      <Max class="xs:double">8369300000000.24</Max>
    </TxFrequency>
    <TxPolarization class="xs:string">V</TxPolarization>
    <PolarizationHVAnglePoly order1="4789">
      <Coef exponent1="-3384" class="xs:double">-12397599999999.8</Coef>
      <Coef exponent1="-3204" class="xs:double">-13827599999999.8</Coef>
    </PolarizationHVAnglePoly>
    <TxSequence size="-3081">
      <TxStep index="-3405" />
      <TxStep index="4322" />
      <TxStep index="2250" />
      <TxStep index="-1073" />
    </TxSequence>
    <Waveform size="-915">
      <WFParameters index="2159" />
      <WFParameters index="-900" />
      <WFParameters index="-1382" />
    </Waveform>
    <RcvChannels size="-4033">
      <ChanParameters index="4393" />
      <ChanParameters index="2937" />
      <ChanParameters index="1873" />
    </RcvChannels>
    <Area>
      <Corner>
        <ACP index="3">
          <Lat class="xs:double">-17831199999999.8</Lat>
          <Lon class="xs:double">27875500000000.2</Lon>
          <HAE class="xs:double">27443900000000.2</HAE>
        </ACP>
        <ACP index="1">
          <Lat class="xs:double">-32801299999999.8</Lat>
          <Lon class="xs:double">10124900000000.2</Lon>
          <HAE class="xs:double">32084200000000.2</HAE>
        </ACP>
        <ACP index="1">
          <Lat class="xs:double">-20380999999999.8</Lat>
          <Lon class="xs:double">45135100000000.2</Lon>
          <HAE class="xs:double">34787500000000.2</HAE>
        </ACP>
        <ACP index="3">
          <Lat class="xs:double">1046300000000.23</Lat>
          <Lon class="xs:double">-22745599999999.8</Lon>
          <HAE class="xs:double">29989600000000.2</HAE>
        </ACP>
      </Corner>
    </Area>
    <Parameter name="string" class="xs:string">string</Parameter>
  </RadarCollection>
  <ImageFormation>
    <SegmentIdentifier class="xs:string">string</SegmentIdentifier>
    <RcvChanProc>
      <NumChanProc class="xs:int">-3489</NumChanProc>
      <PRFScaleFactor class="xs:double">18289800000000.2</PRFScaleFactor>
      <ChanIndex class="xs:int">-4141</ChanIndex>
      <ChanIndex class="xs:int">-1389</ChanIndex>
      <ChanIndex class="xs:int">-4829</ChanIndex>
    </RcvChanProc>
    <TxRcvPolarizationProc class="xs:string">string</TxRcvPolarizationProc>
    <ImageFormAlgo class="xs:string">PFA</ImageFormAlgo>
    <TStartProc class="xs:double">38286200000000.2</TStartProc>
    <TEndProc class="xs:double">-25466599999999.8</TEndProc>
    <TxFrequencyProc>
      <MinProc class="xs:double">-3461499999999.77</MinProc>
      <MaxProc class="xs:double">41672900000000.2</MaxProc>
    </TxFrequencyProc>
    <STBeamComp class="xs:string">NO</STBeamComp>
    <ImageBeamComp class="xs:string">NO</ImageBeamComp>
    <AzAutofocus class="xs:string">NO</AzAutofocus>
    <RgAutofocus class="xs:string">NO</RgAutofocus>
    <Processing>
      <Type class="xs:string">string</Type>
      <Applied class="xs:boolean">false</Applied>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Processing>
    <Processing>
      <Type class="xs:string">string</Type>
      <Applied class="xs:boolean">1</Applied>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Processing>
    <Processing>
      <Type class="xs:string">string</Type>
      <Applied class="xs:boolean">1</Applied>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Processing>
    <Processing>
      <Type class="xs:string">string</Type>
      <Applied class="xs:boolean">0</Applied>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Processing>
    <PolarizationCalibration>
      <HVAngleCompApplied class="xs:boolean">false</HVAngleCompApplied>
      <DistortionCorrectionApplied class="xs:boolean">1</DistortionCorrectionApplied>
      <Distortion>
        <A class="xs:double">-11304699999999.8</A>
        <F1>
          <Real class="xs:double">-8230599999999.76</Real>
          <Imag class="xs:double">-35904399999999.8</Imag>
        </F1>
        <Q1>
          <Real class="xs:double">-47321599999999.8</Real>
          <Imag class="xs:double">40514300000000.2</Imag>
        </Q1>
        <Q2>
          <Real class="xs:double">17844600000000.2</Real>
          <Imag class="xs:double">46592900000000.2</Imag>
        </Q2>
        <F2>
          <Real class="xs:double">404100000000.235</Real>
          <Imag class="xs:double">-29214099999999.8</Imag>
        </F2>
        <Q3>
          <Real class="xs:double">-37470399999999.8</Real>
          <Imag class="xs:double">11453400000000.2</Imag>
        </Q3>
        <Q4>
          <Real class="xs:double">3191400000000.23</Real>
          <Imag class="xs:double">-24512099999999.8</Imag>
        </Q4>
      </Distortion>
    </PolarizationCalibration>
  </ImageFormation>
  <SCPCOA>
    <SCPTime class="xs:double">-736299999999.765</SCPTime>
    <ARPPos>
      <X class="xs:double">31554600000000.2</X>
      <Y class="xs:double">24330900000000.2</Y>
      <Z class="xs:double">27714800000000.2</Z>
    </ARPPos>
    <ARPVel>
      <X class="xs:double">24167100000000.2</X>
      <Y class="xs:double">26065200000000.2</Y>
      <Z class="xs:double">41720100000000.2</Z>
    </ARPVel>
    <ARPAcc>
      <X class="xs:double">2190900000000.23</X>
      <Y class="xs:double">29455200000000.2</Y>
      <Z class="xs:double">-18975699999999.8</Z>
    </ARPAcc>
    <SideOfTrack class="xs:string">L</SideOfTrack>
    <SlantRange class="xs:double">16624300000000.2</SlantRange>
    <GroundRange class="xs:double">-19278599999999.8</GroundRange>
    <DopplerConeAng class="xs:double">45278000000000.2</DopplerConeAng>
    <GrazeAng class="xs:double">46397500000000.2</GrazeAng>
    <IncidenceAng class="xs:double">22504200000000.2</IncidenceAng>
    <TwistAng class="xs:double">34107000000000.2</TwistAng>
    <SlopeAng class="xs:double">38687600000000.2</SlopeAng>
  </SCPCOA>
  <Radiometric>
    <NoisePoly order1="-2695" order2="438">
      <Coef exponent1="390" exponent2="3784" class="xs:double">-29406999999999.8</Coef>
    </NoisePoly>
    <RCSSFPoly order1="1048" order2="-170">
      <Coef exponent1="-4450" exponent2="306" class="xs:double">47398200000000.2</Coef>
      <Coef exponent1="-4581" exponent2="-103" class="xs:double">-42293499999999.8</Coef>
      <Coef exponent1="770" exponent2="-301" class="xs:double">-533399999999.765</Coef>
      <Coef exponent1="-4220" exponent2="4741" class="xs:double">27769500000000.2</Coef>
      <Coef exponent1="-93" exponent2="-4644" class="xs:double">-25686799999999.8</Coef>
    </RCSSFPoly>
    <BetaZeroSFPoly order1="-2271" order2="-4405">
      <Coef exponent1="-3967" exponent2="-3175" class="xs:double">35513600000000.2</Coef>
      <Coef exponent1="2665" exponent2="1106" class="xs:double">-45909299999999.8</Coef>
      <Coef exponent1="-3728" exponent2="-1273" class="xs:double">-39871699999999.8</Coef>
    </BetaZeroSFPoly>
    <SigmaZeroSFPoly order1="-1521" order2="-2459">
      <Coef exponent1="1700" exponent2="-3897" class="xs:double">36708700000000.2</Coef>
      <Coef exponent1="-3455" exponent2="-1029" class="xs:double">-31970099999999.8</Coef>
    </SigmaZeroSFPoly>
    <SigmaZeroSFIncidenceMap class="xs:string">APPLIED</SigmaZeroSFIncidenceMap>
    <GammaZeroSFPoly order1="-3826" order2="-3475">
      <Coef exponent1="2063" exponent2="34" class="xs:double">13524800000000.2</Coef>
      <Coef exponent1="-3702" exponent2="-2003" class="xs:double">4528500000000.24</Coef>
    </GammaZeroSFPoly>
    <GammaZeroSFIncidenceMap class="xs:string">APPLIED</GammaZeroSFIncidenceMap>
  </Radiometric>
  <Antenna>
    <Tx>
      <XAxisPoly>
        <X order1="-2958">
          <Coef exponent1="2301" class="xs:double">21995200000000.2</Coef>
        </X>
        <Y order1="-3495">
          <Coef exponent1="-2910" class="xs:double">-38157099999999.8</Coef>
        </Y>
        <Z order1="884">
          <Coef exponent1="-4350" class="xs:double">20266900000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="1238">
          <Coef exponent1="529" class="xs:double">30265900000000.2</Coef>
        </X>
        <Y order1="-3897">
          <Coef exponent1="-652" class="xs:double">15959400000000.2</Coef>
        </Y>
        <Z order1="4483">
          <Coef exponent1="1737" class="xs:double">-2470299999999.77</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero class="xs:double">34973700000000.2</FreqZero>
      <EB>
        <DCXPoly order1="24">
          <Coef exponent1="3829" class="xs:double">-37423999999999.8</Coef>
        </DCXPoly>
        <DCYPoly order1="2127">
          <Coef exponent1="-4893" class="xs:double">-29384899999999.8</Coef>
        </DCYPoly>
      </EB>
      <MLFreqDilation class="xs:boolean">0</MLFreqDilation>
    </Tx>
    <Rcv>
      <XAxisPoly>
        <X order1="-1048">
          <Coef exponent1="-4148" class="xs:double">-11280099999999.8</Coef>
        </X>
        <Y order1="-2197">
          <Coef exponent1="2788" class="xs:double">-1406699999999.77</Coef>
        </Y>
        <Z order1="-4493">
          <Coef exponent1="-859" class="xs:double">-28578099999999.8</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="1489">
          <Coef exponent1="4044" class="xs:double">18363400000000.2</Coef>
        </X>
        <Y order1="-1139">
          <Coef exponent1="-757" class="xs:double">47191900000000.2</Coef>
        </Y>
        <Z order1="3043">
          <Coef exponent1="-3312" class="xs:double">26775800000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero class="xs:double">-17450999999999.8</FreqZero>
      <HPBW>
        <DCX class="xs:double">-16452099999999.8</DCX>
        <DCY class="xs:double">-26832599999999.8</DCY>
      </HPBW>
      <Elem>
        <GainPoly order1="-2888" order2="2300">
          <Coef exponent1="4005" exponent2="3782" class="xs:double">38005100000000.2</Coef>
        </GainPoly>
        <PhasePoly order1="-984" order2="-3894">
          <Coef exponent1="1774" exponent2="4511" class="xs:double">30684200000000.2</Coef>
        </PhasePoly>
      </Elem>
      <GainBSPoly order1="-692">
        <Coef exponent1="2510" class="xs:double">14638300000000.2</Coef>
      </GainBSPoly>
      <MLFreqDilation class="xs:boolean">false</MLFreqDilation>
    </Rcv>
    <TwoWay>
      <XAxisPoly>
        <X order1="-480">
          <Coef exponent1="-127" class="xs:double">40378100000000.2</Coef>
        </X>
        <Y order1="-2581">
          <Coef exponent1="-292" class="xs:double">40785600000000.2</Coef>
        </Y>
        <Z order1="-1859">
          <Coef exponent1="-4860" class="xs:double">40717100000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="4240">
          <Coef exponent1="-3680" class="xs:double">-24339899999999.8</Coef>
        </X>
        <Y order1="3078">
          <Coef exponent1="-337" class="xs:double">41528000000000.2</Coef>
        </Y>
        <Z order1="-1185">
          <Coef exponent1="-3195" class="xs:double">3523500000000.23</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero class="xs:double">27359100000000.2</FreqZero>
      <EB>
        <DCXPoly order1="3624">
          <Coef exponent1="-377" class="xs:double">-16774799999999.8</Coef>
        </DCXPoly>
        <DCYPoly order1="-1865">
          <Coef exponent1="4990" class="xs:double">30675900000000.2</Coef>
        </DCYPoly>
      </EB>
      <HPBW>
        <DCX class="xs:double">-23686999999999.8</DCX>
        <DCY class="xs:double">-26925399999999.8</DCY>
      </HPBW>
      <Array>
        <GainPoly order1="-3141" order2="-3979">
          <Coef exponent1="4072" exponent2="4247" class="xs:double">-4031299999999.77</Coef>
        </GainPoly>
        <PhasePoly order1="-212" order2="1092">
          <Coef exponent1="4637" exponent2="4995" class="xs:double">43856700000000.2</Coef>
        </PhasePoly>
      </Array>
      <EBFreqShift class="xs:boolean">false</EBFreqShift>
      <MLFreqDilation class="xs:boolean">false</MLFreqDilation>
    </TwoWay>
  </Antenna>
  <ErrorStatistics>
    <CompositeSCP>
      <RgAzErr>
        <Rg class="xs:double">48933200000000.2</Rg>
        <Az class="xs:double">2467900000000.23</Az>
        <RgAz class="xs:double">26028100000000.2</RgAz>
      </RgAzErr>
    </CompositeSCP>
    <Components>
      <PosVelErr>
        <Frame class="xs:string">ECF</Frame>
        <P1 class="xs:double">-22056099999999.8</P1>
        <P2 class="xs:double">28634300000000.2</P2>
        <P3 class="xs:double">-49374399999999.8</P3>
        <V1 class="xs:double">21247200000000.2</V1>
        <V2 class="xs:double">-41771499999999.8</V2>
        <V3 class="xs:double">49698800000000.2</V3>
      </PosVelErr>
      <IonoError>
        <IonoRgRgRateCC class="xs:double">-38296399999999.8</IonoRgRgRateCC>
      </IonoError>
    </Components>
    <AdditionalParms>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </AdditionalParms>
  </ErrorStatistics>
  <MatchInfo>
    <Collect index="-75">
      <CollectorName class="xs:string">string</CollectorName>
      <CoreName class="xs:string">string</CoreName>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Collect>
    <Collect index="-3701">
      <CollectorName class="xs:string">string</CollectorName>
      <CoreName class="xs:string">string</CoreName>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Collect>
    <Collect index="1307">
      <CollectorName class="xs:string">string</CollectorName>
      <CoreName class="xs:string">string</CoreName>
      <MatchType class="xs:string">string</MatchType>
    </Collect>
    <Collect index="1307">
      <CollectorName class="xs:string">string</CollectorName>
      <CoreName class="xs:string">string</CoreName>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Collect>
    <Collect index="630">
      <CollectorName class="xs:string">string</CollectorName>
      <IlluminatorName class="xs:string">string</IlluminatorName>
      <CoreName class="xs:string">string</CoreName>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <MatchType class="xs:string">string</MatchType>
      <Parameter name="string" class="xs:string">string</Parameter>
      <Parameter name="string" class="xs:string">string</Parameter>
    </Collect>
  </MatchInfo>
  <RMA>
    <RMAlgoType class="xs:string">CSA</RMAlgoType>
    <ImageType class="xs:string">INCA</ImageType>
    <RMAT>
      <RMRefTime class="xs:double">7501200000000.24</RMRefTime>
      <RMPosRef>
        <X class="xs:double">-26900999999999.8</X>
        <Y class="xs:double">-43350399999999.8</Y>
        <Z class="xs:double">41618600000000.2</Z>
      </RMPosRef>
      <RMVelRef>
        <X class="xs:double">14786100000000.2</X>
        <Y class="xs:double">-5314299999999.76</Y>
        <Z class="xs:double">13944800000000.2</Z>
      </RMVelRef>
      <CosDCACOAPoly order1="3638" order2="3380">
        <Coef exponent1="-3871" exponent2="-1733" class="xs:double">-6651399999999.76</Coef>
      </CosDCACOAPoly>
      <Kx1 class="xs:double">-7100399999999.76</Kx1>
      <Kx2 class="xs:double">-27020499999999.8</Kx2>
      <Ky1 class="xs:double">471500000000.235</Ky1>
      <Ky2 class="xs:double">46891800000000.2</Ky2>
    </RMAT>
  </RMA>
</SICD>
        "#;
        assert!(match from_str::<SicdMeta>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_collection_info() {
        let xml = r#"
  <CollectionInfo>
    <CollectorName class="xs:string">string</CollectorName>
    <IlluminatorName class="xs:string">string</IlluminatorName>
    <CoreName class="xs:string">string</CoreName>
    <CollectType class="xs:string">MONOSTATIC</CollectType>
    <RadarMode>
      <ModeType class="xs:string">SPOTLIGHT</ModeType>
    </RadarMode>
    <Classification class="xs:string">string</Classification>
    <CountryCode class="xs:string">string</CountryCode>
    <CountryCode class="xs:string">string</CountryCode>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
    <Parameter name="string" class="xs:string">string</Parameter>
  </CollectionInfo>
        "#;
        assert!(match from_str::<CollectionInfo>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_image_creation() {
        let xml = r#"
  <ImageCreation>
    <Application class="xs:string">string</Application>
    <DateTime class="xs:dateTime">1984-08-28T05:27:48.21</DateTime>
    <Site class="xs:string">string</Site>
    <Profile class="xs:string">string</Profile>
  </ImageCreation>
        "#;
        assert!(match from_str::<ImageCreation>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_image_data() {
        let xml = r#"
  <ImageData>
    <PixelType class="xs:string">RE32F_IM32F</PixelType>
    <AmpTable size="-2707">
      <Amplitude index="3686" class="xs:double">-7078599999999.76</Amplitude>
      <Amplitude index="1025" class="xs:double">21028400000000.2</Amplitude>
    </AmpTable>
    <NumRows class="xs:int">2816</NumRows>
    <NumCols class="xs:int">1313</NumCols>
    <FirstRow class="xs:int">-2905</FirstRow>
    <FirstCol class="xs:int">-3467</FirstCol>
    <FullImage>
      <NumRows class="xs:int">135</NumRows>
      <NumCols class="xs:int">-1971</NumCols>
    </FullImage>
    <SCPPixel>
      <Row class="xs:int">1978</Row>
      <Col class="xs:int">1467</Col>
    </SCPPixel>
    <ValidData size="545">
      <Vertex index="-1561">
        <Row class="xs:int">-3455</Row>
        <Col class="xs:int">2979</Col>
      </Vertex>
      <Vertex index="-4756">
        <Row class="xs:int">-108</Row>
        <Col class="xs:int">3828</Col>
      </Vertex>
      <Vertex index="-1004">
        <Row class="xs:int">1031</Row>
        <Col class="xs:int">2580</Col>
      </Vertex>
    </ValidData>
  </ImageData>
        "#;
        assert!(match from_str::<ImageData>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
    #[test]
    fn test_gen_xml_geo_data() {
        let xml = r#"
  <GeoData>
    <EarthModel class="xs:string">WGS_84</EarthModel>
    <SCP>
      <ECF>
        <X class="xs:double">-32922499999999.8</X>
        <Y class="xs:double">45571300000000.2</Y>
        <Z class="xs:double">-28286899999999.8</Z>
      </ECF>
      <LLH>
        <Lat class="xs:double">24698700000000.2</Lat>
        <Lon class="xs:double">-6815199999999.76</Lon>
        <HAE class="xs:double">15901200000000.2</HAE>
      </LLH>
    </SCP>
    <ImageCorners>
      <ICP index="3:LRLC">
        <Lat class="xs:double">-16505799999999.8</Lat>
        <Lon class="xs:double">15760600000000.2</Lon>
      </ICP>
      <ICP index="2:FRLC">
        <Lat class="xs:double">-30138199999999.8</Lat>
        <Lon class="xs:double">18215300000000.2</Lon>
      </ICP>
      <ICP index="1:FRFC">
        <Lat class="xs:double">7613900000000.24</Lat>
        <Lon class="xs:double">38986600000000.2</Lon>
      </ICP>
      <ICP index="4:LRFC">
        <Lat class="xs:double">19679800000000.2</Lat>
        <Lon class="xs:double">19660400000000.2</Lon>
      </ICP>
    </ImageCorners>
    <ValidData size="-1889">
      <Vertex index="-3007">
        <Lat class="xs:double">15413400000000.2</Lat>
        <Lon class="xs:double">18658000000000.2</Lon>
      </Vertex>
      <Vertex index="-3956">
        <Lat class="xs:double">-45829899999999.8</Lat>
        <Lon class="xs:double">1316900000000.23</Lon>
      </Vertex>
      <Vertex index="4940">
        <Lat class="xs:double">-48148799999999.8</Lat>
        <Lon class="xs:double">30975500000000.2</Lon>
      </Vertex>
    </ValidData>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
  </GeoData>
        "#;
        assert!(match from_str::<GeoData>(xml) {
            Ok(_) => true,
            Err(err) => {
                dbg!(err);
                false
            }
        });
    }
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
