use quick_xml::de::from_str;
use sicd_rs::{
    dep::v0_4_0::image_data::{AmpTable, Amplitude, PixelType, ValidDataRC},
    v1_3_0::{image_formation::RcvChanProc, CollectionInfo, ImageCreation, ImageData, SicdMeta},
};

#[test]
fn test_generated_xml() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<!-- Created with Liquid Technologies Online Tools 1.0 (https://www.liquid-technologies.com) -->
<SICD xmlns="urn:SICD:1.3.0" xsi:schemaLocation="urn:SICD:1.3.0 schema.xsd" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
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
    <Parameter name="string">string</Parameter>
  </CollectionInfo>
  <ImageCreation>
    <Application>string</Application>
    <DateTime>1994-04-05T10:27:33.85</DateTime>
    <Site>string</Site>
    <Profile>string</Profile>
  </ImageCreation>
  <ImageData>
    <PixelType>RE16I_IM16I</PixelType>
    <AmpTable size="-1490">
      <Amplitude index="2273">162200000000.235</Amplitude>
      <Amplitude index="555">-43949299999999.8</Amplitude>
      <Amplitude index="1752">2033500000000.23</Amplitude>
      <Amplitude index="782">-49253599999999.8</Amplitude>
      <Amplitude index="1410">39259000000000.2</Amplitude>
      <Amplitude index="3550">-8400399999999.76</Amplitude>
      <Amplitude index="1711">49045500000000.2</Amplitude>
      <Amplitude index="12">35525600000000.2</Amplitude>
      <Amplitude index="2757">39219800000000.2</Amplitude>
      <Amplitude index="-4778">-7486199999999.76</Amplitude>
      <Amplitude index="1095">-48358399999999.8</Amplitude>
      <Amplitude index="-3986">-12052999999999.8</Amplitude>
      <Amplitude index="483">24281200000000.2</Amplitude>
      <Amplitude index="-4323">-18821199999999.8</Amplitude>
      <Amplitude index="2107">43491900000000.2</Amplitude>
      <Amplitude index="-619">31852100000000.2</Amplitude>
      <Amplitude index="-2066">22261400000000.2</Amplitude>
      <Amplitude index="1017">-42300599999999.8</Amplitude>
      <Amplitude index="4210">-6153699999999.76</Amplitude>
      <Amplitude index="964">12312700000000.2</Amplitude>
      <Amplitude index="3536">-4628299999999.76</Amplitude>
      <Amplitude index="4206">45800300000000.2</Amplitude>
      <Amplitude index="-3825">27920800000000.2</Amplitude>
      <Amplitude index="1812">33673500000000.2</Amplitude>
      <Amplitude index="-3454">-2826699999999.77</Amplitude>
      <Amplitude index="2818">-46796499999999.8</Amplitude>
      <Amplitude index="-1712">-32673299999999.8</Amplitude>
      <Amplitude index="-943">21088100000000.2</Amplitude>
      <Amplitude index="-997">-32401699999999.8</Amplitude>
      <Amplitude index="122">43235900000000.2</Amplitude>
      <Amplitude index="-474">-23361999999999.8</Amplitude>
      <Amplitude index="-2032">20606000000000.2</Amplitude>
      <Amplitude index="-455">-46354899999999.8</Amplitude>
      <Amplitude index="-3774">44849100000000.2</Amplitude>
      <Amplitude index="-1113">-7581399999999.76</Amplitude>
      <Amplitude index="4343">-16276299999999.8</Amplitude>
      <Amplitude index="-2042">-10096399999999.8</Amplitude>
      <Amplitude index="715">-34422799999999.8</Amplitude>
      <Amplitude index="-4041">-35663799999999.8</Amplitude>
      <Amplitude index="-2381">26905000000000.2</Amplitude>
      <Amplitude index="-4384">-26901799999999.8</Amplitude>
      <Amplitude index="-3428">-26103199999999.8</Amplitude>
      <Amplitude index="-3469">-9393899999999.77</Amplitude>
      <Amplitude index="-103">-37990399999999.8</Amplitude>
      <Amplitude index="-1831">39083500000000.2</Amplitude>
      <Amplitude index="1767">24499900000000.2</Amplitude>
      <Amplitude index="4262">16405500000000.2</Amplitude>
      <Amplitude index="-3295">8723900000000.24</Amplitude>
      <Amplitude index="-3430">-28550299999999.8</Amplitude>
      <Amplitude index="35">-41890399999999.8</Amplitude>
      <Amplitude index="1566">23271700000000.2</Amplitude>
      <Amplitude index="-520">23038900000000.2</Amplitude>
      <Amplitude index="375">-5542399999999.76</Amplitude>
      <Amplitude index="2363">42977800000000.2</Amplitude>
      <Amplitude index="1017">-25006299999999.8</Amplitude>
      <Amplitude index="1150">-24310699999999.8</Amplitude>
      <Amplitude index="4141">24319700000000.2</Amplitude>
      <Amplitude index="3708">-27836599999999.8</Amplitude>
      <Amplitude index="-3908">-44220699999999.8</Amplitude>
      <Amplitude index="530">-45156899999999.8</Amplitude>
      <Amplitude index="467">-49746299999999.8</Amplitude>
      <Amplitude index="1316">-209799999999.765</Amplitude>
      <Amplitude index="2474">-31072699999999.8</Amplitude>
      <Amplitude index="-890">13172800000000.2</Amplitude>
      <Amplitude index="-2715">48427800000000.2</Amplitude>
      <Amplitude index="4988">38136100000000.2</Amplitude>
      <Amplitude index="1399">-31917599999999.8</Amplitude>
      <Amplitude index="-3875">-17111599999999.8</Amplitude>
      <Amplitude index="2830">-7317699999999.76</Amplitude>
      <Amplitude index="2015">20852800000000.2</Amplitude>
      <Amplitude index="1698">5992800000000.24</Amplitude>
      <Amplitude index="183">-43303299999999.8</Amplitude>
      <Amplitude index="-2242">-8015599999999.76</Amplitude>
      <Amplitude index="3309">-31698299999999.8</Amplitude>
      <Amplitude index="2933">44887500000000.2</Amplitude>
      <Amplitude index="-219">-40083799999999.8</Amplitude>
      <Amplitude index="1615">-4492099999999.76</Amplitude>
      <Amplitude index="344">15403700000000.2</Amplitude>
      <Amplitude index="-3989">45009400000000.2</Amplitude>
      <Amplitude index="4829">-15181299999999.8</Amplitude>
      <Amplitude index="-4664">-39547799999999.8</Amplitude>
      <Amplitude index="2013">11747000000000.2</Amplitude>
      <Amplitude index="-2489">23366100000000.2</Amplitude>
      <Amplitude index="1170">23323700000000.2</Amplitude>
      <Amplitude index="1307">4187100000000.23</Amplitude>
      <Amplitude index="-614">18234400000000.2</Amplitude>
      <Amplitude index="-1437">34442700000000.2</Amplitude>
      <Amplitude index="-1214">48681700000000.2</Amplitude>
      <Amplitude index="-158">6461900000000.24</Amplitude>
      <Amplitude index="-2779">-17239699999999.8</Amplitude>
      <Amplitude index="-1417">-27199499999999.8</Amplitude>
      <Amplitude index="3384">-22034899999999.8</Amplitude>
      <Amplitude index="62">39965500000000.2</Amplitude>
      <Amplitude index="-2802">-31522699999999.8</Amplitude>
      <Amplitude index="1465">-4156299999999.77</Amplitude>
      <Amplitude index="-2722">33288900000000.2</Amplitude>
      <Amplitude index="-561">-14667899999999.8</Amplitude>
      <Amplitude index="1750">6528800000000.24</Amplitude>
      <Amplitude index="3587">40087600000000.2</Amplitude>
      <Amplitude index="3158">4215100000000.23</Amplitude>
      <Amplitude index="3028">-40229999999999.8</Amplitude>
      <Amplitude index="523">-24857299999999.8</Amplitude>
      <Amplitude index="102">29572700000000.2</Amplitude>
      <Amplitude index="2429">31711400000000.2</Amplitude>
      <Amplitude index="-4235">4753300000000.24</Amplitude>
      <Amplitude index="-3302">3651000000000.23</Amplitude>
      <Amplitude index="2280">15533700000000.2</Amplitude>
      <Amplitude index="4899">30554800000000.2</Amplitude>
      <Amplitude index="-2339">-7840099999999.76</Amplitude>
      <Amplitude index="-3887">-14853599999999.8</Amplitude>
      <Amplitude index="138">-6773199999999.76</Amplitude>
      <Amplitude index="-4132">-32771699999999.8</Amplitude>
      <Amplitude index="-1859">10576000000000.2</Amplitude>
      <Amplitude index="-2615">-49703699999999.8</Amplitude>
      <Amplitude index="-3305">31333100000000.2</Amplitude>
      <Amplitude index="-3719">8331300000000.24</Amplitude>
      <Amplitude index="2488">17993700000000.2</Amplitude>
      <Amplitude index="248">-23936499999999.8</Amplitude>
      <Amplitude index="1757">8700100000000.24</Amplitude>
      <Amplitude index="2694">21047900000000.2</Amplitude>
      <Amplitude index="-3432">-9729499999999.77</Amplitude>
      <Amplitude index="-3917">-40105999999999.8</Amplitude>
      <Amplitude index="-2114">19124300000000.2</Amplitude>
      <Amplitude index="-3951">28854400000000.2</Amplitude>
      <Amplitude index="-1365">36942600000000.2</Amplitude>
      <Amplitude index="-2008">-6292299999999.76</Amplitude>
      <Amplitude index="2896">-3561699999999.77</Amplitude>
      <Amplitude index="-4717">-12946499999999.8</Amplitude>
      <Amplitude index="-4891">-11998799999999.8</Amplitude>
      <Amplitude index="4373">40446400000000.2</Amplitude>
      <Amplitude index="573">23991300000000.2</Amplitude>
      <Amplitude index="1477">-23687399999999.8</Amplitude>
      <Amplitude index="-806">8653400000000.24</Amplitude>
      <Amplitude index="2877">-45193199999999.8</Amplitude>
      <Amplitude index="-3695">22921600000000.2</Amplitude>
      <Amplitude index="-3702">17909200000000.2</Amplitude>
      <Amplitude index="1522">-9921599999999.77</Amplitude>
      <Amplitude index="-3053">-38886499999999.8</Amplitude>
      <Amplitude index="-1760">48782300000000.2</Amplitude>
      <Amplitude index="3837">12287300000000.2</Amplitude>
      <Amplitude index="9">-5008199999999.76</Amplitude>
      <Amplitude index="1395">-19997499999999.8</Amplitude>
      <Amplitude index="141">19106300000000.2</Amplitude>
      <Amplitude index="2937">-21561199999999.8</Amplitude>
      <Amplitude index="1516">-34573599999999.8</Amplitude>
      <Amplitude index="-2503">-20430299999999.8</Amplitude>
      <Amplitude index="1497">36496200000000.2</Amplitude>
      <Amplitude index="-3468">-8313599999999.76</Amplitude>
      <Amplitude index="2550">34518700000000.2</Amplitude>
      <Amplitude index="1795">20205600000000.2</Amplitude>
      <Amplitude index="4035">-44318499999999.8</Amplitude>
      <Amplitude index="1580">13425300000000.2</Amplitude>
      <Amplitude index="2396">12010200000000.2</Amplitude>
      <Amplitude index="2849">-11117199999999.8</Amplitude>
      <Amplitude index="-2303">41717400000000.2</Amplitude>
      <Amplitude index="-4534">-47688699999999.8</Amplitude>
      <Amplitude index="-37">-18556299999999.8</Amplitude>
      <Amplitude index="-964">-39265399999999.8</Amplitude>
      <Amplitude index="-3996">-15236399999999.8</Amplitude>
      <Amplitude index="2490">22833000000000.2</Amplitude>
      <Amplitude index="2928">334900000000.235</Amplitude>
      <Amplitude index="-1036">47627300000000.2</Amplitude>
      <Amplitude index="-205">33414100000000.2</Amplitude>
      <Amplitude index="-4706">28721000000000.2</Amplitude>
      <Amplitude index="-2524">27782100000000.2</Amplitude>
      <Amplitude index="-1439">-2123399999999.77</Amplitude>
      <Amplitude index="-1917">-31836299999999.8</Amplitude>
      <Amplitude index="2194">-5593999999999.76</Amplitude>
      <Amplitude index="2919">-49476099999999.8</Amplitude>
      <Amplitude index="604">39403300000000.2</Amplitude>
      <Amplitude index="4062">-9512899999999.77</Amplitude>
      <Amplitude index="-4853">23445200000000.2</Amplitude>
      <Amplitude index="-3923">-27341599999999.8</Amplitude>
      <Amplitude index="2994">-16474799999999.8</Amplitude>
      <Amplitude index="-386">-45413199999999.8</Amplitude>
      <Amplitude index="-1836">-9259399999999.77</Amplitude>
      <Amplitude index="-4038">45121600000000.2</Amplitude>
      <Amplitude index="4092">-9984499999999.77</Amplitude>
      <Amplitude index="1604">18173200000000.2</Amplitude>
      <Amplitude index="-3453">40544500000000.2</Amplitude>
      <Amplitude index="907">49769300000000.2</Amplitude>
      <Amplitude index="-3587">-812399999999.765</Amplitude>
      <Amplitude index="611">6781400000000.24</Amplitude>
      <Amplitude index="2148">-18531999999999.8</Amplitude>
      <Amplitude index="951">45952700000000.2</Amplitude>
      <Amplitude index="-1846">-40481999999999.8</Amplitude>
      <Amplitude index="2872">35495600000000.2</Amplitude>
      <Amplitude index="3222">-11203799999999.8</Amplitude>
      <Amplitude index="-113">16198800000000.2</Amplitude>
      <Amplitude index="3686">-24706699999999.8</Amplitude>
      <Amplitude index="-4652">19414900000000.2</Amplitude>
      <Amplitude index="-1742">-29829999999999.8</Amplitude>
      <Amplitude index="-386">44868600000000.2</Amplitude>
      <Amplitude index="-1174">-14294099999999.8</Amplitude>
      <Amplitude index="-2276">-18078699999999.8</Amplitude>
      <Amplitude index="2837">-38982099999999.8</Amplitude>
      <Amplitude index="3505">15499100000000.2</Amplitude>
      <Amplitude index="-1967">40846400000000.2</Amplitude>
      <Amplitude index="-2364">2281500000000.23</Amplitude>
      <Amplitude index="-3266">3986900000000.23</Amplitude>
      <Amplitude index="117">-1530299999999.77</Amplitude>
      <Amplitude index="2402">184700000000.235</Amplitude>
      <Amplitude index="2304">-27873599999999.8</Amplitude>
      <Amplitude index="1202">-25875999999999.8</Amplitude>
      <Amplitude index="-3710">2125900000000.23</Amplitude>
      <Amplitude index="4115">49838500000000.2</Amplitude>
      <Amplitude index="3131">40180200000000.2</Amplitude>
      <Amplitude index="3707">39655100000000.2</Amplitude>
      <Amplitude index="1719">43955900000000.2</Amplitude>
      <Amplitude index="-4695">11239500000000.2</Amplitude>
      <Amplitude index="-3148">-14222199999999.8</Amplitude>
      <Amplitude index="-4577">-22414399999999.8</Amplitude>
      <Amplitude index="-3242">-29474699999999.8</Amplitude>
      <Amplitude index="-2553">-36779299999999.8</Amplitude>
      <Amplitude index="516">41370600000000.2</Amplitude>
      <Amplitude index="-3757">46584300000000.2</Amplitude>
      <Amplitude index="-114">-17127999999999.8</Amplitude>
      <Amplitude index="2412">5006400000000.24</Amplitude>
      <Amplitude index="4540">32398800000000.2</Amplitude>
      <Amplitude index="-287">-25988899999999.8</Amplitude>
      <Amplitude index="-1715">-35867099999999.8</Amplitude>
      <Amplitude index="-2720">25115600000000.2</Amplitude>
      <Amplitude index="-923">28529900000000.2</Amplitude>
      <Amplitude index="-2030">44870100000000.2</Amplitude>
      <Amplitude index="2843">-9321899999999.77</Amplitude>
      <Amplitude index="-2634">-17594799999999.8</Amplitude>
      <Amplitude index="-77">6097400000000.24</Amplitude>
      <Amplitude index="-1454">-34603299999999.8</Amplitude>
      <Amplitude index="-576">-3567399999999.77</Amplitude>
      <Amplitude index="-1740">2518000000000.23</Amplitude>
      <Amplitude index="4766">-1200099999999.77</Amplitude>
      <Amplitude index="1896">-28466099999999.8</Amplitude>
      <Amplitude index="-1031">44565300000000.2</Amplitude>
      <Amplitude index="98">-1557899999999.77</Amplitude>
      <Amplitude index="-3394">-17938199999999.8</Amplitude>
      <Amplitude index="4426">34786900000000.2</Amplitude>
      <Amplitude index="-317">29038800000000.2</Amplitude>
      <Amplitude index="-2161">-45608499999999.8</Amplitude>
      <Amplitude index="-3702">-20889099999999.8</Amplitude>
      <Amplitude index="3681">-10945899999999.8</Amplitude>
      <Amplitude index="4082">-20402499999999.8</Amplitude>
      <Amplitude index="-1521">-35515799999999.8</Amplitude>
      <Amplitude index="1771">30029800000000.2</Amplitude>
      <Amplitude index="-265">42764400000000.2</Amplitude>
      <Amplitude index="4741">8721900000000.24</Amplitude>
      <Amplitude index="-3924">-1036099999999.77</Amplitude>
      <Amplitude index="-20">44608400000000.2</Amplitude>
      <Amplitude index="-2365">34047900000000.2</Amplitude>
      <Amplitude index="-483">-48732599999999.8</Amplitude>
      <Amplitude index="-1458">-3793799999999.77</Amplitude>
      <Amplitude index="-2245">31261500000000.2</Amplitude>
      <Amplitude index="2881">-3630999999999.77</Amplitude>
      <Amplitude index="-358">-11126299999999.8</Amplitude>
      <Amplitude index="3558">20192600000000.2</Amplitude>
      <Amplitude index="-2229">-18929099999999.8</Amplitude>
      <Amplitude index="-4758">-34867699999999.8</Amplitude>
    </AmpTable>
    <NumRows>4355</NumRows>
    <NumCols>1806</NumCols>
    <FirstRow>448</FirstRow>
    <FirstCol>966</FirstCol>
    <FullImage>
      <NumRows>734</NumRows>
      <NumCols>-471</NumCols>
    </FullImage>
    <SCPPixel>
      <Row>-566</Row>
      <Col>-3829</Col>
    </SCPPixel>
    <ValidData size="-939">
      <Vertex index="79">
        <Row>-2116</Row>
        <Col>2716</Col>
      </Vertex>
      <Vertex index="1506">
        <Row>4563</Row>
        <Col>625</Col>
      </Vertex>
      <Vertex index="-1709">
        <Row>4069</Row>
        <Col>2576</Col>
      </Vertex>
    </ValidData>
  </ImageData>
  <GeoData>
    <EarthModel>WGS_84</EarthModel>
    <SCP>
      <ECF>
        <X>21850700000000.2</X>
        <Y>-28553899999999.8</Y>
        <Z>32045500000000.2</Z>
      </ECF>
      <LLH>
        <Lat>65.7781258039451</Lat>
        <Lon>54.9804658039451</Lon>
        <HAE>43153600000000.2</HAE>
      </LLH>
    </SCP>
    <ImageCorners>
      <ICP index="3:LRLC">
        <Lat>44395600000000.2</Lat>
        <Lon>18526500000000.2</Lon>
      </ICP>
      <ICP index="3:LRLC">
        <Lat>-7840199999999.76</Lat>
        <Lon>-20680199999999.8</Lon>
      </ICP>
      <ICP index="3:LRLC">
        <Lat>11098700000000.2</Lat>
        <Lon>31696400000000.2</Lon>
      </ICP>
      <ICP index="4:LRFC">
        <Lat>20334300000000.2</Lat>
        <Lon>30775700000000.2</Lon>
      </ICP>
    </ImageCorners>
    <ValidData size="-555">
      <Vertex index="-3944">
        <Lat>-7.80209419605491</Lat>
        <Lon>178.084265803945</Lon>
      </Vertex>
      <Vertex index="-482">
        <Lat>-48.1665541960549</Lat>
        <Lon>-167.798334196055</Lon>
      </Vertex>
      <Vertex index="-2355">
        <Lat>-17.9710141960549</Lat>
        <Lon>136.863545803945</Lon>
      </Vertex>
      <Vertex index="1224">
        <Lat>-82.3701541960549</Lat>
        <Lon>-138.519534196055</Lon>
      </Vertex>
    </ValidData>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Point>
        <Lat>25.8773458039451</Lat>
        <Lon>-153.248214196055</Lon>
      </Point>
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Polygon size="956">
        <Vertex index="1973">
          <Lat>62.5789858039451</Lat>
          <Lon>53.2431058039451</Lon>
        </Vertex>
        <Vertex index="2391">
          <Lat>87.1587058039451</Lat>
          <Lon>-178.924494196055</Lon>
        </Vertex>
        <Vertex index="706">
          <Lat>88.5110458039451</Lat>
          <Lon>111.193745803945</Lon>
        </Vertex>
      </Polygon>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
    <GeoInfo name="string">
      <Desc name="string">string</Desc>
      <Desc name="string">string</Desc>
      <Polygon size="1041">
        <Vertex index="3392">
          <Lat>-0.858954196054914</Lat>
          <Lon>-86.4782941960549</Lon>
        </Vertex>
        <Vertex index="629">
          <Lat>30.6257458039451</Lat>
          <Lon>-17.9425741960549</Lon>
        </Vertex>
        <Vertex index="1942">
          <Lat>27.6751858039451</Lat>
          <Lon>74.3776258039451</Lon>
        </Vertex>
      </Polygon>
      <GeoInfo name="string" />
      <GeoInfo name="string" />
      <GeoInfo name="string" />
    </GeoInfo>
  </GeoData>
  <Grid>
    <ImagePlane>OTHER</ImagePlane>
    <Type>XRGYCR</Type>
    <TimeCOAPoly order1="2533" order2="-2782">
      <Coef exponent1="4532" exponent2="-2122">-23411599999999.8</Coef>
      <Coef exponent1="4739" exponent2="1189">42445900000000.2</Coef>
      <Coef exponent1="3926" exponent2="2926">7617500000000.24</Coef>
    </TimeCOAPoly>
    <Row>
      <UVectECF>
        <X>-48862399999999.8</X>
        <Y>-12354999999999.8</Y>
        <Z>-40670399999999.8</Z>
      </UVectECF>
      <SS>10778800000000.2</SS>
      <ImpRespWid>-9395999999999.77</ImpRespWid>
      <Sgn>+1</Sgn>
      <ImpRespBW>17297400000000.2</ImpRespBW>
      <KCtr>-6284899999999.76</KCtr>
      <DeltaK1>47341400000000.2</DeltaK1>
      <DeltaK2>-30064499999999.8</DeltaK2>
      <WgtFunct size="893">
        <Wgt index="451">2528800000000.23</Wgt>
        <Wgt index="-2294">-43152999999999.8</Wgt>
      </WgtFunct>
    </Row>
    <Col>
      <UVectECF>
        <X>46067900000000.2</X>
        <Y>-38204799999999.8</Y>
        <Z>-37149099999999.8</Z>
      </UVectECF>
      <SS>38151500000000.2</SS>
      <ImpRespWid>38174800000000.2</ImpRespWid>
      <Sgn>-1</Sgn>
      <ImpRespBW>-24851499999999.8</ImpRespBW>
      <KCtr>-14452499999999.8</KCtr>
      <DeltaK1>40395000000000.2</DeltaK1>
      <DeltaK2>48907600000000.2</DeltaK2>
      <DeltaKCOAPoly order1="-3860" order2="-4113">
        <Coef exponent1="4705" exponent2="482">44359100000000.2</Coef>
      </DeltaKCOAPoly>
      <WgtFunct size="-3146">
        <Wgt index="-4425">-23456499999999.8</Wgt>
        <Wgt index="2197">6065600000000.24</Wgt>
      </WgtFunct>
    </Col>
  </Grid>
  <Timeline>
    <CollectStart>1992-06-02T03:33:53.29</CollectStart>
    <CollectDuration>41508500000000.2</CollectDuration>
    <IPP size="-1833">
      <Set index="4375">
        <TStart>32702000000000.2</TStart>
        <TEnd>47742900000000.2</TEnd>
        <IPPStart>4398</IPPStart>
        <IPPEnd>4733</IPPEnd>
        <IPPPoly order1="-3693">
          <Coef exponent1="4860">-745599999999.765</Coef>
        </IPPPoly>
      </Set>
    </IPP>
  </Timeline>
  <Position>
    <ARPPoly>
      <X order1="1411">
        <Coef exponent1="2261">-42371599999999.8</Coef>
      </X>
      <Y order1="1306">
        <Coef exponent1="3687">-2956299999999.77</Coef>
      </Y>
      <Z order1="-3800">
        <Coef exponent1="2336">-24446499999999.8</Coef>
      </Z>
    </ARPPoly>
    <GRPPoly>
      <X order1="4075">
        <Coef exponent1="-4738">-13243599999999.8</Coef>
      </X>
      <Y order1="-1924">
        <Coef exponent1="-672">-6174699999999.76</Coef>
      </Y>
      <Z order1="-875">
        <Coef exponent1="3242">43444000000000.2</Coef>
      </Z>
    </GRPPoly>
    <TxAPCPoly>
      <X order1="-1908">
        <Coef exponent1="3277">16027500000000.2</Coef>
      </X>
      <Y order1="2632">
        <Coef exponent1="-4388">-48450799999999.8</Coef>
      </Y>
      <Z order1="2707">
        <Coef exponent1="-3586">20317900000000.2</Coef>
      </Z>
    </TxAPCPoly>
    <RcvAPC size="-1415">
      <RcvAPCPoly index="3101">
        <X order1="-696">
          <Coef exponent1="-861">-18061599999999.8</Coef>
        </X>
        <Y order1="2992">
          <Coef exponent1="-1510">-43296799999999.8</Coef>
        </Y>
        <Z order1="2783">
          <Coef exponent1="-478">-25210799999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-2066">
        <X order1="-827">
          <Coef exponent1="-3843">-11781899999999.8</Coef>
        </X>
        <Y order1="-3789">
          <Coef exponent1="3359">-12122799999999.8</Coef>
        </Y>
        <Z order1="3203">
          <Coef exponent1="756">-44305799999999.8</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-1508">
        <X order1="-2380">
          <Coef exponent1="-783">-27019499999999.8</Coef>
        </X>
        <Y order1="-1368">
          <Coef exponent1="4063">31734000000000.2</Coef>
        </Y>
        <Z order1="1344">
          <Coef exponent1="3226">-3039399999999.77</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="2427">
        <X order1="4024">
          <Coef exponent1="-1890">47024700000000.2</Coef>
        </X>
        <Y order1="4651">
          <Coef exponent1="2903">-10472599999999.8</Coef>
        </Y>
        <Z order1="986">
          <Coef exponent1="454">42436800000000.2</Coef>
        </Z>
      </RcvAPCPoly>
      <RcvAPCPoly index="-3066">
        <X order1="-3521">
          <Coef exponent1="3472">-48142899999999.8</Coef>
        </X>
        <Y order1="3647">
          <Coef exponent1="747">-39400599999999.8</Coef>
        </Y>
        <Z order1="-1825">
          <Coef exponent1="308">-1810699999999.77</Coef>
        </Z>
      </RcvAPCPoly>
    </RcvAPC>
  </Position>
  <RadarCollection>
    <TxFrequency>
      <Min>21019000000000.2</Min>
      <Max>41868400000000.2</Max>
    </TxFrequency>
    <RefFreqIndex>-4199</RefFreqIndex>
    <Waveform size="3564">
      <WFParameters index="3569" />
      <WFParameters index="1843" />
    </Waveform>
    <TxPolarization>S</TxPolarization>
    <TxSequence size="2846">
      <TxStep index="1766" />
      <TxStep index="-2739" />
      <TxStep index="-4200" />
      <TxStep index="-3881" />
    </TxSequence>
    <RcvChannels size="-1572">
      <ChanParameters index="2929">
        <TxRcvPolarization>LHC:OTHER</TxRcvPolarization>
      </ChanParameters>
    </RcvChannels>
    <Area>
      <Corner>
        <ACP index="3">
          <Lat>-30.8356141960549</Lat>
          <Lon>163.080905803945</Lon>
          <HAE>-16973899999999.8</HAE>
        </ACP>
        <ACP index="2">
          <Lat>-49.0103941960549</Lat>
          <Lon>-157.818414196055</Lon>
          <HAE>38749800000000.2</HAE>
        </ACP>
        <ACP index="3">
          <Lat>-21.2320741960549</Lat>
          <Lon>-118.078014196055</Lon>
          <HAE>38578400000000.2</HAE>
        </ACP>
        <ACP index="4">
          <Lat>30.5537458039451</Lat>
          <Lon>179.746025803945</Lon>
          <HAE>33336800000000.2</HAE>
        </ACP>
      </Corner>
    </Area>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
    <Parameter name="string">string</Parameter>
  </RadarCollection>
  <ImageFormation>
    <RcvChanProc>
      <NumChanProc>3798</NumChanProc>
      <PRFScaleFactor>5898700000000.24</PRFScaleFactor>
      <ChanIndex>-2598</ChanIndex>
      <ChanIndex>-2404</ChanIndex>
      <ChanIndex>-675</ChanIndex>
      <ChanIndex>3381</ChanIndex>
    </RcvChanProc>
    <TxRcvPolarizationProc>LHC:OTHERb+1-</TxRcvPolarizationProc>
    <TStartProc>-49317299999999.8</TStartProc>
    <TEndProc>9716600000000.23</TEndProc>
    <TxFrequencyProc>
      <MinProc>-20988999999999.8</MinProc>
      <MaxProc>-35724199999999.8</MaxProc>
    </TxFrequencyProc>
    <SegmentIdentifier>string</SegmentIdentifier>
    <ImageFormAlgo>PFA</ImageFormAlgo>
    <STBeamComp>NO</STBeamComp>
    <ImageBeamComp>SV</ImageBeamComp>
    <AzAutofocus>GLOBAL</AzAutofocus>
    <RgAutofocus>NO</RgAutofocus>
    <Processing>
      <Type>string</Type>
      <Applied>true</Applied>
      <Parameter name="string">string</Parameter>
    </Processing>
    <Processing>
      <Type>string</Type>
      <Applied>true</Applied>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </Processing>
    <PolarizationCalibration>
      <DistortCorrectionApplied>0</DistortCorrectionApplied>
      <Distortion>
        <A>40654600000000.2</A>
        <F1>
          <Real>19937800000000.2</Real>
          <Imag>33656500000000.2</Imag>
        </F1>
        <Q1>
          <Real>10721500000000.2</Real>
          <Imag>49490200000000.2</Imag>
        </Q1>
        <Q2>
          <Real>16411800000000.2</Real>
          <Imag>-38688399999999.8</Imag>
        </Q2>
        <F2>
          <Real>-33692099999999.8</Real>
          <Imag>7940300000000.24</Imag>
        </F2>
        <Q3>
          <Real>26690700000000.2</Real>
          <Imag>-17733999999999.8</Imag>
        </Q3>
        <Q4>
          <Real>29034400000000.2</Real>
          <Imag>-44023799999999.8</Imag>
        </Q4>
      </Distortion>
    </PolarizationCalibration>
  </ImageFormation>
  <SCPCOA>
    <SCPTime>3318900000000.23</SCPTime>
    <ARPPos>
      <X>-7089699999999.76</X>
      <Y>26031100000000.2</Y>
      <Z>-46074199999999.8</Z>
    </ARPPos>
    <ARPVel>
      <X>23449300000000.2</X>
      <Y>37723800000000.2</Y>
      <Z>-49538699999999.8</Z>
    </ARPVel>
    <ARPAcc>
      <X>-18586099999999.8</X>
      <Y>-48628499999999.8</Y>
      <Z>-20948699999999.8</Z>
    </ARPAcc>
    <SideOfTrack>R</SideOfTrack>
    <SlantRange>-447399999999.765</SlantRange>
    <GroundRange>8756300000000.24</GroundRange>
    <DopplerConeAng>16178500000000.2</DopplerConeAng>
    <GrazeAng>55.8152158039451</GrazeAng>
    <IncidenceAng>84.9875458039451</IncidenceAng>
    <TwistAng>32.2857058039451</TwistAng>
    <SlopeAng>13.1157058039451</SlopeAng>
    <AzimAng>130.775585803945</AzimAng>
    <LayoverAng>329.227745803945</LayoverAng>
  </SCPCOA>
  <Radiometric>
    <NoiseLevel>
      <NoiseLevelType>RELATIVE</NoiseLevelType>
      <NoisePoly order1="-2910" order2="869">
        <Coef exponent1="-740" exponent2="-930">29379800000000.2</Coef>
      </NoisePoly>
    </NoiseLevel>
    <RCSSFPoly order1="1384" order2="4621">
      <Coef exponent1="-4371" exponent2="4660">-44385599999999.8</Coef>
    </RCSSFPoly>
    <SigmaZeroSFPoly order1="-3753" order2="-899">
      <Coef exponent1="2056" exponent2="607">-7640699999999.76</Coef>
      <Coef exponent1="-2696" exponent2="3137">-29681999999999.8</Coef>
      <Coef exponent1="3585" exponent2="-1590">-49279399999999.8</Coef>
    </SigmaZeroSFPoly>
    <BetaZeroSFPoly order1="-3507" order2="-2657">
      <Coef exponent1="-472" exponent2="-4086">17740000000000.2</Coef>
      <Coef exponent1="4031" exponent2="157">16246500000000.2</Coef>
    </BetaZeroSFPoly>
    <GammaZeroSFPoly order1="-4507" order2="47">
      <Coef exponent1="-3028" exponent2="3510">-3396999999999.77</Coef>
    </GammaZeroSFPoly>
  </Radiometric>
  <Antenna>
    <Tx>
      <XAxisPoly>
        <X order1="-3367">
          <Coef exponent1="-2772">4845400000000.24</Coef>
        </X>
        <Y order1="3189">
          <Coef exponent1="-4050">-48051699999999.8</Coef>
        </Y>
        <Z order1="132">
          <Coef exponent1="-2784">39442500000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="-4440">
          <Coef exponent1="1165">-14782599999999.8</Coef>
        </X>
        <Y order1="-3972">
          <Coef exponent1="-2868">19367700000000.2</Coef>
        </Y>
        <Z order1="-1392">
          <Coef exponent1="4677">36748500000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-38598999999999.8</FreqZero>
      <Array>
        <GainPoly order1="4384" order2="-3223">
          <Coef exponent1="-2265" exponent2="-708">26764400000000.2</Coef>
        </GainPoly>
        <PhasePoly order1="-2321" order2="959">
          <Coef exponent1="1682" exponent2="127">-8089299999999.76</Coef>
        </PhasePoly>
      </Array>
      <Elem>
        <GainPoly order1="-54" order2="1118">
          <Coef exponent1="-3059" exponent2="-954">20873800000000.2</Coef>
        </GainPoly>
        <PhasePoly order1="-2140" order2="-1459">
          <Coef exponent1="-3694" exponent2="-740">-31825699999999.8</Coef>
        </PhasePoly>
      </Elem>
    </Tx>
    <Rcv>
      <XAxisPoly>
        <X order1="-3285">
          <Coef exponent1="-4195">-28792999999999.8</Coef>
        </X>
        <Y order1="-450">
          <Coef exponent1="4580">-41855199999999.8</Coef>
        </Y>
        <Z order1="-2376">
          <Coef exponent1="4836">18104900000000.2</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="-865">
          <Coef exponent1="-347">35432000000000.2</Coef>
        </X>
        <Y order1="-2113">
          <Coef exponent1="1041">21633900000000.2</Coef>
        </Y>
        <Z order1="994">
          <Coef exponent1="4755">46462500000000.2</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-333299999999.765</FreqZero>
      <EB>
        <DCXPoly order1="-3206">
          <Coef exponent1="1902">41935400000000.2</Coef>
        </DCXPoly>
        <DCYPoly order1="2215">
          <Coef exponent1="-3045">9247500000000.23</Coef>
        </DCYPoly>
      </EB>
      <Array>
        <GainPoly order1="-1833" order2="2642">
          <Coef exponent1="4712" exponent2="-2991">-3190199999999.77</Coef>
        </GainPoly>
        <PhasePoly order1="-2583" order2="608">
          <Coef exponent1="-4119" exponent2="616">-48102799999999.8</Coef>
        </PhasePoly>
      </Array>
      <GainBSPoly order1="-398">
        <Coef exponent1="-2393">40848100000000.2</Coef>
      </GainBSPoly>
    </Rcv>
    <TwoWay>
      <XAxisPoly>
        <X order1="208">
          <Coef exponent1="233">19656600000000.2</Coef>
        </X>
        <Y order1="-2996">
          <Coef exponent1="3055">13017000000000.2</Coef>
        </Y>
        <Z order1="-1387">
          <Coef exponent1="538">-48223999999999.8</Coef>
        </Z>
      </XAxisPoly>
      <YAxisPoly>
        <X order1="-2715">
          <Coef exponent1="-601">-17029099999999.8</Coef>
        </X>
        <Y order1="-918">
          <Coef exponent1="-1898">4576600000000.24</Coef>
        </Y>
        <Z order1="4430">
          <Coef exponent1="1326">-21102099999999.8</Coef>
        </Z>
      </YAxisPoly>
      <FreqZero>-10844299999999.8</FreqZero>
      <EB>
        <DCXPoly order1="-4478">
          <Coef exponent1="-4299">39945300000000.2</Coef>
        </DCXPoly>
        <DCYPoly order1="4278">
          <Coef exponent1="1502">-12737099999999.8</Coef>
        </DCYPoly>
      </EB>
      <Array>
        <GainPoly order1="2959" order2="-924">
          <Coef exponent1="-2494" exponent2="3524">-46423199999999.8</Coef>
        </GainPoly>
        <PhasePoly order1="-2403" order2="4618">
          <Coef exponent1="-2619" exponent2="3557">-23521999999999.8</Coef>
        </PhasePoly>
      </Array>
      <Elem>
        <GainPoly order1="1177" order2="4693">
          <Coef exponent1="2579" exponent2="-2404">7126200000000.24</Coef>
        </GainPoly>
        <PhasePoly order1="-797" order2="-483">
          <Coef exponent1="-2043" exponent2="3543">-49057699999999.8</Coef>
        </PhasePoly>
      </Elem>
      <GainBSPoly order1="3367">
        <Coef exponent1="-3343">15215300000000.2</Coef>
      </GainBSPoly>
      <EBFreqShift>0</EBFreqShift>
      <MLFreqDilation>1</MLFreqDilation>
    </TwoWay>
  </Antenna>
  <ErrorStatistics>
    <CompositeSCP>
      <Rg>33405600000000.2</Rg>
      <Az>-38448299999999.8</Az>
      <RgAz>32078700000000.2</RgAz>
    </CompositeSCP>
    <Components>
      <PosVelErr>
        <Frame>RIC_ECI</Frame>
        <P1>8411000000000.24</P1>
        <P2>-13582299999999.8</P2>
        <P3>14033200000000.2</P3>
        <V1>8991600000000.23</V1>
        <V2>-11663299999999.8</V2>
        <V3>-28207599999999.8</V3>
      </PosVelErr>
      <RadarSensor>
        <RangeBias>-46222199999999.8</RangeBias>
      </RadarSensor>
    </Components>
    <Unmodeled>
      <Xrow>-11682299999999.8</Xrow>
      <Ycol>47135000000000.2</Ycol>
      <XrowYcol>-26824799999999.8</XrowYcol>
    </Unmodeled>
    <AdditionalParms>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
      <Parameter name="string">string</Parameter>
    </AdditionalParms>
  </ErrorStatistics>
  <MatchInfo>
    <NumMatchTypes>4352</NumMatchTypes>
    <MatchType index="-3785">
      <TypeID>string</TypeID>
      <NumMatchCollections>106</NumMatchCollections>
      <MatchCollection index="-4251">
        <CoreName>string</CoreName>
      </MatchCollection>
      <MatchCollection index="-790">
        <CoreName>string</CoreName>
      </MatchCollection>
      <MatchCollection index="2353">
        <CoreName>string</CoreName>
      </MatchCollection>
      <MatchCollection index="7">
        <CoreName>string</CoreName>
      </MatchCollection>
    </MatchType>
    <MatchType index="-3826">
      <TypeID>string</TypeID>
      <NumMatchCollections>2488</NumMatchCollections>
      <MatchCollection index="1258">
        <CoreName>string</CoreName>
      </MatchCollection>
      <MatchCollection index="1696">
        <CoreName>string</CoreName>
      </MatchCollection>
    </MatchType>
    <MatchType index="1199">
      <TypeID>string</TypeID>
      <CurrentIndex>-497</CurrentIndex>
      <NumMatchCollections>728</NumMatchCollections>
      <MatchCollection index="681">
        <CoreName>string</CoreName>
      </MatchCollection>
    </MatchType>
    <MatchType index="3106">
      <TypeID>string</TypeID>
      <CurrentIndex>2021</CurrentIndex>
      <NumMatchCollections>-1620</NumMatchCollections>
    </MatchType>
  </MatchInfo>
  <PFA>
    <FPN>
      <X>31626500000000.2</X>
      <Y>-25002999999999.8</Y>
      <Z>-28178199999999.8</Z>
    </FPN>
    <IPN>
      <X>42929200000000.2</X>
      <Y>44532100000000.2</Y>
      <Z>25609100000000.2</Z>
    </IPN>
    <PolarAngRefTime>-42929899999999.8</PolarAngRefTime>
    <PolarAngPoly order1="-3772">
      <Coef exponent1="-520">-26162499999999.8</Coef>
    </PolarAngPoly>
    <SpatialFreqSFPoly order1="3615">
      <Coef exponent1="-4017">49174500000000.2</Coef>
    </SpatialFreqSFPoly>
    <Krg1>-38271499999999.8</Krg1>
    <Krg2>16180700000000.2</Krg2>
    <Kaz1>15022800000000.2</Kaz1>
    <Kaz2>25184800000000.2</Kaz2>
    <STDeskew>
      <Applied>1</Applied>
      <STDSPhasePoly order1="4718" order2="2252">
        <Coef exponent1="-4770" exponent2="-1993">-7193099999999.76</Coef>
      </STDSPhasePoly>
    </STDeskew>
  </PFA>
</SICD>"#;
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
#[test]
fn test_generated_xml_collection_info() {
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
    <Parameter name="string">string</Parameter>
  </CollectionInfo>"#;
    assert!(match from_str::<CollectionInfo>(xml) {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
#[test]
fn test_generated_xml_image_creation() {
    let xml = r#"
  <ImageCreation>
    <Application>string</Application>
    <DateTime>1994-04-05T10:27:33.85</DateTime>
    <Site>string</Site>
    <Profile>string</Profile>
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
fn test_generated_xml_image_data() {
    let xml = r#"
  <ImageData>
    <PixelType>RE16I_IM16I</PixelType>
    <AmpTable size="-1490">
      <Amplitude index="2273">162200000000.235</Amplitude>
      <Amplitude index="555">-43949299999999.8</Amplitude>
      <Amplitude index="1752">2033500000000.23</Amplitude>
      <Amplitude index="782">-49253599999999.8</Amplitude>
      <Amplitude index="1410">39259000000000.2</Amplitude>
      <Amplitude index="3550">-8400399999999.76</Amplitude>
      <Amplitude index="1711">49045500000000.2</Amplitude>
      <Amplitude index="12">35525600000000.2</Amplitude>
      <Amplitude index="2757">39219800000000.2</Amplitude>
      <Amplitude index="-4778">-7486199999999.76</Amplitude>
      <Amplitude index="1095">-48358399999999.8</Amplitude>
      <Amplitude index="-3986">-12052999999999.8</Amplitude>
      <Amplitude index="483">24281200000000.2</Amplitude>
      <Amplitude index="-4323">-18821199999999.8</Amplitude>
      <Amplitude index="2107">43491900000000.2</Amplitude>
      <Amplitude index="-619">31852100000000.2</Amplitude>
      <Amplitude index="-2066">22261400000000.2</Amplitude>
      <Amplitude index="1017">-42300599999999.8</Amplitude>
      <Amplitude index="4210">-6153699999999.76</Amplitude>
      <Amplitude index="964">12312700000000.2</Amplitude>
      <Amplitude index="3536">-4628299999999.76</Amplitude>
      <Amplitude index="4206">45800300000000.2</Amplitude>
      <Amplitude index="-3825">27920800000000.2</Amplitude>
      <Amplitude index="1812">33673500000000.2</Amplitude>
      <Amplitude index="-3454">-2826699999999.77</Amplitude>
      <Amplitude index="2818">-46796499999999.8</Amplitude>
      <Amplitude index="-1712">-32673299999999.8</Amplitude>
      <Amplitude index="-943">21088100000000.2</Amplitude>
      <Amplitude index="-997">-32401699999999.8</Amplitude>
      <Amplitude index="122">43235900000000.2</Amplitude>
      <Amplitude index="-474">-23361999999999.8</Amplitude>
      <Amplitude index="-2032">20606000000000.2</Amplitude>
      <Amplitude index="-455">-46354899999999.8</Amplitude>
      <Amplitude index="-3774">44849100000000.2</Amplitude>
      <Amplitude index="-1113">-7581399999999.76</Amplitude>
      <Amplitude index="4343">-16276299999999.8</Amplitude>
      <Amplitude index="-2042">-10096399999999.8</Amplitude>
      <Amplitude index="715">-34422799999999.8</Amplitude>
      <Amplitude index="-4041">-35663799999999.8</Amplitude>
      <Amplitude index="-2381">26905000000000.2</Amplitude>
      <Amplitude index="-4384">-26901799999999.8</Amplitude>
      <Amplitude index="-3428">-26103199999999.8</Amplitude>
      <Amplitude index="-3469">-9393899999999.77</Amplitude>
      <Amplitude index="-103">-37990399999999.8</Amplitude>
      <Amplitude index="-1831">39083500000000.2</Amplitude>
      <Amplitude index="1767">24499900000000.2</Amplitude>
      <Amplitude index="4262">16405500000000.2</Amplitude>
      <Amplitude index="-3295">8723900000000.24</Amplitude>
      <Amplitude index="-3430">-28550299999999.8</Amplitude>
      <Amplitude index="35">-41890399999999.8</Amplitude>
      <Amplitude index="1566">23271700000000.2</Amplitude>
      <Amplitude index="-520">23038900000000.2</Amplitude>
      <Amplitude index="375">-5542399999999.76</Amplitude>
      <Amplitude index="2363">42977800000000.2</Amplitude>
      <Amplitude index="1017">-25006299999999.8</Amplitude>
      <Amplitude index="1150">-24310699999999.8</Amplitude>
      <Amplitude index="4141">24319700000000.2</Amplitude>
      <Amplitude index="3708">-27836599999999.8</Amplitude>
      <Amplitude index="-3908">-44220699999999.8</Amplitude>
      <Amplitude index="530">-45156899999999.8</Amplitude>
      <Amplitude index="467">-49746299999999.8</Amplitude>
      <Amplitude index="1316">-209799999999.765</Amplitude>
      <Amplitude index="2474">-31072699999999.8</Amplitude>
      <Amplitude index="-890">13172800000000.2</Amplitude>
      <Amplitude index="-2715">48427800000000.2</Amplitude>
      <Amplitude index="4988">38136100000000.2</Amplitude>
      <Amplitude index="1399">-31917599999999.8</Amplitude>
      <Amplitude index="-3875">-17111599999999.8</Amplitude>
      <Amplitude index="2830">-7317699999999.76</Amplitude>
      <Amplitude index="2015">20852800000000.2</Amplitude>
      <Amplitude index="1698">5992800000000.24</Amplitude>
      <Amplitude index="183">-43303299999999.8</Amplitude>
      <Amplitude index="-2242">-8015599999999.76</Amplitude>
      <Amplitude index="3309">-31698299999999.8</Amplitude>
      <Amplitude index="2933">44887500000000.2</Amplitude>
      <Amplitude index="-219">-40083799999999.8</Amplitude>
      <Amplitude index="1615">-4492099999999.76</Amplitude>
      <Amplitude index="344">15403700000000.2</Amplitude>
      <Amplitude index="-3989">45009400000000.2</Amplitude>
      <Amplitude index="4829">-15181299999999.8</Amplitude>
      <Amplitude index="-4664">-39547799999999.8</Amplitude>
      <Amplitude index="2013">11747000000000.2</Amplitude>
      <Amplitude index="-2489">23366100000000.2</Amplitude>
      <Amplitude index="1170">23323700000000.2</Amplitude>
      <Amplitude index="1307">4187100000000.23</Amplitude>
      <Amplitude index="-614">18234400000000.2</Amplitude>
      <Amplitude index="-1437">34442700000000.2</Amplitude>
      <Amplitude index="-1214">48681700000000.2</Amplitude>
      <Amplitude index="-158">6461900000000.24</Amplitude>
      <Amplitude index="-2779">-17239699999999.8</Amplitude>
      <Amplitude index="-1417">-27199499999999.8</Amplitude>
      <Amplitude index="3384">-22034899999999.8</Amplitude>
      <Amplitude index="62">39965500000000.2</Amplitude>
      <Amplitude index="-2802">-31522699999999.8</Amplitude>
      <Amplitude index="1465">-4156299999999.77</Amplitude>
      <Amplitude index="-2722">33288900000000.2</Amplitude>
      <Amplitude index="-561">-14667899999999.8</Amplitude>
      <Amplitude index="1750">6528800000000.24</Amplitude>
      <Amplitude index="3587">40087600000000.2</Amplitude>
      <Amplitude index="3158">4215100000000.23</Amplitude>
      <Amplitude index="3028">-40229999999999.8</Amplitude>
      <Amplitude index="523">-24857299999999.8</Amplitude>
      <Amplitude index="102">29572700000000.2</Amplitude>
      <Amplitude index="2429">31711400000000.2</Amplitude>
      <Amplitude index="-4235">4753300000000.24</Amplitude>
      <Amplitude index="-3302">3651000000000.23</Amplitude>
      <Amplitude index="2280">15533700000000.2</Amplitude>
      <Amplitude index="4899">30554800000000.2</Amplitude>
      <Amplitude index="-2339">-7840099999999.76</Amplitude>
      <Amplitude index="-3887">-14853599999999.8</Amplitude>
      <Amplitude index="138">-6773199999999.76</Amplitude>
      <Amplitude index="-4132">-32771699999999.8</Amplitude>
      <Amplitude index="-1859">10576000000000.2</Amplitude>
      <Amplitude index="-2615">-49703699999999.8</Amplitude>
      <Amplitude index="-3305">31333100000000.2</Amplitude>
      <Amplitude index="-3719">8331300000000.24</Amplitude>
      <Amplitude index="2488">17993700000000.2</Amplitude>
      <Amplitude index="248">-23936499999999.8</Amplitude>
      <Amplitude index="1757">8700100000000.24</Amplitude>
      <Amplitude index="2694">21047900000000.2</Amplitude>
      <Amplitude index="-3432">-9729499999999.77</Amplitude>
      <Amplitude index="-3917">-40105999999999.8</Amplitude>
      <Amplitude index="-2114">19124300000000.2</Amplitude>
      <Amplitude index="-3951">28854400000000.2</Amplitude>
      <Amplitude index="-1365">36942600000000.2</Amplitude>
      <Amplitude index="-2008">-6292299999999.76</Amplitude>
      <Amplitude index="2896">-3561699999999.77</Amplitude>
      <Amplitude index="-4717">-12946499999999.8</Amplitude>
      <Amplitude index="-4891">-11998799999999.8</Amplitude>
      <Amplitude index="4373">40446400000000.2</Amplitude>
      <Amplitude index="573">23991300000000.2</Amplitude>
      <Amplitude index="1477">-23687399999999.8</Amplitude>
      <Amplitude index="-806">8653400000000.24</Amplitude>
      <Amplitude index="2877">-45193199999999.8</Amplitude>
      <Amplitude index="-3695">22921600000000.2</Amplitude>
      <Amplitude index="-3702">17909200000000.2</Amplitude>
      <Amplitude index="1522">-9921599999999.77</Amplitude>
      <Amplitude index="-3053">-38886499999999.8</Amplitude>
      <Amplitude index="-1760">48782300000000.2</Amplitude>
      <Amplitude index="3837">12287300000000.2</Amplitude>
      <Amplitude index="9">-5008199999999.76</Amplitude>
      <Amplitude index="1395">-19997499999999.8</Amplitude>
      <Amplitude index="141">19106300000000.2</Amplitude>
      <Amplitude index="2937">-21561199999999.8</Amplitude>
      <Amplitude index="1516">-34573599999999.8</Amplitude>
      <Amplitude index="-2503">-20430299999999.8</Amplitude>
      <Amplitude index="1497">36496200000000.2</Amplitude>
      <Amplitude index="-3468">-8313599999999.76</Amplitude>
      <Amplitude index="2550">34518700000000.2</Amplitude>
      <Amplitude index="1795">20205600000000.2</Amplitude>
      <Amplitude index="4035">-44318499999999.8</Amplitude>
      <Amplitude index="1580">13425300000000.2</Amplitude>
      <Amplitude index="2396">12010200000000.2</Amplitude>
      <Amplitude index="2849">-11117199999999.8</Amplitude>
      <Amplitude index="-2303">41717400000000.2</Amplitude>
      <Amplitude index="-4534">-47688699999999.8</Amplitude>
      <Amplitude index="-37">-18556299999999.8</Amplitude>
      <Amplitude index="-964">-39265399999999.8</Amplitude>
      <Amplitude index="-3996">-15236399999999.8</Amplitude>
      <Amplitude index="2490">22833000000000.2</Amplitude>
      <Amplitude index="2928">334900000000.235</Amplitude>
      <Amplitude index="-1036">47627300000000.2</Amplitude>
      <Amplitude index="-205">33414100000000.2</Amplitude>
      <Amplitude index="-4706">28721000000000.2</Amplitude>
      <Amplitude index="-2524">27782100000000.2</Amplitude>
      <Amplitude index="-1439">-2123399999999.77</Amplitude>
      <Amplitude index="-1917">-31836299999999.8</Amplitude>
      <Amplitude index="2194">-5593999999999.76</Amplitude>
      <Amplitude index="2919">-49476099999999.8</Amplitude>
      <Amplitude index="604">39403300000000.2</Amplitude>
      <Amplitude index="4062">-9512899999999.77</Amplitude>
      <Amplitude index="-4853">23445200000000.2</Amplitude>
      <Amplitude index="-3923">-27341599999999.8</Amplitude>
      <Amplitude index="2994">-16474799999999.8</Amplitude>
      <Amplitude index="-386">-45413199999999.8</Amplitude>
      <Amplitude index="-1836">-9259399999999.77</Amplitude>
      <Amplitude index="-4038">45121600000000.2</Amplitude>
      <Amplitude index="4092">-9984499999999.77</Amplitude>
      <Amplitude index="1604">18173200000000.2</Amplitude>
      <Amplitude index="-3453">40544500000000.2</Amplitude>
      <Amplitude index="907">49769300000000.2</Amplitude>
      <Amplitude index="-3587">-812399999999.765</Amplitude>
      <Amplitude index="611">6781400000000.24</Amplitude>
      <Amplitude index="2148">-18531999999999.8</Amplitude>
      <Amplitude index="951">45952700000000.2</Amplitude>
      <Amplitude index="-1846">-40481999999999.8</Amplitude>
      <Amplitude index="2872">35495600000000.2</Amplitude>
      <Amplitude index="3222">-11203799999999.8</Amplitude>
      <Amplitude index="-113">16198800000000.2</Amplitude>
      <Amplitude index="3686">-24706699999999.8</Amplitude>
      <Amplitude index="-4652">19414900000000.2</Amplitude>
      <Amplitude index="-1742">-29829999999999.8</Amplitude>
      <Amplitude index="-386">44868600000000.2</Amplitude>
      <Amplitude index="-1174">-14294099999999.8</Amplitude>
      <Amplitude index="-2276">-18078699999999.8</Amplitude>
      <Amplitude index="2837">-38982099999999.8</Amplitude>
      <Amplitude index="3505">15499100000000.2</Amplitude>
      <Amplitude index="-1967">40846400000000.2</Amplitude>
      <Amplitude index="-2364">2281500000000.23</Amplitude>
      <Amplitude index="-3266">3986900000000.23</Amplitude>
      <Amplitude index="117">-1530299999999.77</Amplitude>
      <Amplitude index="2402">184700000000.235</Amplitude>
      <Amplitude index="2304">-27873599999999.8</Amplitude>
      <Amplitude index="1202">-25875999999999.8</Amplitude>
      <Amplitude index="-3710">2125900000000.23</Amplitude>
      <Amplitude index="4115">49838500000000.2</Amplitude>
      <Amplitude index="3131">40180200000000.2</Amplitude>
      <Amplitude index="3707">39655100000000.2</Amplitude>
      <Amplitude index="1719">43955900000000.2</Amplitude>
      <Amplitude index="-4695">11239500000000.2</Amplitude>
      <Amplitude index="-3148">-14222199999999.8</Amplitude>
      <Amplitude index="-4577">-22414399999999.8</Amplitude>
      <Amplitude index="-3242">-29474699999999.8</Amplitude>
      <Amplitude index="-2553">-36779299999999.8</Amplitude>
      <Amplitude index="516">41370600000000.2</Amplitude>
      <Amplitude index="-3757">46584300000000.2</Amplitude>
      <Amplitude index="-114">-17127999999999.8</Amplitude>
      <Amplitude index="2412">5006400000000.24</Amplitude>
      <Amplitude index="4540">32398800000000.2</Amplitude>
      <Amplitude index="-287">-25988899999999.8</Amplitude>
      <Amplitude index="-1715">-35867099999999.8</Amplitude>
      <Amplitude index="-2720">25115600000000.2</Amplitude>
      <Amplitude index="-923">28529900000000.2</Amplitude>
      <Amplitude index="-2030">44870100000000.2</Amplitude>
      <Amplitude index="2843">-9321899999999.77</Amplitude>
      <Amplitude index="-2634">-17594799999999.8</Amplitude>
      <Amplitude index="-77">6097400000000.24</Amplitude>
      <Amplitude index="-1454">-34603299999999.8</Amplitude>
      <Amplitude index="-576">-3567399999999.77</Amplitude>
      <Amplitude index="-1740">2518000000000.23</Amplitude>
      <Amplitude index="4766">-1200099999999.77</Amplitude>
      <Amplitude index="1896">-28466099999999.8</Amplitude>
      <Amplitude index="-1031">44565300000000.2</Amplitude>
      <Amplitude index="98">-1557899999999.77</Amplitude>
      <Amplitude index="-3394">-17938199999999.8</Amplitude>
      <Amplitude index="4426">34786900000000.2</Amplitude>
      <Amplitude index="-317">29038800000000.2</Amplitude>
      <Amplitude index="-2161">-45608499999999.8</Amplitude>
      <Amplitude index="-3702">-20889099999999.8</Amplitude>
      <Amplitude index="3681">-10945899999999.8</Amplitude>
      <Amplitude index="4082">-20402499999999.8</Amplitude>
      <Amplitude index="-1521">-35515799999999.8</Amplitude>
      <Amplitude index="1771">30029800000000.2</Amplitude>
      <Amplitude index="-265">42764400000000.2</Amplitude>
      <Amplitude index="4741">8721900000000.24</Amplitude>
      <Amplitude index="-3924">-1036099999999.77</Amplitude>
      <Amplitude index="-20">44608400000000.2</Amplitude>
      <Amplitude index="-2365">34047900000000.2</Amplitude>
      <Amplitude index="-483">-48732599999999.8</Amplitude>
      <Amplitude index="-1458">-3793799999999.77</Amplitude>
      <Amplitude index="-2245">31261500000000.2</Amplitude>
      <Amplitude index="2881">-3630999999999.77</Amplitude>
      <Amplitude index="-358">-11126299999999.8</Amplitude>
      <Amplitude index="3558">20192600000000.2</Amplitude>
      <Amplitude index="-2229">-18929099999999.8</Amplitude>
      <Amplitude index="-4758">-34867699999999.8</Amplitude>
    </AmpTable>
    <NumRows>4355</NumRows>
    <NumCols>1806</NumCols>
    <FirstRow>448</FirstRow>
    <FirstCol>966</FirstCol>
    <FullImage>
      <NumRows>734</NumRows>
      <NumCols>-471</NumCols>
    </FullImage>
    <SCPPixel>
      <Row>-566</Row>
      <Col>-3829</Col>
    </SCPPixel>
    <ValidData size="-939">
      <Vertex index="79">
        <Row>-2116</Row>
        <Col>2716</Col>
      </Vertex>
      <Vertex index="1506">
        <Row>4563</Row>
        <Col>625</Col>
      </Vertex>
      <Vertex index="-1709">
        <Row>4069</Row>
        <Col>2576</Col>
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
fn test_generated_xml_pixel_type() {
    let xml = r#"
    <PixelType>RE16I_IM16I</PixelType>
        "#;
    assert!(match from_str::<PixelType>(xml) {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
#[test]
fn test_generated_xml_amp_table() {
    let xml = r#"
    <AmpTable size="-1490">
      <Amplitude index="2273">162200000000.235</Amplitude>
      <Amplitude index="555">-43949299999999.8</Amplitude>
      <Amplitude index="1752">2033500000000.23</Amplitude>
      <Amplitude index="782">-49253599999999.8</Amplitude>
      <Amplitude index="1410">39259000000000.2</Amplitude>
      <Amplitude index="3550">-8400399999999.76</Amplitude>
      <Amplitude index="1711">49045500000000.2</Amplitude>
      <Amplitude index="12">35525600000000.2</Amplitude>
      <Amplitude index="2757">39219800000000.2</Amplitude>
      <Amplitude index="-4778">-7486199999999.76</Amplitude>
      <Amplitude index="1095">-48358399999999.8</Amplitude>
      <Amplitude index="-3986">-12052999999999.8</Amplitude>
      <Amplitude index="483">24281200000000.2</Amplitude>
      <Amplitude index="-4323">-18821199999999.8</Amplitude>
      <Amplitude index="2107">43491900000000.2</Amplitude>
      <Amplitude index="-619">31852100000000.2</Amplitude>
      <Amplitude index="-2066">22261400000000.2</Amplitude>
      <Amplitude index="1017">-42300599999999.8</Amplitude>
      <Amplitude index="4210">-6153699999999.76</Amplitude>
      <Amplitude index="964">12312700000000.2</Amplitude>
      <Amplitude index="3536">-4628299999999.76</Amplitude>
      <Amplitude index="4206">45800300000000.2</Amplitude>
      <Amplitude index="-3825">27920800000000.2</Amplitude>
      <Amplitude index="1812">33673500000000.2</Amplitude>
      <Amplitude index="-3454">-2826699999999.77</Amplitude>
      <Amplitude index="2818">-46796499999999.8</Amplitude>
      <Amplitude index="-1712">-32673299999999.8</Amplitude>
      <Amplitude index="-943">21088100000000.2</Amplitude>
      <Amplitude index="-997">-32401699999999.8</Amplitude>
      <Amplitude index="122">43235900000000.2</Amplitude>
      <Amplitude index="-474">-23361999999999.8</Amplitude>
      <Amplitude index="-2032">20606000000000.2</Amplitude>
      <Amplitude index="-455">-46354899999999.8</Amplitude>
      <Amplitude index="-3774">44849100000000.2</Amplitude>
      <Amplitude index="-1113">-7581399999999.76</Amplitude>
      <Amplitude index="4343">-16276299999999.8</Amplitude>
      <Amplitude index="-2042">-10096399999999.8</Amplitude>
      <Amplitude index="715">-34422799999999.8</Amplitude>
      <Amplitude index="-4041">-35663799999999.8</Amplitude>
      <Amplitude index="-2381">26905000000000.2</Amplitude>
      <Amplitude index="-4384">-26901799999999.8</Amplitude>
      <Amplitude index="-3428">-26103199999999.8</Amplitude>
      <Amplitude index="-3469">-9393899999999.77</Amplitude>
      <Amplitude index="-103">-37990399999999.8</Amplitude>
      <Amplitude index="-1831">39083500000000.2</Amplitude>
      <Amplitude index="1767">24499900000000.2</Amplitude>
      <Amplitude index="4262">16405500000000.2</Amplitude>
      <Amplitude index="-3295">8723900000000.24</Amplitude>
      <Amplitude index="-3430">-28550299999999.8</Amplitude>
      <Amplitude index="35">-41890399999999.8</Amplitude>
      <Amplitude index="1566">23271700000000.2</Amplitude>
      <Amplitude index="-520">23038900000000.2</Amplitude>
      <Amplitude index="375">-5542399999999.76</Amplitude>
      <Amplitude index="2363">42977800000000.2</Amplitude>
      <Amplitude index="1017">-25006299999999.8</Amplitude>
      <Amplitude index="1150">-24310699999999.8</Amplitude>
      <Amplitude index="4141">24319700000000.2</Amplitude>
      <Amplitude index="3708">-27836599999999.8</Amplitude>
      <Amplitude index="-3908">-44220699999999.8</Amplitude>
      <Amplitude index="530">-45156899999999.8</Amplitude>
      <Amplitude index="467">-49746299999999.8</Amplitude>
      <Amplitude index="1316">-209799999999.765</Amplitude>
      <Amplitude index="2474">-31072699999999.8</Amplitude>
      <Amplitude index="-890">13172800000000.2</Amplitude>
      <Amplitude index="-2715">48427800000000.2</Amplitude>
      <Amplitude index="4988">38136100000000.2</Amplitude>
      <Amplitude index="1399">-31917599999999.8</Amplitude>
      <Amplitude index="-3875">-17111599999999.8</Amplitude>
      <Amplitude index="2830">-7317699999999.76</Amplitude>
      <Amplitude index="2015">20852800000000.2</Amplitude>
      <Amplitude index="1698">5992800000000.24</Amplitude>
      <Amplitude index="183">-43303299999999.8</Amplitude>
      <Amplitude index="-2242">-8015599999999.76</Amplitude>
      <Amplitude index="3309">-31698299999999.8</Amplitude>
      <Amplitude index="2933">44887500000000.2</Amplitude>
      <Amplitude index="-219">-40083799999999.8</Amplitude>
      <Amplitude index="1615">-4492099999999.76</Amplitude>
      <Amplitude index="344">15403700000000.2</Amplitude>
      <Amplitude index="-3989">45009400000000.2</Amplitude>
      <Amplitude index="4829">-15181299999999.8</Amplitude>
      <Amplitude index="-4664">-39547799999999.8</Amplitude>
      <Amplitude index="2013">11747000000000.2</Amplitude>
      <Amplitude index="-2489">23366100000000.2</Amplitude>
      <Amplitude index="1170">23323700000000.2</Amplitude>
      <Amplitude index="1307">4187100000000.23</Amplitude>
      <Amplitude index="-614">18234400000000.2</Amplitude>
      <Amplitude index="-1437">34442700000000.2</Amplitude>
      <Amplitude index="-1214">48681700000000.2</Amplitude>
      <Amplitude index="-158">6461900000000.24</Amplitude>
      <Amplitude index="-2779">-17239699999999.8</Amplitude>
      <Amplitude index="-1417">-27199499999999.8</Amplitude>
      <Amplitude index="3384">-22034899999999.8</Amplitude>
      <Amplitude index="62">39965500000000.2</Amplitude>
      <Amplitude index="-2802">-31522699999999.8</Amplitude>
      <Amplitude index="1465">-4156299999999.77</Amplitude>
      <Amplitude index="-2722">33288900000000.2</Amplitude>
      <Amplitude index="-561">-14667899999999.8</Amplitude>
      <Amplitude index="1750">6528800000000.24</Amplitude>
      <Amplitude index="3587">40087600000000.2</Amplitude>
      <Amplitude index="3158">4215100000000.23</Amplitude>
      <Amplitude index="3028">-40229999999999.8</Amplitude>
      <Amplitude index="523">-24857299999999.8</Amplitude>
      <Amplitude index="102">29572700000000.2</Amplitude>
      <Amplitude index="2429">31711400000000.2</Amplitude>
      <Amplitude index="-4235">4753300000000.24</Amplitude>
      <Amplitude index="-3302">3651000000000.23</Amplitude>
      <Amplitude index="2280">15533700000000.2</Amplitude>
      <Amplitude index="4899">30554800000000.2</Amplitude>
      <Amplitude index="-2339">-7840099999999.76</Amplitude>
      <Amplitude index="-3887">-14853599999999.8</Amplitude>
      <Amplitude index="138">-6773199999999.76</Amplitude>
      <Amplitude index="-4132">-32771699999999.8</Amplitude>
      <Amplitude index="-1859">10576000000000.2</Amplitude>
      <Amplitude index="-2615">-49703699999999.8</Amplitude>
      <Amplitude index="-3305">31333100000000.2</Amplitude>
      <Amplitude index="-3719">8331300000000.24</Amplitude>
      <Amplitude index="2488">17993700000000.2</Amplitude>
      <Amplitude index="248">-23936499999999.8</Amplitude>
      <Amplitude index="1757">8700100000000.24</Amplitude>
      <Amplitude index="2694">21047900000000.2</Amplitude>
      <Amplitude index="-3432">-9729499999999.77</Amplitude>
      <Amplitude index="-3917">-40105999999999.8</Amplitude>
      <Amplitude index="-2114">19124300000000.2</Amplitude>
      <Amplitude index="-3951">28854400000000.2</Amplitude>
      <Amplitude index="-1365">36942600000000.2</Amplitude>
      <Amplitude index="-2008">-6292299999999.76</Amplitude>
      <Amplitude index="2896">-3561699999999.77</Amplitude>
      <Amplitude index="-4717">-12946499999999.8</Amplitude>
      <Amplitude index="-4891">-11998799999999.8</Amplitude>
      <Amplitude index="4373">40446400000000.2</Amplitude>
      <Amplitude index="573">23991300000000.2</Amplitude>
      <Amplitude index="1477">-23687399999999.8</Amplitude>
      <Amplitude index="-806">8653400000000.24</Amplitude>
      <Amplitude index="2877">-45193199999999.8</Amplitude>
      <Amplitude index="-3695">22921600000000.2</Amplitude>
      <Amplitude index="-3702">17909200000000.2</Amplitude>
      <Amplitude index="1522">-9921599999999.77</Amplitude>
      <Amplitude index="-3053">-38886499999999.8</Amplitude>
      <Amplitude index="-1760">48782300000000.2</Amplitude>
      <Amplitude index="3837">12287300000000.2</Amplitude>
      <Amplitude index="9">-5008199999999.76</Amplitude>
      <Amplitude index="1395">-19997499999999.8</Amplitude>
      <Amplitude index="141">19106300000000.2</Amplitude>
      <Amplitude index="2937">-21561199999999.8</Amplitude>
      <Amplitude index="1516">-34573599999999.8</Amplitude>
      <Amplitude index="-2503">-20430299999999.8</Amplitude>
      <Amplitude index="1497">36496200000000.2</Amplitude>
      <Amplitude index="-3468">-8313599999999.76</Amplitude>
      <Amplitude index="2550">34518700000000.2</Amplitude>
      <Amplitude index="1795">20205600000000.2</Amplitude>
      <Amplitude index="4035">-44318499999999.8</Amplitude>
      <Amplitude index="1580">13425300000000.2</Amplitude>
      <Amplitude index="2396">12010200000000.2</Amplitude>
      <Amplitude index="2849">-11117199999999.8</Amplitude>
      <Amplitude index="-2303">41717400000000.2</Amplitude>
      <Amplitude index="-4534">-47688699999999.8</Amplitude>
      <Amplitude index="-37">-18556299999999.8</Amplitude>
      <Amplitude index="-964">-39265399999999.8</Amplitude>
      <Amplitude index="-3996">-15236399999999.8</Amplitude>
      <Amplitude index="2490">22833000000000.2</Amplitude>
      <Amplitude index="2928">334900000000.235</Amplitude>
      <Amplitude index="-1036">47627300000000.2</Amplitude>
      <Amplitude index="-205">33414100000000.2</Amplitude>
      <Amplitude index="-4706">28721000000000.2</Amplitude>
      <Amplitude index="-2524">27782100000000.2</Amplitude>
      <Amplitude index="-1439">-2123399999999.77</Amplitude>
      <Amplitude index="-1917">-31836299999999.8</Amplitude>
      <Amplitude index="2194">-5593999999999.76</Amplitude>
      <Amplitude index="2919">-49476099999999.8</Amplitude>
      <Amplitude index="604">39403300000000.2</Amplitude>
      <Amplitude index="4062">-9512899999999.77</Amplitude>
      <Amplitude index="-4853">23445200000000.2</Amplitude>
      <Amplitude index="-3923">-27341599999999.8</Amplitude>
      <Amplitude index="2994">-16474799999999.8</Amplitude>
      <Amplitude index="-386">-45413199999999.8</Amplitude>
      <Amplitude index="-1836">-9259399999999.77</Amplitude>
      <Amplitude index="-4038">45121600000000.2</Amplitude>
      <Amplitude index="4092">-9984499999999.77</Amplitude>
      <Amplitude index="1604">18173200000000.2</Amplitude>
      <Amplitude index="-3453">40544500000000.2</Amplitude>
      <Amplitude index="907">49769300000000.2</Amplitude>
      <Amplitude index="-3587">-812399999999.765</Amplitude>
      <Amplitude index="611">6781400000000.24</Amplitude>
      <Amplitude index="2148">-18531999999999.8</Amplitude>
      <Amplitude index="951">45952700000000.2</Amplitude>
      <Amplitude index="-1846">-40481999999999.8</Amplitude>
      <Amplitude index="2872">35495600000000.2</Amplitude>
      <Amplitude index="3222">-11203799999999.8</Amplitude>
      <Amplitude index="-113">16198800000000.2</Amplitude>
      <Amplitude index="3686">-24706699999999.8</Amplitude>
      <Amplitude index="-4652">19414900000000.2</Amplitude>
      <Amplitude index="-1742">-29829999999999.8</Amplitude>
      <Amplitude index="-386">44868600000000.2</Amplitude>
      <Amplitude index="-1174">-14294099999999.8</Amplitude>
      <Amplitude index="-2276">-18078699999999.8</Amplitude>
      <Amplitude index="2837">-38982099999999.8</Amplitude>
      <Amplitude index="3505">15499100000000.2</Amplitude>
      <Amplitude index="-1967">40846400000000.2</Amplitude>
      <Amplitude index="-2364">2281500000000.23</Amplitude>
      <Amplitude index="-3266">3986900000000.23</Amplitude>
      <Amplitude index="117">-1530299999999.77</Amplitude>
      <Amplitude index="2402">184700000000.235</Amplitude>
      <Amplitude index="2304">-27873599999999.8</Amplitude>
      <Amplitude index="1202">-25875999999999.8</Amplitude>
      <Amplitude index="-3710">2125900000000.23</Amplitude>
      <Amplitude index="4115">49838500000000.2</Amplitude>
      <Amplitude index="3131">40180200000000.2</Amplitude>
      <Amplitude index="3707">39655100000000.2</Amplitude>
      <Amplitude index="1719">43955900000000.2</Amplitude>
      <Amplitude index="-4695">11239500000000.2</Amplitude>
      <Amplitude index="-3148">-14222199999999.8</Amplitude>
      <Amplitude index="-4577">-22414399999999.8</Amplitude>
      <Amplitude index="-3242">-29474699999999.8</Amplitude>
      <Amplitude index="-2553">-36779299999999.8</Amplitude>
      <Amplitude index="516">41370600000000.2</Amplitude>
      <Amplitude index="-3757">46584300000000.2</Amplitude>
      <Amplitude index="-114">-17127999999999.8</Amplitude>
      <Amplitude index="2412">5006400000000.24</Amplitude>
      <Amplitude index="4540">32398800000000.2</Amplitude>
      <Amplitude index="-287">-25988899999999.8</Amplitude>
      <Amplitude index="-1715">-35867099999999.8</Amplitude>
      <Amplitude index="-2720">25115600000000.2</Amplitude>
      <Amplitude index="-923">28529900000000.2</Amplitude>
      <Amplitude index="-2030">44870100000000.2</Amplitude>
      <Amplitude index="2843">-9321899999999.77</Amplitude>
      <Amplitude index="-2634">-17594799999999.8</Amplitude>
      <Amplitude index="-77">6097400000000.24</Amplitude>
      <Amplitude index="-1454">-34603299999999.8</Amplitude>
      <Amplitude index="-576">-3567399999999.77</Amplitude>
      <Amplitude index="-1740">2518000000000.23</Amplitude>
      <Amplitude index="4766">-1200099999999.77</Amplitude>
      <Amplitude index="1896">-28466099999999.8</Amplitude>
      <Amplitude index="-1031">44565300000000.2</Amplitude>
      <Amplitude index="98">-1557899999999.77</Amplitude>
      <Amplitude index="-3394">-17938199999999.8</Amplitude>
      <Amplitude index="4426">34786900000000.2</Amplitude>
      <Amplitude index="-317">29038800000000.2</Amplitude>
      <Amplitude index="-2161">-45608499999999.8</Amplitude>
      <Amplitude index="-3702">-20889099999999.8</Amplitude>
      <Amplitude index="3681">-10945899999999.8</Amplitude>
      <Amplitude index="4082">-20402499999999.8</Amplitude>
      <Amplitude index="-1521">-35515799999999.8</Amplitude>
      <Amplitude index="1771">30029800000000.2</Amplitude>
      <Amplitude index="-265">42764400000000.2</Amplitude>
      <Amplitude index="4741">8721900000000.24</Amplitude>
      <Amplitude index="-3924">-1036099999999.77</Amplitude>
      <Amplitude index="-20">44608400000000.2</Amplitude>
      <Amplitude index="-2365">34047900000000.2</Amplitude>
      <Amplitude index="-483">-48732599999999.8</Amplitude>
      <Amplitude index="-1458">-3793799999999.77</Amplitude>
      <Amplitude index="-2245">31261500000000.2</Amplitude>
      <Amplitude index="2881">-3630999999999.77</Amplitude>
      <Amplitude index="-358">-11126299999999.8</Amplitude>
      <Amplitude index="3558">20192600000000.2</Amplitude>
      <Amplitude index="-2229">-18929099999999.8</Amplitude>
      <Amplitude index="-4758">-34867699999999.8</Amplitude>
    </AmpTable>
        "#;
    assert!(match from_str::<AmpTable>(xml) {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
#[test]
fn test_generated_xml_amplitude() {
    let xml = r#"
      <Amplitude index="2273">162200000000.235</Amplitude>
        "#;
    assert!(match from_str::<Amplitude>(xml) {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
#[test]
fn test_generated_xml_valid_data() {
    let xml = r#"
    <ValidData size="-939">
      <Vertex index="79">
        <Row>-2116</Row>
        <Col>2716</Col>
      </Vertex>
      <Vertex index="1506">
        <Row>4563</Row>
        <Col>625</Col>
      </Vertex>
      <Vertex index="-1709">
        <Row>4069</Row>
        <Col>2576</Col>
      </Vertex>
    </ValidData>
        "#;
    assert!(match from_str::<ValidDataRC>(xml) {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}
#[test]
fn test_generated_xml_rcv_chan_proc() {
    let xml = r#"
    <RcvChanProc>
      <NumChanProc>3798</NumChanProc>
      <PRFScaleFactor>5898700000000.24</PRFScaleFactor>
      <ChanIndex>-2598</ChanIndex>
      <ChanIndex>-2404</ChanIndex>
      <ChanIndex>-675</ChanIndex>
      <ChanIndex>3381</ChanIndex>
    </RcvChanProc>
        "#;
    assert!(match from_str::<RcvChanProc>(xml) {
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
