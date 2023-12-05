use core::panic;

use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

#[test]
fn test_generated_130_xml() {
    let xml = include_str!("sicd_meta_130_00.xml");
    parse_130_components(xml);
    parse_xml::<sicd_rs::v1_3_0::SicdMeta>(xml);
    let xml = include_str!("sicd_meta_130_01.xml");
    parse_130_components(xml);
    parse_xml::<sicd_rs::v1_3_0::SicdMeta>(xml);
    let xml = include_str!("sicd_meta_130_02.xml");
    parse_130_components(xml);
    parse_xml::<sicd_rs::v1_3_0::SicdMeta>(xml);
    let xml = include_str!("sicd_meta_130_03.xml");
    parse_130_components(xml);
    parse_xml::<sicd_rs::v1_3_0::SicdMeta>(xml);
}

#[test]
fn parse_050_rgazcomp() {
    let xml = include_str!("sicd_meta_050_00.xml");
    let sicd_meta = from_str::<sicd_rs::dep::v0_5_0::SicdMeta>(xml).unwrap();
    match sicd_meta.rg_az_comp {
        Some(val) => {
            dbg!(val);
        }
        None => {
            panic!("Expected something");
        }
    }
    let xml = include_str!("sicd_meta_050_01.xml");
    let sicd_meta = from_str::<sicd_rs::dep::v0_5_0::SicdMeta>(xml).unwrap();
    match sicd_meta.rg_az_comp {
        Some(val) => {
            dbg!(val);
            panic!("Expected nothing")
        }
        None => (),
    }
}

#[test]
fn parse_131_rgazcomp() {
    let xml = include_str!("sicd_meta_130_03.xml");
    let sicd_meta = from_str::<sicd_rs::v1_3_0::SicdMeta>(xml).unwrap();
    match sicd_meta.rg_az_comp {
        Some(val) => {
            dbg!(val);
        }
        None => {
            panic!("Expectd something");
        }
    }
    let xml = include_str!("sicd_meta_130_00.xml");
    let sicd_meta = from_str::<sicd_rs::v1_3_0::SicdMeta>(xml).unwrap();
    match sicd_meta.rg_az_comp {
        Some(val) => {
            dbg!(val);
            panic!("Expected nothing")
        }
        None => (),
    }
}

fn parse_xml<'a, T>(xml: &'a str)
where
    T: serde::de::Deserialize<'a>,
{
    let sicd_meta = from_str::<T>(xml);
    assert!(match sicd_meta {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}

#[test]
fn test_generated_050_xml() {
    let xml = include_str!("sicd_meta_050_00.xml");
    parse_050_components(xml);
    parse_xml::<sicd_rs::dep::v0_5_0::SicdMeta>(xml);
    let xml = include_str!("sicd_meta_050_01.xml");
    parse_050_components(xml);
    parse_xml::<sicd_rs::dep::v0_5_0::SicdMeta>(xml);
}

#[test]
fn test_generated_040_xml() {
    let xml = include_str!("sicd_meta_040_00.xml");
    parse_040_components(xml);
    parse_xml::<sicd_rs::dep::v0_4_0::SicdMeta>(xml);
    let xml = include_str!("sicd_meta_040_01.xml");
    parse_040_components(xml);
    parse_xml::<sicd_rs::dep::v0_4_0::SicdMeta>(xml);
}

macro_rules! create_130_component_test {
    ($($x:ident),*) => {
fn parse_130_components(xml: &str) {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut print_count = 0;
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match std::str::from_utf8(e.name().as_ref()).unwrap() {
                $(
                stringify!($x) => {
                    let attributes = e.attributes().map( |a| {
                        let a=a.unwrap();
                        format!(
                            "{}=\"{}\"",
                            std::str::from_utf8(a.key.into_inner()).unwrap(),
                            std::str::from_utf8(&a.value).unwrap())}).collect::<Vec<_>>().join(" ");
                    let qname = QName(stringify!($x).as_bytes());
                    let span = reader.read_to_end(qname).unwrap();
                    let xml_sub = format!(
                        "<{0} {2}>{1}</{0}>",
                        stringify!($x),
                        &xml[span],
                        attributes
                    );
                    let result = from_str::<sicd_rs::v1_3_0::$x>(&xml_sub);
                    match result {
                        Ok(_) => println!("Parsed {}", stringify!($x)),
                        Err(err) => {
                            panic!("{} did not parse: {}", stringify!($x), err);
                        }
                    }
                }
            )*
                other => {
                    if print_count < 3 {
                        print_count += 1;
                        println!("Found other tag {}", other);
                        assert_eq!("SICD", other);
                    }
                }
            },
            _ => (),
        }
    }
        }

    };
}

