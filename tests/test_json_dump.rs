use std::fs::File;
use std::io::{BufReader, Read};
use dade::{json_dump, json_load};

#[test]
fn unicode() {
    let file = File::open("./tests/data/unicode.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_ok(), "{}", ret.err().unwrap());
    let json_value = ret.unwrap();
    let dumped = json_dump(&json_value, false);
    assert_eq!(dumped, contents);
}

#[test]
fn unicode_with_ascii() {
    let file = File::open("./tests/data/unicode_with_ascii.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_ok(), "{}", ret.err().unwrap());
    let json_value = ret.unwrap();
    let dumped = json_dump(&json_value, true);
    assert_eq!(dumped, contents);
}
