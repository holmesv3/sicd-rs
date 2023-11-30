use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use sicd_rs::v1_3_0::SicdMeta;

#[test]
fn test_generated_xml() {
    let xml = include_str!("sicd_meta_130.xml");
    let sicd_meta = from_str::<SicdMeta>(xml);
    assert!(match sicd_meta {
        Ok(_) => true,
        Err(err) => {
            dbg!(err);
            false
        }
    });
}

#[test]
fn parse_components_test() {
    let xml = include_str!("sicd_meta_130.xml");
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Antenna" => {
                    println!("Found antenna");
                    let span = reader.read_to_end(QName(b"Antenna")).unwrap();
                    match from_str::<sicd_rs::v1_3_0::Antenna>(&xml[span]) {
                        Ok(_) => println!("Parsed Antenna"),
                        Err(err) => {
                            dbg!(err);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
