//! Common types and metadata definition for SICD Version 0.5.0 [2011-01-12]
use serde::Deserialize;

pub mod grid;
pub mod match_info;

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
pub use crate::dep::v0_4_0::radiometric::Radiometric;
pub use crate::dep::v0_4_0::scpcoa::ScpCoa;
pub use crate::dep::v0_4_0::timeline::Timeline;
pub use crate::dep::v0_4_0::{
    Coef1D, Coef2D, IdxLL, IdxLLH, IdxRowCol, IdxXyzPoly, Parameter, Poly1D, Poly2D, RowCol,
    XyzPoly, CMPLX, LL, LLH, XYZ,
};
use grid::Grid;
use match_info::MatchInfo;

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

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use crate::dep::{v0_5_0::{CollectionInfo, SicdMeta}, v0_4_0::image_formation::ImageFormation};

    #[test]
    fn test_gen_xml() {
        let xml = r#"
        <?xml version="1.0" encoding="utf-8"?>
<!-- Created with Liquid Technologies Online Tools 1.0 (https://www.liquid-technologies.com) -->
<SICD xmlns="urn:SICD:0.5.0" xsi:schemaLocation="urn:SICD:0.5.0 schema.xsd" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <CollectionInfo>
    <CollectorName>string</CollectorName>
    <IlluminatorName>string</IlluminatorName>
    <CoreName>string</CoreName>
    <CollectType>MONOSTATIC</CollectType>
    <RadarMode>
      <ModeType>DYNAMIC STRIPMAP</ModeType>
    </RadarMode>
    <Classification>string</Classification>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <Parameter name="string">string</Parameter>
  </CollectionInfo>
  <ImageCreation>
    <Application>string</Application>
    <DateTime>1972-12-18T09:10:25.35</DateTime>
    <Site>string</Site>
    <Profile>string</Profile>
  </ImageCreation>
  <ImageData>
    <PixelType>RE16I_IM16I</PixelType>
    <AmpTable size="-2537">
      <Amplitude index="-319">-45893299999999.8</Amplitude>
    </AmpTable>
    <NumRows>-3498</NumRows>
    <NumCols>-982</NumCols>
    <FirstRow>-3964</FirstRow>
    <FirstCol>-2364</FirstCol>
    <FullImage>
      <NumRows>2751</NumRows>
      <NumCols>3227</NumCols>
    </FullImage>
    <SCPPixel>
      <Row>4438</Row>
      <Col>-2927</Col>
    </SCPPixel>
    <ValidData size="-2343">
      <Vertex index="1838">
        <Row>-4397</Row>
        <Col>4062</Col>
      </Vertex>
      <Vertex index="3726">
        <Row>4040</Row>
        <Col>3906</Col>
      </Vertex>
      <Vertex index="1223">
        <Row>-4292</Row>
        <Col>-4078</Col>
      </Vertex>
      <Vertex index="295">
        <Row>-2852</Row>
        <Col>4069</Col>
      </Vertex>
      <Vertex index="-827">
        <Row>-1028</Row>
        <Col>-2392</Col>
      </Vertex>
      <Vertex index="-1035">
        <Row>-1283</Row>
        <Col>-1627</Col>
      </Vertex>
      <Vertex index="-1289">
        <Row>-979</Row>
        <Col>-3792</Col>
      </Vertex>
    </ValidData>
  </ImageData>
  <GeoData>
    <EarthModel>WGS_84</EarthModel>
    <SCP>
      <ECF>
        <X>23620200000000.2</X>
        <Y>-7352699999999.76</Y>
        <Z>3728200000000.23</Z>
      </ECF>
      <LLH>
        <Lat>-32430599999999.8</Lat>
        <Lon>37980000000000.2</Lon>
        <HAE>752500000000.235</HAE>
      </LLH>
    </SCP>
    <ImageCorners>
      <ICP index="3:LRLC">
        <Lat>34382000000000.2</Lat>
        <Lon>-37261899999999.8</Lon>
      </ICP>
      <ICP index="3:LRLC">
        <Lat>35625700000000.2</Lat>
        <Lon>-20003499999999.8</Lon>
      </ICP>
      <ICP index="3:LRLC">
        <Lat>-47299999999999.8</Lat>
        <Lon>34413200000000.2</Lon>
      </ICP>
      <ICP index="2:FRLC">
        <Lat>-25096199999999.8</Lat>
        <Lon>4558400000000.24</Lon>
      </ICP>
    </ImageCorners>
    <ValidData size="3635">
      <Vertex index="-2590">
        <Lat>21868200000000.2</Lat>
        <Lon>17134200000000.2</Lon>
      </Vertex>
      <Vertex index="2747">
        <Lat>-24018699999999.8</Lat>
        <Lon>10882100000000.2</Lon>
      </Vertex>
      <Vertex index="-4728">
        <Lat>18102400000000.2</Lat>
        <Lon>-15695099999999.8</Lon>
      </Vertex>
    </ValidData>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Polygon size="-529">
        <Vertex index="-4427">
          <Lat>46576800000000.2</Lat>
          <Lon>-38099799999999.8</Lon>
        </Vertex>
        <Vertex index="3521">
          <Lat>-505799999999.765</Lat>
          <Lon>-11177599999999.8</Lon>
        </Vertex>
        <Vertex index="-3669">
          <Lat>-39702599999999.8</Lat>
          <Lon>1216900000000.23</Lon>
        </Vertex>
      </Polygon>
    </GeoInfo>
    <GeoInfo name="string">
      <Polygon size="2735">
        <Vertex index="-2200">
          <Lat>-43812299999999.8</Lat>
          <Lon>-40709399999999.8</Lon>
        </Vertex>
        <Vertex index="-3750">
          <Lat>38241000000000.2</Lat>
          <Lon>10245300000000.2</Lon>
        </Vertex>
        <Vertex index="3353">
          <Lat>45009200000000.2</Lat>
          <Lon>-1104999999999.77</Lon>
        </Vertex>
      </Polygon>
    </GeoInfo>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <Point>
        <Lat>7213100000000.24</Lat>
        <Lon>42092100000000.2</Lon>
      </Point>
    </GeoInfo>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Point>
        <Lat>1662900000000.23</Lat>
        <Lon>-32008199999999.8</Lon>
      </Point>
    </GeoInfo>
    <GeoInfo name="string">
      <GeoInfo name="string" />
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Polygon size="-4708">
        <Vertex index="224">
          <Lat>23112000000000.2</Lat>
          <Lon>-1192699999999.77</Lon>
        </Vertex>
        <Vertex index="-2523">
          <Lat>28479100000000.2</Lat>
          <Lon>13712100000000.2</Lon>
        </Vertex>
        <Vertex index="-3951">
          <Lat>16563100000000.2</Lat>
          <Lon>-29391199999999.8</Lon>
        </Vertex>
      </Polygon>
    </GeoInfo>
  </GeoData>
  <Grid>
    <ImagePlane>OTHER</ImagePlane>
    <Type>XRGYCR</Type>
    <TimeCOAPoly order1="3790" order2="-3971">
      <Coef exponent1="4644" exponent2="4055">-17121599999999.8</Coef>
    </TimeCOAPoly>
    <Row>
      <UVectECF>
        <X>-45179199999999.8</X>
        <Y>15679900000000.2</Y>
        <Z>11014900000000.2</Z>
      </UVectECF>
      <SS>-30450099999999.8</SS>
      <ImpRespWid>26584800000000.2</ImpRespWid>
      <Sgn>+1</Sgn>
      <ImpRespBW>44238400000000.2</ImpRespBW>
      <KCtr>12487900000000.2</KCtr>
      <DeltaK1>-14917699999999.8</DeltaK1>
      <DeltaK2>33583800000000.2</DeltaK2>
    </Row>
    <Col>
      <UVectECF>
        <X>-16749499999999.8</X>
        <Y>-47202199999999.8</Y>
        <Z>33339700000000.2</Z>
      </UVectECF>
      <SS>23388000000000.2</SS>
      <ImpRespWid>-12426599999999.8</ImpRespWid>
      <Sgn>+1</Sgn>
      <ImpRespBW>-22811299999999.8</ImpRespBW>
      <KCtr>47136500000000.2</KCtr>
      <DeltaK1>46829900000000.2</DeltaK1>
      <DeltaK2>23004500000000.2</DeltaK2>
    </Col>
  </Grid>
  <Timeline>
    <CollectStart>2011-11-23T10:30:52.85</CollectStart>
    <CollectDuration>36299400000000.2</CollectDuration>
    <IPP size="2379">
      <Set index="-2081">
        <TStart>-25085599999999.8</TStart>
        <TEnd>-44357899999999.8</TEnd>
        <IPPStart>640</IPPStart>
        <IPPEnd>-2895</IPPEnd>
        <IPPPoly order1="1407">
          <Coef exponent1="1158">26515300000000.2</Coef>
        </IPPPoly>
      </Set>
      <Set index="-4481">
        <TStart>-45685099999999.8</TStart>
        <TEnd>33171400000000.2</TEnd>
        <IPPStart>-1566</IPPStart>
        <IPPEnd>-1403</IPPEnd>
        <IPPPoly order1="-1163">
          <Coef exponent1="2018">34612500000000.2</Coef>
        </IPPPoly>
      </Set>
      <Set index="3608">
        <TStart>43031600000000.2</TStart>
        <TEnd>-46856099999999.8</TEnd>
        <IPPStart>1696</IPPStart>
        <IPPEnd>-2104</IPPEnd>
        <IPPPoly order1="1611">
          <Coef exponent1="-3137">-20298799999999.8</Coef>
        </IPPPoly>
      </Set>
    </IPP>
  </Timeline>
  <Position>
    <ARPPoly>
      <X order1="-293">
        <Coef exponent1="-921">-20705899999999.8</Coef>
      </X>
      <Y order1="1379">
        <Coef exponent1="1667">868700000000.235</Coef>
      </Y>
      <Z order1="2799">
        <Coef exponent1="1855">-3875999999999.77</Coef>
      </Z>
    </ARPPoly>
    <GRPPoly>
      <X order1="2170">
        <Coef exponent1="-2989">-32132899999999.8</Coef>
      </X>
      <Y order1="-3266">
        <Coef exponent1="2264">837800000000.235</Coef>
      </Y>
      <Z order1="-898">
        <Coef exponent1="4779">8402200000000.24</Coef>
      </Z>
    </GRPPoly>
    <TxAPCPoly>
      <X order1="-4682">
        <Coef exponent1="-3201">-34754399999999.8</Coef>
      </X>
      <Y order1="4989">
        <Coef exponent1="541">-7106699999999.76</Coef>
      </Y>
      <Z order1="2334">
        <Coef exponent1="2806">-2494799999999.77</Coef>
      </Z>
    </TxAPCPoly>
    <RcvAPC size="-2708">
      <RcvAPCPoly index="163">
        <X order1="-3417">
          <Coef exponent1="4834">-44041799999999.8</Coef>
        </X>
        <Y order1="-3985">
          <Coef exponent1="-4867">10047300000000.2</Coef>
        </Y>
        <Z order1="-1521">
          <Coef exponent1="3176">-32152399999999.8</Coef>
        </Z>
      </RcvAPCPoly>
    </RcvAPC>
  </Position>
  <RadarCollection>
    <RefFreqIndex>2178</RefFreqIndex>
    <TxFrequency>
      <Min>-38286199999999.8</Min>
      <Max>-6120699999999.76</Max>
    </TxFrequency>
    <TxPolarization>LHC</TxPolarization>
    <PolarizationHVAnglePoly order1="155">
      <Coef exponent1="-4784">4100000000.23491</Coef>
    </PolarizationHVAnglePoly>
    <TxSequence size="-4844">
      <TxStep index="-474" />
      <TxStep index="-2968" />
      <TxStep index="-4034" />
      <TxStep index="511" />
      <TxStep index="295" />
    </TxSequence>
    <Waveform size="1627">
      <WFParameters index="4540" />
      <WFParameters index="-2908" />
      <WFParameters index="-3023" />
      <WFParameters index="3346" />
    </Waveform>
    <RcvChannels size="4911">
      <ChanParameters index="-1795" />
      <ChanParameters index="-3794" />
      <ChanParameters index="780" />
      <ChanParameters index="1379" />
    </RcvChannels>
    <Area>
      <Corner>
        <ACP index="3">
          <Lat>14400900000000.2</Lat>
          <Lon>37388700000000.2</Lon>
          <HAE>-15015199999999.8</HAE>
        </ACP>
        <ACP index="4">
          <Lat>46906000000000.2</Lat>
          <Lon>-3059099999999.77</Lon>
          <HAE>37643100000000.2</HAE>
        </ACP>
        <ACP index="3">
          <Lat>-32304399999999.8</Lat>
          <Lon>47001100000000.2</Lon>
          <HAE>-40731599999999.8</HAE>
        </ACP>
        <ACP index="3">
          <Lat>-47869099999999.8</Lat>
          <Lon>-1010499999999.77</Lon>
          <HAE>-21731899999999.8</HAE>
        </ACP>
      </Corner>
      <Plane>
        <RefPt>
          <ECF>
            <X>-35539999999999.8</X>
            <Y>35257600000000.2</Y>
            <Z>37220800000000.2</Z>
          </ECF>
          <Line>7884200000000.24</Line>
          <Sample>37787500000000.2</Sample>
        </RefPt>
        <XDir>
          <UVectECF>
            <X>-19841899999999.8</X>
            <Y>32410900000000.2</Y>
            <Z>31904300000000.2</Z>
          </UVectECF>
          <LineSpacing>-5659499999999.76</LineSpacing>
          <NumLines>1866</NumLines>
          <FirstLine>-1460</FirstLine>
        </XDir>
        <YDir>
          <UVectECF>
            <X>-12816899999999.8</X>
            <Y>-6936099999999.76</Y>
            <Z>17150500000000.2</Z>
          </UVectECF>
          <SampleSpacing>33125300000000.2</SampleSpacing>
          <NumSamples>-4796</NumSamples>
          <FirstSample>-3065</FirstSample>
        </YDir>
      </Plane>
    </Area>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
  </RadarCollection>
  <ImageFormation>
    <SegmentIdentifier>string</SegmentIdentifier>
    <RcvChanProc>
      <NumChanProc>-3506</NumChanProc>
      <ChanIndex>-2078</ChanIndex>
      <ChanIndex>-3974</ChanIndex>
      <ChanIndex>1346</ChanIndex>
    </RcvChanProc>
    <TxRcvPolarizationProc>H:H</TxRcvPolarizationProc>
    <ImageFormAlgo>OTHER</ImageFormAlgo>
    <TStartProc>389700000000.235</TStartProc>
    <TEndProc>27477100000000.2</TEndProc>
    <TxFrequencyProc>
      <MinProc>-27918099999999.8</MinProc>
      <MaxProc>-6827499999999.76</MaxProc>
    </TxFrequencyProc>
    <STBeamComp>SV</STBeamComp>
    <ImageBeamComp>SV</ImageBeamComp>
    <AzAutofocus>NO</AzAutofocus>
    <RgAutofocus>NO</RgAutofocus>
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>false</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>false</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <PolarizationCalibration>
      <HVAngleCompApplied>0</HVAngleCompApplied>
      <DistortionCorrectionApplied>0</DistortionCorrectionApplied>
      <Distortion>
        <A>-48525499999999.8</A>
        <F1>
          <Real>-21292899999999.8</Real>
          <Imag>17736800000000.2</Imag>
        </F1>
        <Q1>
          <Real>-40448199999999.8</Real>
          <Imag>-44459999999999.8</Imag>
        </Q1>
        <Q2>
          <Real>23265500000000.2</Real>
          <Imag>49951100000000.2</Imag>
        </Q2>
        <F2>
          <Real>21192700000000.2</Real>
          <Imag>2476600000000.23</Imag>
        </F2>
        <Q3>
          <Real>32474900000000.2</Real>
          <Imag>35329200000000.2</Imag>
        </Q3>
        <Q4>
          <Real>5411500000000.24</Real>
          <Imag>-32795299999999.8</Imag>
        </Q4>
      </Distortion>
    </PolarizationCalibration>
  </ImageFormation>
  <SCPCOA>
    <SCPTime>23069800000000.2</SCPTime>
    <ARPPos>
      <X>-11863299999999.8</X>
      <Y>-17465699999999.8</Y>
      <Z>-16652899999999.8</Z>
    </ARPPos>
    <ARPVel>
      <X>-36903599999999.8</X>
      <Y>1849900000000.23</Y>
      <Z>16913200000000.2</Z>
    </ARPVel>
    <ARPAcc>
      <X>-18763999999999.8</X>
      <Y>-40215699999999.8</Y>
      <Z>30284600000000.2</Z>
    </ARPAcc>
    <SideOfTrack>L</SideOfTrack>
    <SlantRange>24701400000000.2</SlantRange>
    <GroundRange>24371600000000.2</GroundRange>
    <DopplerConeAng>24444800000000.2</DopplerConeAng>
    <GrazeAng>33.5424658039451</GrazeAng>
    <IncidenceAng>78.7263358039451</IncidenceAng>
    <TwistAng>44.3905258039451</TwistAng>
    <SlopeAng>83.2549558039451</SlopeAng>
  </SCPCOA>
  <Radiometric>
    <NoisePoly order1="-2435" order2="-4311">
      <Coef exponent1="-3707" exponent2="-3563">25750300000000.2</Coef>
      <Coef exponent1="3486" exponent2="2771">-8580599999999.76</Coef>
    </NoisePoly>
    <NoiseLevelType>ABSOLUTE</NoiseLevelType>
    <RCSSFPoly order1="-925" order2="3486">
      <Coef exponent1="-3251" exponent2="3204">-29769199999999.8</Coef>
    </RCSSFPoly>
    <BetaZeroSFPoly order1="-3289" order2="-1672">
      <Coef exponent1="-3745" exponent2="-4452">32219100000000.2</Coef>
      <Coef exponent1="-2189" exponent2="4253">-42655599999999.8</Coef>
    </BetaZeroSFPoly>
    <SigmaZeroSFPoly order1="-102" order2="380">
      <Coef exponent1="-4987" exponent2="-4855">11772500000000.2</Coef>
      <Coef exponent1="-1307" exponent2="4862">-18423099999999.8</Coef>
      <Coef exponent1="4673" exponent2="-1160">-11563599999999.8</Coef>
    </SigmaZeroSFPoly>
    <SigmaZeroSFIncidenceMap>NOT_APPLIED</SigmaZeroSFIncidenceMap>
    <GammaZeroSFPoly order1="3967" order2="-1430">
      <Coef exponent1="-3434" exponent2="-942">-15860799999999.8</Coef>
    </GammaZeroSFPoly>
    <GammaZeroSFIncidenceMap>NOT_APPLIED</GammaZeroSFIncidenceMap>
  </Radiometric>
  <Antenna>
    <Tx>
      <XAxisPoly>
        <X order1="-2823">
          <Coef exponent1="3612">-11523899999999.8</Coef>
        </X>
        <Y order1="-4369">
          <Coef exponent1="-2839">13066600000000.2</Coef>
        </Y>
        <Z order1="4487">
          <Coef exponent1="-2649">34789900000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="-4906">
          <Coef exponent1="-3033">-3919599999999.77</Coef>
        </X>
        <Y order1="-556">
          <Coef exponent1="-2196">-44909999999999.8</Coef>
        </Y>
        <Z order1="-3845">
          <Coef exponent1="2804">-5330699999999.76</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>1194500000000.23</FreqZero>
      <HPBW>
        <DCX>-31031399999999.8</DCX>
        <DCY>38178500000000.2</DCY>
      </HPBW>
      <Array>
        <GainPoly order1="2687" order2="-2589">
          <Coef exponent1="2714" exponent2="-1686">10623900000000.2</Coef>
        </GainPoly>
        <PhasePoly order1="-2530" order2="-996">
          <Coef exponent1="3936" exponent2="4432">-32932499999999.8</Coef>
        </PhasePoly>
      </Array>
      <GainBSPoly order1="-4118">
        <Coef exponent1="3975">28841700000000.2</Coef>
      </GainBSPoly>
      <EBFreqShift>1</EBFreqShift>
      <MLFreqDilation>1</MLFreqDilation>
    </Tx>
    <Rcv>
      <XAxisPoly>
        <X order1="4402">
          <Coef exponent1="3453">-46074699999999.8</Coef>
        </X>
        <Y order1="-3950">
          <Coef exponent1="604">43443200000000.2</Coef>
        </Y>
        <Z order1="4452">
          <Coef exponent1="3882">6369600000000.24</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="2936">
          <Coef exponent1="-2304">48026100000000.2</Coef>
        </X>
        <Y order1="963">
          <Coef exponent1="3289">20033600000000.2</Coef>
        </Y>
        <Z order1="2611">
          <Coef exponent1="-2041">39101500000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-36857899999999.8</FreqZero>
      <HPBW>
        <DCX>20473600000000.2</DCX>
        <DCY>8876300000000.23</DCY>
      </HPBW>
      <Array>
        <GainPoly order1="-895" order2="4388">
          <Coef exponent1="1576" exponent2="2854">-44173899999999.8</Coef>
        </GainPoly>
        <PhasePoly order1="2330" order2="3650">
          <Coef exponent1="-4116" exponent2="3714">38305100000000.2</Coef>
        </PhasePoly>
      </Array>
      <Elem>
        <GainPoly order1="-3762" order2="-3620">
          <Coef exponent1="-343" exponent2="-965">40036900000000.2</Coef>
        </GainPoly>
        <PhasePoly order1="-4505" order2="1884">
          <Coef exponent1="-1085" exponent2="-3182">-46239799999999.8</Coef>
        </PhasePoly>
      </Elem>
      <MLFreqDilation>1</MLFreqDilation>
    </Rcv>
    <TwoWay>
      <XAxisPoly>
        <X order1="-3420">
          <Coef exponent1="1793">18893900000000.2</Coef>
        </X>
        <Y order1="-1895">
          <Coef exponent1="-3498">-6674799999999.76</Coef>
        </Y>
        <Z order1="585">
          <Coef exponent1="4676">46942300000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="-1957">
          <Coef exponent1="-3813">-9282099999999.77</Coef>
        </X>
        <Y order1="3093">
          <Coef exponent1="4008">-16501599999999.8</Coef>
        </Y>
        <Z order1="-206">
          <Coef exponent1="-2672">7245600000000.24</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-13793799999999.8</FreqZero>
      <EBFreqShift>true</EBFreqShift>
      <MLFreqDilation>true</MLFreqDilation>
    </TwoWay>
  </Antenna>
  <ErrorStatistics>
    <CompositeSCP>
      <RowColErr>
        <Row>-22783999999999.8</Row>
        <Col>-43663699999999.8</Col>
        <RowCol>-18614899999999.8</RowCol>
      </RowColErr>
    </CompositeSCP>
    <Components>
      <RadarSensor>
        <RangeBias>-25973599999999.8</RangeBias>
      </RadarSensor>
      <TropoError />
    </Components>
    <AdditionalParms>
      <Parameter name="string">string</Parameter>
    </AdditionalParms>
  </ErrorStatistics>
  <MatchInfo size="2883">
    <Collect index="-3094">
      <CollectorName>string</CollectorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Collect>
    <Collect index="-3865">
      <CollectorName>string</CollectorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Collect>
    <Collect index="45">
      <CollectorName>string</CollectorName>
      <IlluminatorName>string</IlluminatorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Collect>
    <Collect index="-2032">
      <CollectorName>string</CollectorName>
      <IlluminatorName>string</IlluminatorName>
      <CoreName>string</CoreName>
      <MatchType>string</MatchType>
      <MatchType>string</MatchType>
    </Collect>
  </MatchInfo>
  <RGAZCOMP>
    <RgAzRefTime>25069900000000.2</RgAzRefTime>
    <Time1>-3338299999999.77</Time1>
    <Time2>3500100000000.23</Time2>
    <AzToCosSF>5476900000000.24</AzToCosSF>
    <KazToTimePoly order1="4316">
      <Coef exponent1="3062">46705300000000.2</Coef>
    </KazToTimePoly>
  </RGAZCOMP>
</SICD>
        "#;
        assert!(match from_str::<SicdMeta>(xml) {
            Ok(_) => true,
            Err(err) => {dbg!(err);false},
        });
    }
    #[test]
    fn test_gen_xml_collection_info() {
        let xml = r#"
  <CollectionInfo>
    <CollectorName>string</CollectorName>
    <IlluminatorName>string</IlluminatorName>
    <CoreName>string</CoreName>
    <CollectType>MONOSTATIC</CollectType>
    <RadarMode>
      <ModeType>DYNAMIC STRIPMAP</ModeType>
    </RadarMode>
    <Classification>string</Classification>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <Parameter name="string">string</Parameter>
  </CollectionInfo>
        "#;
        assert!(match from_str::<CollectionInfo>(xml) {
            Ok(_) => true,
            Err(err) => {dbg!(err);false},
        });
    }
    #[test]
    fn test_gen_xml_image_formation() {
        let xml = r#"
  <ImageFormation>
    <SegmentIdentifier>string</SegmentIdentifier>
    <RcvChanProc>
      <NumChanProc>-3506</NumChanProc>
      <ChanIndex>-2078</ChanIndex>
      <ChanIndex>-3974</ChanIndex>
      <ChanIndex>1346</ChanIndex>
    </RcvChanProc>
    <TxRcvPolarizationProc>H:H</TxRcvPolarizationProc>
    <ImageFormAlgo>OTHER</ImageFormAlgo>
    <TStartProc>389700000000.235</TStartProc>
    <TEndProc>27477100000000.2</TEndProc>
    <TxFrequencyProc>
      <MinProc>-27918099999999.8</MinProc>
      <MaxProc>-6827499999999.76</MaxProc>
    </TxFrequencyProc>
    <STBeamComp>SV</STBeamComp>
    <ImageBeamComp>SV</ImageBeamComp>
    <AzAutofocus>NO</AzAutofocus>
    <RgAutofocus>NO</RgAutofocus>
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>false</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>false</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <PolarizationCalibration>
      <HVAngleCompApplied>0</HVAngleCompApplied>
      <DistortionCorrectionApplied>0</DistortionCorrectionApplied>
      <Distortion>
        <A>-48525499999999.8</A>
        <F1>
          <Real>-21292899999999.8</Real>
          <Imag>17736800000000.2</Imag>
        </F1>
        <Q1>
          <Real>-40448199999999.8</Real>
          <Imag>-44459999999999.8</Imag>
        </Q1>
        <Q2>
          <Real>23265500000000.2</Real>
          <Imag>49951100000000.2</Imag>
        </Q2>
        <F2>
          <Real>21192700000000.2</Real>
          <Imag>2476600000000.23</Imag>
        </F2>
        <Q3>
          <Real>32474900000000.2</Real>
          <Imag>35329200000000.2</Imag>
        </Q3>
        <Q4>
          <Real>5411500000000.24</Real>
          <Imag>-32795299999999.8</Imag>
        </Q4>
      </Distortion>
    </PolarizationCalibration>
  </ImageFormation>
        "#;
        assert!(match from_str::<ImageFormation>(xml) {
            Ok(_) => true,
            Err(err) => {dbg!(err);false},
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
