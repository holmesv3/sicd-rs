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

    let mut print_count = 0;
    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"CollectionInfo" => {
                    println!("Found CollectionInfo");
                    let span = reader.read_to_end(QName(b"CollectionInfo")).unwrap();
                    match from_str::<sicd_rs::v1_3_0::CollectionInfo>(&format!("<CollectionInfo>{}</CollectionInfo>", &xml[span.clone()])) {
                        Ok(_) => {
                            println!("Parsed CollectionInfo");
                        }
                        Err(err) => {
                            dbg!(&xml[span]);
                            dbg!(err);
                        }
                    }
                }
                b"Antenna" => {
                    println!("Found antenna");
                    let span = reader.read_to_end(QName(b"Antenna")).unwrap();
                    match from_str::<sicd_rs::v1_3_0::Antenna>(&format!("<Antenna>{}</Antenna>", &xml[span.clone()])) {
                        Ok(_) => {
                            println!("Parsed Antenna");
                        }
                        Err(err) => {
                            dbg!(err);
                        }
                    }
                }
                other => {
                    if print_count < 3 {
                        print_count += 1;
                        println!("Found other tag {}", std::str::from_utf8(other).unwrap());
                    }
                }
            },
            _ => (),
        }
    }
}
