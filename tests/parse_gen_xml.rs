use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

#[test]
fn test_generated_130_xml() {
    let xml = include_str!("sicd_meta_130.xml");
    let sicd_meta = from_str::<sicd_rs::v1_3_0::SicdMeta>(xml);
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
    let xml = include_str!("sicd_meta_050.xml");
    let sicd_meta = from_str::<sicd_rs::dep::v0_5_0::SicdMeta>(xml);
    assert!(match sicd_meta {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}

macro_rules! create_component_test {
    ($($x:ident),*) => {
#[test]
fn parse_components_test_macro() {
    let xml = include_str!("sicd_meta_130.xml");
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
                    let qname = QName(stringify!($x).as_bytes());
                    let span = reader.read_to_end(qname).unwrap();
                    let result = from_str::<sicd_rs::v1_3_0::$x>(&format!(
                        "<{0}>{1}</{0}>",
                        stringify!($x),
                        &xml[span]
                    ));
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
                    }
                }
            },
            _ => (),
        }
    }
        }

    };
}

create_component_test!(
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
    PFA
);