create_130_component_test!(
    CollectionInfo,
    ImageCreation,
    ImageData,
    GeoData,
    Grid,
    Timeline,
    Position,
    RadarCollection,
    ImageFormation,
    SCPCOA,
    Radiometric,
    Antenna,
    ErrorStatistics,
    MatchInfo,
    PFA,
    RMA,
    RgAzComp
);

macro_rules! create_050_component_test {
    ($($x:ident),*) => {
fn parse_050_components(xml: &str) {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut print_count = 0;
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match std::str::from_utf8(e.name().as_ref()).unwrap() {
                $(
                stringify!($x) => {
                    let attributes = e.attributes().map( |a| {
                        let a=a.unwrap();
                        format!(
                            "{}=\"{}\"",
                            std::str::from_utf8(a.key.into_inner()).unwrap(),
                            std::str::from_utf8(&a.value).unwrap())}).collect::<Vec<_>>().join(" ");
                    let qname = QName(stringify!($x).as_bytes());
                    let span = reader.read_to_end(qname).unwrap();
                    let xml_sub = format!(
                        "<{0} {2}>{1}</{0}>",
                        stringify!($x),
                        &xml[span],
                        attributes
                    );
                    let result = from_str::<sicd_rs::dep::v0_5_0::$x>(&xml_sub);
                    match result {
                        Ok(_) => println!("Parsed {}", stringify!($x)),
                        Err(err) => {
                            dbg!(xml_sub);
                            dbg!(attributes);
                            panic!("{} did not parse: {}", stringify!($x), err);
                        }
                    }
                }
            )*
                other => {
                    if print_count < 3 {
                        print_count += 1;
                        println!("Found other tag {}", other);
                        assert_eq!("SICD", other);
                    }
                }
            },
            _ => (),
        }
    }
        }

    };
}

create_050_component_test!(
    CollectionInfo,
    ImageCreation,
    ImageData,
    GeoData,
    Grid,
    Timeline,
    Position,
    RadarCollection,
    ImageFormation,
    SCPCOA,
    Radiometric,
    Antenna,
    ErrorStatistics,
    MatchInfo,
    PFA,
    RGAZCOMP,
    RMA
);

macro_rules! create_040_component_test {
    ($($x:ident),*) => {
fn parse_040_components(xml: &str) {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut print_count = 0;
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match std::str::from_utf8(e.name().as_ref()).unwrap() {
                $(
                stringify!($x) => {
                    let attributes = e.attributes().map( |a| {
                        let a=a.unwrap();
                        format!(
                            "{}=\"{}\"",
                            std::str::from_utf8(a.key.into_inner()).unwrap(),
                            std::str::from_utf8(&a.value).unwrap())}).collect::<Vec<_>>().join(" ");
                    let qname = QName(stringify!($x).as_bytes());
                    let span = reader.read_to_end(qname).unwrap();
                    let xml_sub = format!(
                        "<{0} {2}>{1}</{0}>",
                        stringify!($x),
                        &xml[span],
                        attributes
                    );
                    let result = from_str::<sicd_rs::dep::v0_4_0::$x>(&xml_sub);
                    match result {
                        Ok(_) => println!("Parsed {}", stringify!($x)),
                        Err(err) => {
                            dbg!(xml_sub);
                            dbg!(attributes);
                            panic!("{} did not parse: {}", stringify!($x), err);
                        }
                    }
                }
            )*
                other => {
                    if print_count < 3 {
                        print_count += 1;
                        println!("Found other tag {}", other);
                        assert_eq!("SICD", other);
                    }
                }
            },
            _ => (),
        }
    }
        }

    };
}

create_040_component_test!(
    CollectionInfo,
    ImageCreation,
    ImageData,
    GeoData,
    Grid,
    Timeline,
    Position,
    RadarCollection,
    ImageFormation,
    SCPCOA,
    Radiometric,
    Antenna,
    ErrorStatistics,
    MatchInfo,
    PFA,
    RGAZCOMP,
    RMA
);
