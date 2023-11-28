use quick_xml::de::from_str;
use sicd_rs::{
    dep::v0_4_0::image_data::{AmpTable, Amplitude, PixelType, ValidDataRC},
    v1_3_0::{image_formation::RcvChanProc, CollectionInfo, ImageCreation, ImageData, SicdMeta},
};

#[test]
fn test_generated_xml() {
    let xml = r#"
    <?xml version="1.0" encoding="utf-8"?>
<!-- Created with Liquid Technologies Online Tools 1.0 (https://www.liquid-technologies.com) -->
<SICD xmlns="urn:SICD:1.3.0" xsi:schemaLocation="urn:SICD:1.3.0 schema.xsd" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <CollectionInfo>
    <CollectorName>string</CollectorName>
    <IlluminatorName>string</IlluminatorName>
    <CoreName>string</CoreName>
    <CollectType>BISTATIC</CollectType>
    <RadarMode>
      <ModeType>SPOTLIGHT</ModeType>
      <ModeID>string</ModeID>
    </RadarMode>
    <Classification>string</Classification>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <CountryCode>string</CountryCode>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
  </CollectionInfo>
  <ImageCreation>
    <Application>string</Application>
    <DateTime>2001-07-16T00:53:41.38</DateTime>
    <Site>string</Site>
    <Profile>string</Profile>
  </ImageCreation>
  <ImageData>
    <PixelType>AMP8I_PHS8I</PixelType>
    <AmpTable size="-2108">
      <Amplitude index="-2945">35839600000000.2</Amplitude>
      <Amplitude index="-4227">48471100000000.2</Amplitude>
      <Amplitude index="-4710">48178500000000.2</Amplitude>
      <Amplitude index="-1179">-33888699999999.8</Amplitude>
      <Amplitude index="-1677">21511700000000.2</Amplitude>
      <Amplitude index="842">22052600000000.2</Amplitude>
      <Amplitude index="4190">-17212299999999.8</Amplitude>
      <Amplitude index="-3995">42199900000000.2</Amplitude>
      <Amplitude index="-4418">35582700000000.2</Amplitude>
      <Amplitude index="-4312">15188600000000.2</Amplitude>
      <Amplitude index="349">3140700000000.23</Amplitude>
      <Amplitude index="1411">11033800000000.2</Amplitude>
      <Amplitude index="-4906">928100000000.235</Amplitude>
      <Amplitude index="-3998">25566500000000.2</Amplitude>
      <Amplitude index="-1780">47988100000000.2</Amplitude>
      <Amplitude index="3263">-31778099999999.8</Amplitude>
      <Amplitude index="-1928">-6066899999999.76</Amplitude>
      <Amplitude index="2920">42384700000000.2</Amplitude>
      <Amplitude index="3197">15139600000000.2</Amplitude>
      <Amplitude index="-39">7869100000000.24</Amplitude>
      <Amplitude index="1278">23110400000000.2</Amplitude>
      <Amplitude index="-1453">17305700000000.2</Amplitude>
      <Amplitude index="-1894">4599600000000.24</Amplitude>
      <Amplitude index="-1943">-32943599999999.8</Amplitude>
      <Amplitude index="-3251">-12518799999999.8</Amplitude>
      <Amplitude index="2116">44431600000000.2</Amplitude>
      <Amplitude index="-2796">33463700000000.2</Amplitude>
      <Amplitude index="1775">17411800000000.2</Amplitude>
      <Amplitude index="-2827">-3302699999999.77</Amplitude>
      <Amplitude index="4753">1967400000000.23</Amplitude>
      <Amplitude index="3816">12639600000000.2</Amplitude>
      <Amplitude index="3391">-14753799999999.8</Amplitude>
      <Amplitude index="3888">-9805799999999.77</Amplitude>
      <Amplitude index="-867">-2036699999999.77</Amplitude>
      <Amplitude index="359">-32332599999999.8</Amplitude>
      <Amplitude index="-3977">-9318699999999.77</Amplitude>
      <Amplitude index="-1403">-990199999999.765</Amplitude>
      <Amplitude index="-4759">30377600000000.2</Amplitude>
      <Amplitude index="-3233">46804700000000.2</Amplitude>
      <Amplitude index="-2002">-3662899999999.77</Amplitude>
      <Amplitude index="-2964">42959700000000.2</Amplitude>
      <Amplitude index="808">44723500000000.2</Amplitude>
      <Amplitude index="-2317">38195200000000.2</Amplitude>
      <Amplitude index="4618">-2745699999999.77</Amplitude>
      <Amplitude index="2618">-38213299999999.8</Amplitude>
      <Amplitude index="2066">-14730399999999.8</Amplitude>
      <Amplitude index="1761">47647300000000.2</Amplitude>
      <Amplitude index="1971">-49859099999999.8</Amplitude>
      <Amplitude index="3920">-49776299999999.8</Amplitude>
      <Amplitude index="2842">40863100000000.2</Amplitude>
      <Amplitude index="-3674">32607400000000.2</Amplitude>
      <Amplitude index="1347">49823100000000.2</Amplitude>
      <Amplitude index="-2275">-19525799999999.8</Amplitude>
      <Amplitude index="846">23033300000000.2</Amplitude>
      <Amplitude index="3106">37373900000000.2</Amplitude>
      <Amplitude index="-26">-25075599999999.8</Amplitude>
      <Amplitude index="-3328">1197200000000.23</Amplitude>
      <Amplitude index="-1839">45202600000000.2</Amplitude>
      <Amplitude index="-4544">39183000000000.2</Amplitude>
      <Amplitude index="-4159">-49310499999999.8</Amplitude>
      <Amplitude index="-598">44080200000000.2</Amplitude>
      <Amplitude index="2178">-8197599999999.76</Amplitude>
      <Amplitude index="-299">24963200000000.2</Amplitude>
      <Amplitude index="2308">-11679399999999.8</Amplitude>
      <Amplitude index="2930">-47732999999999.8</Amplitude>
      <Amplitude index="4117">-32552999999999.8</Amplitude>
      <Amplitude index="-3162">-10886799999999.8</Amplitude>
      <Amplitude index="-1693">-12250099999999.8</Amplitude>
      <Amplitude index="-2051">8252800000000.24</Amplitude>
      <Amplitude index="1748">46358200000000.2</Amplitude>
      <Amplitude index="-2027">-26854499999999.8</Amplitude>
      <Amplitude index="1620">38807600000000.2</Amplitude>
      <Amplitude index="1205">-4268899999999.77</Amplitude>
      <Amplitude index="-3145">-33592099999999.8</Amplitude>
      <Amplitude index="1604">24504200000000.2</Amplitude>
      <Amplitude index="4558">-49979199999999.8</Amplitude>
      <Amplitude index="4181">27731300000000.2</Amplitude>
      <Amplitude index="-315">-30815299999999.8</Amplitude>
      <Amplitude index="-3917">-28335199999999.8</Amplitude>
      <Amplitude index="281">2285100000000.23</Amplitude>
      <Amplitude index="740">-29857099999999.8</Amplitude>
      <Amplitude index="4373">28788500000000.2</Amplitude>
      <Amplitude index="4620">-17706099999999.8</Amplitude>
      <Amplitude index="-4346">27607700000000.2</Amplitude>
      <Amplitude index="-3187">43860100000000.2</Amplitude>
      <Amplitude index="1571">-3689199999999.77</Amplitude>
      <Amplitude index="-2829">-37944499999999.8</Amplitude>
      <Amplitude index="2096">-29130099999999.8</Amplitude>
      <Amplitude index="-2212">32974000000000.2</Amplitude>
      <Amplitude index="2976">-48719299999999.8</Amplitude>
      <Amplitude index="641">6668100000000.24</Amplitude>
      <Amplitude index="2228">-45201299999999.8</Amplitude>
      <Amplitude index="-4331">41152800000000.2</Amplitude>
      <Amplitude index="-2436">-9347599999999.77</Amplitude>
      <Amplitude index="4227">-36112699999999.8</Amplitude>
      <Amplitude index="-2308">-42172499999999.8</Amplitude>
      <Amplitude index="-4456">-34809999999999.8</Amplitude>
      <Amplitude index="-1104">-40411399999999.8</Amplitude>
      <Amplitude index="-2058">37410000000000.2</Amplitude>
      <Amplitude index="4261">-20248399999999.8</Amplitude>
      <Amplitude index="-1081">-9058999999999.77</Amplitude>
      <Amplitude index="4828">22177100000000.2</Amplitude>
      <Amplitude index="-4121">-733599999999.765</Amplitude>
      <Amplitude index="2831">29754400000000.2</Amplitude>
      <Amplitude index="-4323">-24022299999999.8</Amplitude>
      <Amplitude index="4130">-22148399999999.8</Amplitude>
      <Amplitude index="-809">1530400000000.23</Amplitude>
      <Amplitude index="4587">-49269999999999.8</Amplitude>
      <Amplitude index="-214">38931600000000.2</Amplitude>
      <Amplitude index="2210">-44952699999999.8</Amplitude>
      <Amplitude index="-4334">15888200000000.2</Amplitude>
      <Amplitude index="3534">-45762199999999.8</Amplitude>
      <Amplitude index="1694">7882600000000.24</Amplitude>
      <Amplitude index="-913">-43482199999999.8</Amplitude>
      <Amplitude index="2310">11369200000000.2</Amplitude>
      <Amplitude index="4145">-9528399999999.77</Amplitude>
      <Amplitude index="4037">5000000000.23491</Amplitude>
      <Amplitude index="1209">-34526599999999.8</Amplitude>
      <Amplitude index="839">-49893799999999.8</Amplitude>
      <Amplitude index="4600">7423300000000.24</Amplitude>
      <Amplitude index="-3716">-4117399999999.77</Amplitude>
      <Amplitude index="-1612">16287800000000.2</Amplitude>
      <Amplitude index="-2742">49072600000000.2</Amplitude>
      <Amplitude index="1592">3911000000000.23</Amplitude>
      <Amplitude index="-3068">-11770099999999.8</Amplitude>
      <Amplitude index="1173">-9516499999999.77</Amplitude>
      <Amplitude index="-3469">37560100000000.2</Amplitude>
      <Amplitude index="-2691">23302200000000.2</Amplitude>
      <Amplitude index="560">44044800000000.2</Amplitude>
      <Amplitude index="-4476">911300000000.235</Amplitude>
      <Amplitude index="-4160">21795200000000.2</Amplitude>
      <Amplitude index="-4334">-4595799999999.76</Amplitude>
      <Amplitude index="-1548">831400000000.235</Amplitude>
      <Amplitude index="-1252">41903000000000.2</Amplitude>
      <Amplitude index="3944">30397100000000.2</Amplitude>
      <Amplitude index="-766">-2246699999999.77</Amplitude>
      <Amplitude index="4293">-35324599999999.8</Amplitude>
      <Amplitude index="4221">10774200000000.2</Amplitude>
      <Amplitude index="-1799">-30945699999999.8</Amplitude>
      <Amplitude index="3165">17863200000000.2</Amplitude>
      <Amplitude index="4196">36958100000000.2</Amplitude>
      <Amplitude index="3720">-15134499999999.8</Amplitude>
      <Amplitude index="4964">971200000000.235</Amplitude>
      <Amplitude index="-2484">-47193599999999.8</Amplitude>
      <Amplitude index="-2309">38788000000000.2</Amplitude>
      <Amplitude index="987">14340400000000.2</Amplitude>
      <Amplitude index="4486">-4906699999999.76</Amplitude>
      <Amplitude index="-97">-8955499999999.77</Amplitude>
      <Amplitude index="-1078">38474700000000.2</Amplitude>
      <Amplitude index="-1824">21752700000000.2</Amplitude>
      <Amplitude index="1159">24016400000000.2</Amplitude>
      <Amplitude index="1447">-11077899999999.8</Amplitude>
      <Amplitude index="4589">-36024399999999.8</Amplitude>
      <Amplitude index="-244">-49364199999999.8</Amplitude>
      <Amplitude index="4535">12320500000000.2</Amplitude>
      <Amplitude index="-871">-13455499999999.8</Amplitude>
      <Amplitude index="-3761">-12621899999999.8</Amplitude>
      <Amplitude index="895">-28561099999999.8</Amplitude>
      <Amplitude index="3460">21793300000000.2</Amplitude>
      <Amplitude index="-424">33551600000000.2</Amplitude>
      <Amplitude index="-2433">-15321499999999.8</Amplitude>
      <Amplitude index="1499">-49347899999999.8</Amplitude>
      <Amplitude index="-2948">28004200000000.2</Amplitude>
      <Amplitude index="289">-2166499999999.77</Amplitude>
      <Amplitude index="1565">1162900000000.23</Amplitude>
      <Amplitude index="-2845">-6465499999999.76</Amplitude>
      <Amplitude index="3729">-40099199999999.8</Amplitude>
      <Amplitude index="-4372">-32053199999999.8</Amplitude>
      <Amplitude index="-2751">-1723799999999.77</Amplitude>
      <Amplitude index="-1102">35664500000000.2</Amplitude>
      <Amplitude index="-4659">-25475499999999.8</Amplitude>
      <Amplitude index="-4255">14589700000000.2</Amplitude>
      <Amplitude index="-251">-26669999999999.8</Amplitude>
      <Amplitude index="195">7486700000000.24</Amplitude>
      <Amplitude index="3615">-22412799999999.8</Amplitude>
      <Amplitude index="645">17430300000000.2</Amplitude>
      <Amplitude index="-729">-1786799999999.77</Amplitude>
      <Amplitude index="-392">-23091099999999.8</Amplitude>
      <Amplitude index="-4098">13818800000000.2</Amplitude>
      <Amplitude index="-3160">-32118999999999.8</Amplitude>
      <Amplitude index="1109">49723100000000.2</Amplitude>
      <Amplitude index="-1501">-5810499999999.76</Amplitude>
      <Amplitude index="-923">47758400000000.2</Amplitude>
      <Amplitude index="-74">-47509599999999.8</Amplitude>
      <Amplitude index="-1890">-8998699999999.77</Amplitude>
      <Amplitude index="4895">-13679499999999.8</Amplitude>
      <Amplitude index="-1719">10099400000000.2</Amplitude>
      <Amplitude index="3014">-48853499999999.8</Amplitude>
      <Amplitude index="-2278">-49597099999999.8</Amplitude>
      <Amplitude index="316">47192400000000.2</Amplitude>
      <Amplitude index="-2395">45401600000000.2</Amplitude>
      <Amplitude index="1169">-11938199999999.8</Amplitude>
      <Amplitude index="4472">4120100000000.23</Amplitude>
      <Amplitude index="-4918">-10923899999999.8</Amplitude>
      <Amplitude index="1382">29366600000000.2</Amplitude>
      <Amplitude index="-4108">8668400000000.24</Amplitude>
      <Amplitude index="-2012">-28900899999999.8</Amplitude>
      <Amplitude index="-2542">-46308599999999.8</Amplitude>
      <Amplitude index="3954">13264300000000.2</Amplitude>
      <Amplitude index="-2618">-263399999999.765</Amplitude>
      <Amplitude index="2407">-541399999999.765</Amplitude>
      <Amplitude index="-2362">-4856199999999.76</Amplitude>
      <Amplitude index="-2136">-29872799999999.8</Amplitude>
      <Amplitude index="-1538">32611300000000.2</Amplitude>
      <Amplitude index="1808">-5064899999999.76</Amplitude>
      <Amplitude index="4968">-41379299999999.8</Amplitude>
      <Amplitude index="-3934">-28792899999999.8</Amplitude>
      <Amplitude index="4183">15685500000000.2</Amplitude>
      <Amplitude index="-1197">46924700000000.2</Amplitude>
      <Amplitude index="-53">36649900000000.2</Amplitude>
      <Amplitude index="4693">-39821099999999.8</Amplitude>
      <Amplitude index="-1133">1735800000000.23</Amplitude>
      <Amplitude index="-1792">-9716799999999.77</Amplitude>
      <Amplitude index="-4356">-38286099999999.8</Amplitude>
      <Amplitude index="-1448">26445500000000.2</Amplitude>
      <Amplitude index="-3839">13960300000000.2</Amplitude>
      <Amplitude index="2658">-46579599999999.8</Amplitude>
      <Amplitude index="-2688">26596300000000.2</Amplitude>
      <Amplitude index="1902">-33455599999999.8</Amplitude>
      <Amplitude index="-4057">24587900000000.2</Amplitude>
      <Amplitude index="-3050">-31787599999999.8</Amplitude>
      <Amplitude index="2100">-31116099999999.8</Amplitude>
      <Amplitude index="2968">-49697699999999.8</Amplitude>
      <Amplitude index="-199">-41330499999999.8</Amplitude>
      <Amplitude index="-2073">8891800000000.23</Amplitude>
      <Amplitude index="1566">42613100000000.2</Amplitude>
      <Amplitude index="-3621">-12830199999999.8</Amplitude>
      <Amplitude index="280">13895900000000.2</Amplitude>
      <Amplitude index="-3921">24648300000000.2</Amplitude>
      <Amplitude index="-3693">38352300000000.2</Amplitude>
      <Amplitude index="-3632">-27096699999999.8</Amplitude>
      <Amplitude index="-291">41630600000000.2</Amplitude>
      <Amplitude index="-1667">-14276199999999.8</Amplitude>
      <Amplitude index="-1796">-42758699999999.8</Amplitude>
      <Amplitude index="4809">-34770399999999.8</Amplitude>
      <Amplitude index="4667">-28509599999999.8</Amplitude>
      <Amplitude index="3750">24886400000000.2</Amplitude>
      <Amplitude index="1715">28722100000000.2</Amplitude>
      <Amplitude index="-1082">-30212599999999.8</Amplitude>
      <Amplitude index="2205">-18224099999999.8</Amplitude>
      <Amplitude index="4228">47774300000000.2</Amplitude>
      <Amplitude index="3244">26630100000000.2</Amplitude>
      <Amplitude index="-3921">-31004599999999.8</Amplitude>
      <Amplitude index="17">-10586699999999.8</Amplitude>
      <Amplitude index="62">9221500000000.23</Amplitude>
      <Amplitude index="1581">44369000000000.2</Amplitude>
      <Amplitude index="-4652">-28926099999999.8</Amplitude>
      <Amplitude index="1090">46599500000000.2</Amplitude>
      <Amplitude index="2112">29369300000000.2</Amplitude>
      <Amplitude index="3556">-6041599999999.76</Amplitude>
      <Amplitude index="1826">-9234699999999.77</Amplitude>
      <Amplitude index="-3942">-35956599999999.8</Amplitude>
      <Amplitude index="1222">-5833799999999.76</Amplitude>
      <Amplitude index="-4489">-11096499999999.8</Amplitude>
      <Amplitude index="2002">24079800000000.2</Amplitude>
      <Amplitude index="-2528">40999800000000.2</Amplitude>
    </AmpTable>
    <NumRows>3129</NumRows>
    <NumCols>-3409</NumCols>
    <FirstRow>2046</FirstRow>
    <FirstCol>-3567</FirstCol>
    <FullImage>
      <NumRows>-1113</NumRows>
      <NumCols>-4770</NumCols>
    </FullImage>
    <SCPPixel>
      <Row>2055</Row>
      <Col>981</Col>
    </SCPPixel>
    <ValidData size="-2191">
      <Vertex index="3719">
        <Row>1662</Row>
        <Col>-3873</Col>
      </Vertex>
      <Vertex index="1939">
        <Row>2481</Row>
        <Col>-2781</Col>
      </Vertex>
      <Vertex index="-2500">
        <Row>-4934</Row>
        <Col>2123</Col>
      </Vertex>
      <Vertex index="841">
        <Row>2171</Row>
        <Col>992</Col>
      </Vertex>
      <Vertex index="-2385">
        <Row>4948</Row>
        <Col>1491</Col>
      </Vertex>
    </ValidData>
  </ImageData>
  <GeoData>
    <EarthModel>WGS_84</EarthModel>
    <SCP>
      <ECF>
        <X>-26766399999999.8</X>
        <Y>-45636599999999.8</Y>
        <Z>-39475099999999.8</Z>
      </ECF>
      <LLH>
        <Lat>-63.8263741960549</Lat>
        <Lon>40.8497458039451</Lon>
        <HAE>19690000000000.2</HAE>
      </LLH>
    </SCP>
    <ImageCorners>
      <ICP index="1:FRFC">
        <Lat>25190500000000.2</Lat>
        <Lon>19915100000000.2</Lon>
      </ICP>
      <ICP index="1:FRFC">
        <Lat>-32725699999999.8</Lat>
        <Lon>30433500000000.2</Lon>
      </ICP>
      <ICP index="1:FRFC">
        <Lat>34308600000000.2</Lat>
        <Lon>34118400000000.2</Lon>
      </ICP>
      <ICP index="3:LRLC">
        <Lat>40011400000000.2</Lat>
        <Lon>-19668099999999.8</Lon>
      </ICP>
    </ImageCorners>
    <ValidData size="594">
      <Vertex index="-4637">
        <Lat>-35.9310541960549</Lat>
        <Lon>-132.129894196055</Lon>
      </Vertex>
      <Vertex index="1829">
        <Lat>-44.2972741960549</Lat>
        <Lon>12.1815058039451</Lon>
      </Vertex>
      <Vertex index="2003">
        <Lat>78.0859858039451</Lat>
        <Lon>39.8089858039451</Lon>
      </Vertex>
      <Vertex index="271">
        <Lat>48.7868458039451</Lat>
        <Lon>128.292305803945</Lon>
      </Vertex>
      <Vertex index="-1454">
        <Lat>-64.9918741960549</Lat>
        <Lon>-143.275494196055</Lon>
      </Vertex>
    </ValidData>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Line size="-3754">
        <Endpoint index="-4695">
          <Lat>-39442399999999.8</Lat>
          <Lon>46177200000000.2</Lon>
        </Endpoint>
        <Endpoint index="4909">
          <Lat>-16560399999999.8</Lat>
          <Lon>-11080299999999.8</Lon>
        </Endpoint>
      </Line>
      <GeoInfo name="string" />
    </GeoInfo>
    <GeoInfo name="string">
      <Point>
        <Lat>24.7417258039451</Lat>
        <Lon>27.3461458039451</Lon>
      </Point>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
  </GeoData>
  <Grid>
    <ImagePlane>SLANT</ImagePlane>
    <Type>RGZERO</Type>
    <TimeCOAPoly order1="2947" order2="956">
      <Coef exponent1="6840" exponent2="1171">3304600000000.23</Coef>
      <Coef exponent1="7273" exponent2="1947">-1469999999999.77</Coef>
    </TimeCOAPoly>
    <Row>
      <UVectECF>
        <X>23570000000000.2</X>
        <Y>13139800000000.2</Y>
        <Z>39955400000000.2</Z>
      </UVectECF>
      <SS>29083400000000.2</SS>
      <ImpRespWid>13382000000000.2</ImpRespWid>
      <Sgn>+1</Sgn>
      <ImpRespBW>37048400000000.2</ImpRespBW>
      <KCtr>4544000000000.24</KCtr>
      <DeltaK1>-41703999999999.8</DeltaK1>
      <DeltaK2>-1709999999999.77</DeltaK2>
      <WgtType>
        <WindowName>string</WindowName>
      </WgtType>
    </Row>
    <Col>
      <UVectECF>
        <X>2312800000000.23</X>
        <Y>47781500000000.2</Y>
        <Z>23985800000000.2</Z>
      </UVectECF>
      <SS>-7462399999999.76</SS>
      <ImpRespWid>-1611199999999.77</ImpRespWid>
      <Sgn>-1</Sgn>
      <ImpRespBW>-25822099999999.8</ImpRespBW>
      <KCtr>13657000000000.2</KCtr>
      <DeltaK1>16256200000000.2</DeltaK1>
      <DeltaK2>14928900000000.2</DeltaK2>
      <DeltaKCOAPoly order1="4245" order2="1348">
        <Coef exponent1="8760" exponent2="8577">14046900000000.2</Coef>
      </DeltaKCOAPoly>
      <WgtType>
        <WindowName>string</WindowName>
      </WgtType>
    </Col>
  </Grid>
  <Timeline>
    <CollectStart>2015-12-08T02:29:30.43</CollectStart>
    <CollectDuration>33921900000000.2</CollectDuration>
    <IPP size="1363">
      <Set index="-3143">
        <TStart>25834300000000.2</TStart>
        <TEnd>-13355799999999.8</TEnd>
        <IPPStart>-4663</IPPStart>
        <IPPEnd>-4258</IPPEnd>
        <IPPPoly order1="7915">
          <Coef exponent1="4530">-29263999999999.8</Coef>
        </IPPPoly>
      </Set>
      <Set index="-3793">
        <TStart>-4816599999999.76</TStart>
        <TEnd>-17322699999999.8</TEnd>
        <IPPStart>-127</IPPStart>
        <IPPEnd>-4394</IPPEnd>
        <IPPPoly order1="6959">
          <Coef exponent1="3767">-27857899999999.8</Coef>
        </IPPPoly>
      </Set>
      <Set index="4375">
        <TStart>32484100000000.2</TStart>
        <TEnd>12485800000000.2</TEnd>
        <IPPStart>-236</IPPStart>
        <IPPEnd>-1847</IPPEnd>
        <IPPPoly order1="8460">
          <Coef exponent1="6046">-23447099999999.8</Coef>
        </IPPPoly>
      </Set>
    </IPP>
  </Timeline>
  <Position>
    <ARPPoly>
      <X order1="1770">
        <Coef exponent1="8275">-27742999999999.8</Coef>
      </X>
      <Y order1="4575">
        <Coef exponent1="4680">19751200000000.2</Coef>
      </Y>
      <Z order1="8540">
        <Coef exponent1="6281">32067700000000.2</Coef>
      </Z>
    </ARPPoly>
    <GRPPoly>
      <X order1="1434">
        <Coef exponent1="8963">-38065599999999.8</Coef>
      </X>
      <Y order1="4560">
        <Coef exponent1="7238">11678300000000.2</Coef>
      </Y>
      <Z order1="9841">
        <Coef exponent1="2729">-13892499999999.8</Coef>
      </Z>
    </GRPPoly>
    <TxAPCPoly>
      <X order1="190">
        <Coef exponent1="6349">-35147899999999.8</Coef>
      </X>
      <Y order1="4681">
        <Coef exponent1="3578">4609200000000.24</Coef>
      </Y>
      <Z order1="121">
        <Coef exponent1="2048">13504400000000.2</Coef>
      </Z>
    </TxAPCPoly>
    <RcvAPC size="2081">
      <RcvAPCPoly index="3140">
        <X order1="8512">
          <Coef exponent1="1676">44070200000000.2</Coef>
        </X>
        <Y order1="2671">
          <Coef exponent1="155">40354800000000.2</Coef>
        </Y>
        <Z order1="6001">
          <Coef exponent1="4122">24561700000000.2</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="1808">
        <X order1="1777">
          <Coef exponent1="6693">7857800000000.24</Coef>
        </X>
        <Y order1="6378">
          <Coef exponent1="3934">18523200000000.2</Coef>
        </Y>
        <Z order1="6337">
          <Coef exponent1="1287">-46952999999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-1995">
        <X order1="907">
          <Coef exponent1="9966">-8872299999999.77</Coef>
        </X>
        <Y order1="6047">
          <Coef exponent1="2959">-23675899999999.8</Coef>
        </Y>
        <Z order1="3178">
          <Coef exponent1="4571">-14584599999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-4926">
        <X order1="2076">
          <Coef exponent1="7001">-16971599999999.8</Coef>
        </X>
        <Y order1="8937">
          <Coef exponent1="6190">-11827299999999.8</Coef>
        </Y>
        <Z order1="8464">
          <Coef exponent1="8931">-23496899999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="1690">
        <X order1="792">
          <Coef exponent1="5552">-36981199999999.8</Coef>
        </X>
        <Y order1="8871">
          <Coef exponent1="3266">27458800000000.2</Coef>
        </Y>
        <Z order1="7114">
          <Coef exponent1="6172">37424600000000.2</Coef>
        </Z>
      </RcvAPCPoly>
    </RcvAPC>
  </Position>
  <RadarCollection>
    <TxFrequency>
      <Min>-29927999999999.8</Min>
      <Max>31808600000000.2</Max>
    </TxFrequency>
    <RefFreqIndex>-3765</RefFreqIndex>
    <Waveform size="-3515">
      <WFParameters index="2494" />
      <WFParameters index="3444" />
      <WFParameters index="-1492" />
      <WFParameters index="-998" />
    </Waveform>
    <TxPolarization>H</TxPolarization>
    <TxSequence size="-3704">
      <TxStep index="-2959" />
      <TxStep index="2265" />
      <TxStep index="-4521" />
      <TxStep index="2550" />
    </TxSequence>
    <RcvChannels size="267">
      <ChanParameters index="-2133">
        <TxRcvPolarization>E:H</TxRcvPolarization>
      </ChanParameters>
      <ChanParameters index="4109">
        <TxRcvPolarization>V:H</TxRcvPolarization>
      </ChanParameters>
      <ChanParameters index="1012">
        <TxRcvPolarization>X:V</TxRcvPolarization>
      </ChanParameters>
      <ChanParameters index="-979">
        <TxRcvPolarization>V:H</TxRcvPolarization>
      </ChanParameters>
      <ChanParameters index="-3712">
        <TxRcvPolarization>S:S</TxRcvPolarization>
      </ChanParameters>
    </RcvChannels>
    <Area>
      <Corner>
        <ACP index="3">
          <Lat>27.4914058039451</Lat>
          <Lon>113.816345803945</Lon>
          <HAE>25664600000000.2</HAE>
        </ACP>
        <ACP index="2">
          <Lat>36.8598658039451</Lat>
          <Lon>-175.669374196055</Lon>
          <HAE>-42381199999999.8</HAE>
        </ACP>
        <ACP index="4">
          <Lat>31.9001458039451</Lat>
          <Lon>-39.9432541960549</Lon>
          <HAE>-30230699999999.8</HAE>
        </ACP>
        <ACP index="4">
          <Lat>-7.86815419605491</Lat>
          <Lon>35.3554258039451</Lon>
          <HAE>-43805699999999.8</HAE>
        </ACP>
      </Corner>
      <Plane>
        <RefPt>
          <ECF>
            <X>-586699999999.765</X>
            <Y>-22656399999999.8</Y>
            <Z>47002200000000.2</Z>
          </ECF>
          <Line>33629000000000.2</Line>
          <Sample>-16347199999999.8</Sample>
        </RefPt>
        <XDir>
          <UVectECF>
            <X>1915500000000.23</X>
            <Y>-28841899999999.8</Y>
            <Z>27521700000000.2</Z>
          </UVectECF>
          <LineSpacing>-18210599999999.8</LineSpacing>
          <NumLines>-3401</NumLines>
          <FirstLine>3116</FirstLine>
        </XDir>
        <YDir>
          <UVectECF>
            <X>24043900000000.2</X>
            <Y>-1565399999999.77</Y>
            <Z>8737700000000.24</Z>
          </UVectECF>
          <SampleSpacing>-20234799999999.8</SampleSpacing>
          <NumSamples>753</NumSamples>
          <FirstSample>-4274</FirstSample>
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
    <RcvChanProc>
      <NumChanProc>841</NumChanProc>
      <PRFScaleFactor>-48329899999999.8</PRFScaleFactor>
      <ChanIndex>-3118</ChanIndex>
      <ChanIndex>4321</ChanIndex>
      <ChanIndex>-1460</ChanIndex>
      <ChanIndex>-2843</ChanIndex>
    </RcvChanProc>
    <TxRcvPolarizationProc>LHC:RHC</TxRcvPolarizationProc>
    <TStartProc>16995200000000.2</TStartProc>
    <TEndProc>14517600000000.2</TEndProc>
    <TxFrequencyProc>
      <MinProc>-27889099999999.8</MinProc>
      <MaxProc>-14676999999999.8</MaxProc>
    </TxFrequencyProc>
    <SegmentIdentifier>string</SegmentIdentifier>
    <ImageFormAlgo>PFA</ImageFormAlgo>
    <STBeamComp>SV</STBeamComp>
    <ImageBeamComp>SV</ImageBeamComp>
    <AzAutofocus>NO</AzAutofocus>
    <RgAutofocus>SV</RgAutofocus>
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
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
    <Processing>
      <Type>string</Type>
      <Applied>0</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <PolarizationCalibration>
      <DistortCorrectionApplied>0</DistortCorrectionApplied>
      <Distortion>
        <A>-1070499999999.77</A>
        <F1>
          <Real>49764800000000.2</Real>
          <Imag>4910600000000.24</Imag>
        </F1>
        <Q1>
          <Real>12037400000000.2</Real>
          <Imag>21435900000000.2</Imag>
        </Q1>
        <Q2>
          <Real>-16137399999999.8</Real>
          <Imag>-39769099999999.8</Imag>
        </Q2>
        <F2>
          <Real>-13639399999999.8</Real>
          <Imag>13078200000000.2</Imag>
        </F2>
        <Q3>
          <Real>42111800000000.2</Real>
          <Imag>4421300000000.24</Imag>
        </Q3>
        <Q4>
          <Real>-19324799999999.8</Real>
          <Imag>-5304599999999.76</Imag>
        </Q4>
      </Distortion>
    </PolarizationCalibration>
  </ImageFormation>
  <SCPCOA>
    <SCPTime>34243400000000.2</SCPTime>
    <ARPPos>
      <X>28934600000000.2</X>
      <Y>-26665299999999.8</Y>
      <Z>-43593799999999.8</Z>
    </ARPPos>
    <ARPVel>
      <X>39776700000000.2</X>
      <Y>-5709999999999.76</Y>
      <Z>407000000000.235</Z>
    </ARPVel>
    <ARPAcc>
      <X>-12590999999999.8</X>
      <Y>4237600000000.23</Y>
      <Z>-48323399999999.8</Z>
    </ARPAcc>
    <SideOfTrack>R</SideOfTrack>
    <SlantRange>28554600000000.2</SlantRange>
    <GroundRange>-32131899999999.8</GroundRange>
    <DopplerConeAng>-22907799999999.8</DopplerConeAng>
    <GrazeAng>40.1450458039451</GrazeAng>
    <IncidenceAng>49.6833358039451</IncidenceAng>
    <TwistAng>1.02240580394509</TwistAng>
    <SlopeAng>1.92780580394509</SlopeAng>
    <AzimAng>101.659865803945</AzimAng>
    <LayoverAng>162.166145803945</LayoverAng>
  </SCPCOA>
  <Radiometric>
    <NoiseLevel>
      <NoiseLevelType>RELATIVE</NoiseLevelType>
      <NoisePoly order1="8199" order2="528">
        <Coef exponent1="7230" exponent2="6804">3310400000000.23</Coef>
      </NoisePoly>
    </NoiseLevel>
    <RCSSFPoly order1="8609" order2="6339">
      <Coef exponent1="9537" exponent2="4142">19391200000000.2</Coef>
      <Coef exponent1="2683" exponent2="5122">-16532999999999.8</Coef>
      <Coef exponent1="2801" exponent2="4926">-40100099999999.8</Coef>
    </RCSSFPoly>
    <SigmaZeroSFPoly order1="5028" order2="868">
      <Coef exponent1="2549" exponent2="716">-41929999999999.8</Coef>
      <Coef exponent1="854" exponent2="1604">-2061299999999.77</Coef>
      <Coef exponent1="5945" exponent2="2760">-4899699999999.76</Coef>
      <Coef exponent1="1510" exponent2="9696">-46468599999999.8</Coef>
      <Coef exponent1="8265" exponent2="5436">47127100000000.2</Coef>
    </SigmaZeroSFPoly>
    <BetaZeroSFPoly order1="1579" order2="2374">
      <Coef exponent1="4962" exponent2="3476">-929599999999.765</Coef>
    </BetaZeroSFPoly>
    <GammaZeroSFPoly order1="7748" order2="2905">
      <Coef exponent1="8564" exponent2="4705">5008200000000.24</Coef>
    </GammaZeroSFPoly>
  </Radiometric>
  <Antenna>
    <Tx>
      <XAxisPoly>
        <X order1="3434">
          <Coef exponent1="9632">4289400000000.23</Coef>
        </X>
        <Y order1="4769">
          <Coef exponent1="5213">33375000000000.2</Coef>
        </Y>
        <Z order1="2653">
          <Coef exponent1="4466">40730100000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="8698">
          <Coef exponent1="4196">48048900000000.2</Coef>
        </X>
        <Y order1="5825">
          <Coef exponent1="1182">979300000000.235</Coef>
        </Y>
        <Z order1="3823">
          <Coef exponent1="701">31849100000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>44780900000000.2</FreqZero>
      <Array>
        <GainPoly order1="5703" order2="226">
          <Coef exponent1="7591" exponent2="9915">-16648099999999.8</Coef>
        </GainPoly>
        <PhasePoly order1="8806" order2="9559">
          <Coef exponent1="3458" exponent2="4534">-24194099999999.8</Coef>
        </PhasePoly>
      </Array>
      <Elem>
        <GainPoly order1="3718" order2="4619">
          <Coef exponent1="3004" exponent2="8092">144100000000.235</Coef>
        </GainPoly>
        <PhasePoly order1="8686" order2="6282">
          <Coef exponent1="7699" exponent2="127">-41247299999999.8</Coef>
        </PhasePoly>
      </Elem>
      <EBFreqShift>1</EBFreqShift>
      <MLFreqDilation>1</MLFreqDilation>
    </Tx>
    <Rcv>
      <XAxisPoly>
        <X order1="5774">
          <Coef exponent1="7696">11670000000000.2</Coef>
        </X>
        <Y order1="4017">
          <Coef exponent1="9261">-17960899999999.8</Coef>
        </Y>
        <Z order1="5588">
          <Coef exponent1="7833">-25329899999999.8</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="2474">
          <Coef exponent1="5162">-40642099999999.8</Coef>
        </X>
        <Y order1="8611">
          <Coef exponent1="8928">-24170699999999.8</Coef>
        </Y>
        <Z order1="181">
          <Coef exponent1="8918">-15872799999999.8</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-24810299999999.8</FreqZero>
      <Array>
        <GainPoly order1="867" order2="7671">
          <Coef exponent1="6747" exponent2="2317">3285800000000.23</Coef>
        </GainPoly>
        <PhasePoly order1="9906" order2="74">
          <Coef exponent1="3914" exponent2="4652">14398300000000.2</Coef>
        </PhasePoly>
      </Array>
      <Elem>
        <GainPoly order1="1205" order2="3801">
          <Coef exponent1="5712" exponent2="1091">-32343999999999.8</Coef>
        </GainPoly>
        <PhasePoly order1="5780" order2="4420">
          <Coef exponent1="6376" exponent2="8741">12018300000000.2</Coef>
        </PhasePoly>
      </Elem>
      <GainBSPoly order1="3655">
        <Coef exponent1="1471">-31363599999999.8</Coef>
      </GainBSPoly>
    </Rcv>
    <TwoWay>
      <XAxisPoly>
        <X order1="3316">
          <Coef exponent1="667">-34397699999999.8</Coef>
        </X>
        <Y order1="6">
          <Coef exponent1="4189">-30411199999999.8</Coef>
        </Y>
        <Z order1="1972">
          <Coef exponent1="908">-42722899999999.8</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="7616">
          <Coef exponent1="8949">20756900000000.2</Coef>
        </X>
        <Y order1="5130">
          <Coef exponent1="808">-9992899999999.77</Coef>
        </Y>
        <Z order1="6959">
          <Coef exponent1="1515">24475500000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>21501800000000.2</FreqZero>
      <Array>
        <GainPoly order1="9466" order2="5532">
          <Coef exponent1="5109" exponent2="3958">-49788799999999.8</Coef>
        </GainPoly>
        <PhasePoly order1="1481" order2="3720">
          <Coef exponent1="6460" exponent2="3268">30235500000000.2</Coef>
        </PhasePoly>
      </Array>
      <EBFreqShift>false</EBFreqShift>
      <MLFreqDilation>0</MLFreqDilation>
    </TwoWay>
  </Antenna>
  <ErrorStatistics>
    <CompositeSCP>
      <Rg>-15394399999999.8</Rg>
      <Az>-9546799999999.77</Az>
      <RgAz>35723600000000.2</RgAz>
    </CompositeSCP>
    <Components>
      <PosVelErr>
        <Frame>RIC_ECI</Frame>
        <P1>17529700000000.2</P1>
        <P2>33740900000000.2</P2>
        <P3>-4245499999999.77</P3>
        <V1>43722500000000.2</V1>
        <V2>32469400000000.2</V2>
        <V3>-42195799999999.8</V3>
      </PosVelErr>
      <RadarSensor>
        <RangeBias>42781300000000.2</RangeBias>
      </RadarSensor>
      <TropoError />
    </Components>
    <Unmodeled>
      <Xrow>-32628799999999.8</Xrow>
      <Ycol>10409300000000.2</Ycol>
      <XrowYcol>35331800000000.2</XrowYcol>
    </Unmodeled>
    <AdditionalParms>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </AdditionalParms>
  </ErrorStatistics>
  <MatchInfo>
    <NumMatchTypes>1775</NumMatchTypes>
    <MatchType index="575">
      <TypeID>string</TypeID>
      <CurrentIndex>4757</CurrentIndex>
      <NumMatchCollections>-2747</NumMatchCollections>
    </MatchType>
    <MatchType index="-3774">
      <TypeID>string</TypeID>
      <CurrentIndex>1290</CurrentIndex>
      <NumMatchCollections>83</NumMatchCollections>
    </MatchType>
    <MatchType index="-3118">
      <TypeID>string</TypeID>
      <CurrentIndex>-182</CurrentIndex>
      <NumMatchCollections>-2005</NumMatchCollections>
      <MatchCollection index="1897">
        <CoreName>string</CoreName>
      </MatchCollection>
      <MatchCollection index="-1743">
        <CoreName>string</CoreName>
      </MatchCollection>
    </MatchType>
  </MatchInfo>
  <RMA>
    <RMAlgoType>RG_DOP</RMAlgoType>
    <ImageType>RMCR</ImageType>
    <RMCR>
      <PosRef>
        <X>-38152099999999.8</X>
        <Y>4768800000000.24</Y>
        <Z>42183800000000.2</Z>
      </PosRef>
      <VelRef>
        <X>-27645899999999.8</X>
        <Y>48748900000000.2</Y>
        <Z>37702900000000.2</Z>
      </VelRef>
      <DopConeAngRef>-27466899999999.8</DopConeAngRef>
    </RMCR>
  </RMA>
</SICD>
"#;
    //let sicd_meta = quick_xml::de::
    let sicd_meta = from_str::<SicdMeta>(xml);
    // let sicd_meta = serde_path_to_error::deserialize(sicd_meta);
    assert!(match sicd_meta {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
// #[test]
// fn test_generated_xml_() {
//     let xml = r#"
//     "#;
//     assert!(match from_str::<>(xml) {
//         Ok(_) => true,
//         Err(err) => {dbg!(err);false},
//     });
// }
