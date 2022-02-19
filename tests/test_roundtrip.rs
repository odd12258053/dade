use dade::{json_dump, json_load};
use std::fs::File;
use std::io::{BufReader, Read};

macro_rules! test {
    ($test_name: ident, $file: literal) => {
        #[test]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_ok());
            assert_eq!(json_dump(&ret.as_ref().unwrap(), false), contents);
            assert_eq!(json_dump(&ret.as_ref().unwrap(), true), contents);
        }
    };
}

test!(test_roundtrip01, "./tests/data/roundtrip/roundtrip01.json");
test!(test_roundtrip02, "./tests/data/roundtrip/roundtrip02.json");
test!(test_roundtrip03, "./tests/data/roundtrip/roundtrip03.json");
test!(test_roundtrip04, "./tests/data/roundtrip/roundtrip04.json");
test!(test_roundtrip05, "./tests/data/roundtrip/roundtrip05.json");
test!(test_roundtrip06, "./tests/data/roundtrip/roundtrip06.json");
test!(test_roundtrip07, "./tests/data/roundtrip/roundtrip07.json");
test!(test_roundtrip08, "./tests/data/roundtrip/roundtrip08.json");
test!(test_roundtrip09, "./tests/data/roundtrip/roundtrip09.json");
test!(test_roundtrip10, "./tests/data/roundtrip/roundtrip10.json");
test!(test_roundtrip11, "./tests/data/roundtrip/roundtrip11.json");
test!(test_roundtrip12, "./tests/data/roundtrip/roundtrip12.json");
test!(test_roundtrip13, "./tests/data/roundtrip/roundtrip13.json");
test!(test_roundtrip14, "./tests/data/roundtrip/roundtrip14.json");
test!(test_roundtrip15, "./tests/data/roundtrip/roundtrip15.json");
test!(test_roundtrip16, "./tests/data/roundtrip/roundtrip16.json");
test!(test_roundtrip17, "./tests/data/roundtrip/roundtrip17.json");
test!(test_roundtrip18, "./tests/data/roundtrip/roundtrip18.json");
test!(test_roundtrip19, "./tests/data/roundtrip/roundtrip19.json");
test!(test_roundtrip20, "./tests/data/roundtrip/roundtrip20.json");
test!(test_roundtrip21, "./tests/data/roundtrip/roundtrip21.json");
test!(test_roundtrip22, "./tests/data/roundtrip/roundtrip22.json");
test!(test_roundtrip23, "./tests/data/roundtrip/roundtrip23.json");
test!(test_roundtrip24, "./tests/data/roundtrip/roundtrip24.json");
test!(test_roundtrip25, "./tests/data/roundtrip/roundtrip25.json");
test!(test_roundtrip26, "./tests/data/roundtrip/roundtrip26.json");
test!(test_roundtrip27, "./tests/data/roundtrip/roundtrip27.json");
