use dade::{json_dump, json_load};
use std::fs::File;
use std::io::{BufReader, Read};

macro_rules! test_success {
    ($test_name: ident, $file: literal) => {
        #[allow(non_snake_case)]
        #[test]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_ok());
            assert_eq!(json_dump(&ret.as_ref().unwrap(), false), contents);
        }
    };
}

macro_rules! test_success_with_correct {
    ($test_name: ident, $file: literal, $correct: literal) => {
        #[allow(non_snake_case)]
        #[test]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_ok());
            assert_eq!(json_dump(&ret.as_ref().unwrap(), false), $correct);
        }
    };
}

macro_rules! test_fail {
    ($test_name: ident, $file: literal) => {
        #[allow(non_snake_case)]
        #[test]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_err());
        }
    };
}

test_success!(test_number_1, "./tests/data/transform/number_1.0.json");
test_success!(
    test_number_1_05,
    "./tests/data/transform/number_1.000000000000000005.json"
);
test_success!(
    test_number_1_09,
    "./tests/data/transform/number_10000000000000000999.json"
);
test_success!(
    test_number_1_00,
    "./tests/data/transform/number_1000000000000000.json"
);
test_success!(test_number_1_0, "./tests/data/transform/number_1.0.json");
test_success!(test_number_1e6, "./tests/data/transform/number_1e6.json");
test_success!(
    test_number_1e999,
    "./tests/data/transform/number_1e-999.json"
);
//TODO; When fix dump, we will fix a correct.
test_success_with_correct!(
    test_object_key_nfc_nfd,
    "./tests/data/transform/object_key_nfc_nfd.json",
    "{\"é\":\"NFC\",\"e\\u0301\":\"NFD\"}"
);
//TODO; When fix dump, we will fix a correct.
test_success_with_correct!(
    test_object_key_nfd_nfc,
    "./tests/data/transform/object_key_nfd_nfc.json",
    "{\"e\\u0301\":\"NFD\",\"é\":\"NFC\"}"
);
test_fail!(
    test_object_same_key_different_values,
    "./tests/data/transform/object_same_key_different_values.json"
);
test_fail!(
    test_object_same_key_same_value,
    "./tests/data/transform/object_same_key_same_value.json"
);
test_fail!(
    test_object_same_key_unclear_values,
    "./tests/data/transform/object_same_key_unclear_values.json"
);
test_fail!(
    test_string_1_escaped_invalid_codepoint,
    "./tests/data/transform/string_1_escaped_invalid_codepoint.json"
);
// stream did not contain valid UTF-8
// test_fail!(test_string_1_invalid_codepoint, "./tests/data/transform/string_1_invalid_codepoint.json");
test_fail!(
    test_string_2_escaped_invalid_codepoints,
    "./tests/data/transform/string_2_escaped_invalid_codepoints.json"
);
// stream did not contain valid UTF-8
// test_fail!(test_string_2_invalid_codepoints, "./tests/data/transform/string_2_invalid_codepoints.json");
test_fail!(
    test_string_3_escaped_invalid_codepoints,
    "./tests/data/transform/string_3_escaped_invalid_codepoints.json"
);
// stream did not contain valid UTF-8
// test_fail!(test_string_3_invalid_codepoints, "./tests/data/transform/string_3_invalid_codepoints.json");
test_success!(
    test_string_with_escaped_NULL,
    "./tests/data/transform/string_with_escaped_NULL.json"
);
