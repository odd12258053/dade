use dade::{json_load, JsonValue};
use std::fs::File;
use std::io::{BufReader, Read};

#[test]
fn null() {
    let ret = json_load("null");
    assert!(if let Ok(JsonValue::Null) = ret {
        true
    } else {
        false
    });
}

#[test]
fn bool() {
    let ret = json_load("true");
    assert!(if let Ok(JsonValue::Bool(val)) = ret {
        val == true
    } else {
        false
    });

    let ret = json_load("false");
    assert!(if let Ok(JsonValue::Bool(val)) = ret {
        val == false
    } else {
        false
    });
}

#[test]
fn int() {
    let ret = json_load("1");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: usize = num.parse().unwrap();
        val == 1
    } else {
        false
    });

    let ret = json_load("0");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: usize = num.parse().unwrap();
        val == 0
    } else {
        false
    });

    let ret = json_load("-1");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: isize = num.parse().unwrap();
        val == -1
    } else {
        false
    });
}

#[test]
fn float() {
    let ret = json_load("1.25");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: f32 = num.parse().unwrap();
        val == 1.25
    } else {
        false
    });

    let ret = json_load("0.0");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: f32 = num.parse().unwrap();
        val == 0.0
    } else {
        false
    });

    let ret = json_load("-0.2");
    assert!(if let Ok(JsonValue::Number(num)) = ret {
        let val: f32 = num.parse().unwrap();
        val == -0.2
    } else {
        false
    });
}

#[test]
fn string() {
    let ret = json_load("\"\"");
    assert!(if let Ok(JsonValue::String(val)) = ret {
        val == ""
    } else {
        false
    });

    let ret = json_load("\"abc\"");
    assert!(if let Ok(JsonValue::String(val)) = ret {
        val == "abc"
    } else {
        false
    });
}

#[test]
fn array() {
    let ret = json_load("[]");
    assert!(if let Ok(JsonValue::Array(arr)) = ret {
        arr.is_empty()
    } else {
        false
    });

    let ret = json_load("[\"abc\"]");
    assert!(if let Ok(JsonValue::Array(arr)) = ret {
        let mut it = arr.iter();
        assert!(if let Some(JsonValue::String(val)) = it.next() {
            val == "abc"
        } else {
            false
        });
        it.next().is_none()
    } else {
        false
    });

    let ret = json_load("[1, 2]");
    assert!(if let Ok(JsonValue::Array(arr)) = ret {
        let mut it = arr.iter();
        assert!(if let Some(JsonValue::Number(num)) = it.next() {
            let val: isize = num.parse().unwrap();
            val == 1
        } else {
            false
        });
        assert!(if let Some(JsonValue::Number(num)) = it.next() {
            let val: isize = num.parse().unwrap();
            val == 2
        } else {
            false
        });
        it.next().is_none()
    } else {
        false
    });
}

#[test]
fn object() {
    let ret = json_load("{}");
    assert!(if let Ok(JsonValue::Object(dict)) = ret {
        dict.is_empty()
    } else {
        false
    });

    let ret = json_load("{\"abc\": 1}");
    assert!(if let Ok(JsonValue::Object(dict)) = ret {
        assert!(if let Some(JsonValue::Number(num)) = dict.get("abc") {
            let val: isize = num.parse().unwrap();
            val == 1
        } else {
            false
        });
        dict.len() == 1
    } else {
        false
    });

    let ret = json_load("{\"abc\": 1, \"foo\": \"bar\"}");
    assert!(if let Ok(JsonValue::Object(dict)) = ret {
        assert!(if let Some(JsonValue::Number(num)) = dict.get("abc") {
            let val: isize = num.parse().unwrap();
            val == 1
        } else {
            false
        });
        assert!(if let Some(JsonValue::String(val)) = dict.get("foo") {
            val == "bar"
        } else {
            false
        });
        dict.len() == 2
    } else {
        false
    });
}

// #[test]
// fn fail01()
// {
//     let file = File::open("./tests/data/jsonchecker/fail01.json").unwrap();
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents).unwrap();
//     let ret = json_load(contents.as_str());
//     assert!(ret.is_err());
// }

#[test]
fn fail02() {
    let file = File::open("./tests/data/jsonchecker/fail02.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail03() {
    let file = File::open("./tests/data/jsonchecker/fail03.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail04() {
    let file = File::open("./tests/data/jsonchecker/fail04.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail05() {
    let file = File::open("./tests/data/jsonchecker/fail05.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail06() {
    let file = File::open("./tests/data/jsonchecker/fail06.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail07() {
    let file = File::open("./tests/data/jsonchecker/fail07.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail08() {
    let file = File::open("./tests/data/jsonchecker/fail08.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail09() {
    let file = File::open("./tests/data/jsonchecker/fail09.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail10() {
    let file = File::open("./tests/data/jsonchecker/fail10.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail11() {
    let file = File::open("./tests/data/jsonchecker/fail11.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail12() {
    let file = File::open("./tests/data/jsonchecker/fail12.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail13() {
    let file = File::open("./tests/data/jsonchecker/fail13.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail14() {
    let file = File::open("./tests/data/jsonchecker/fail14.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail15() {
    let file = File::open("./tests/data/jsonchecker/fail15.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail16() {
    let file = File::open("./tests/data/jsonchecker/fail16.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail17() {
    let file = File::open("./tests/data/jsonchecker/fail17.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

// #[test]
// fn fail18()
// {
//     let file = File::open("./tests/data/jsonchecker/fail18.json").unwrap();
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents).unwrap();
//     let ret = json_load(contents.as_str());
//     assert!(ret.is_err());
// }

#[test]
fn fail19() {
    let file = File::open("./tests/data/jsonchecker/fail19.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail20() {
    let file = File::open("./tests/data/jsonchecker/fail20.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail21() {
    let file = File::open("./tests/data/jsonchecker/fail21.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail22() {
    let file = File::open("./tests/data/jsonchecker/fail22.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail23() {
    let file = File::open("./tests/data/jsonchecker/fail23.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail24() {
    let file = File::open("./tests/data/jsonchecker/fail24.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail25() {
    let file = File::open("./tests/data/jsonchecker/fail25.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail26() {
    let file = File::open("./tests/data/jsonchecker/fail26.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail27() {
    let file = File::open("./tests/data/jsonchecker/fail27.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail28() {
    let file = File::open("./tests/data/jsonchecker/fail28.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail29() {
    let file = File::open("./tests/data/jsonchecker/fail29.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail30() {
    let file = File::open("./tests/data/jsonchecker/fail30.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail31() {
    let file = File::open("./tests/data/jsonchecker/fail31.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail32() {
    let file = File::open("./tests/data/jsonchecker/fail32.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn fail33() {
    let file = File::open("./tests/data/jsonchecker/fail33.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_err());
}

#[test]
fn pass01() {
    let file = File::open("./tests/data/jsonchecker/pass01.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_ok(), "{}", ret.err().unwrap());
}

#[test]
fn pass02() {
    let file = File::open("./tests/data/jsonchecker/pass02.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_ok(), "{}", ret.err().unwrap());
}

#[test]
fn pass03() {
    let file = File::open("./tests/data/jsonchecker/pass03.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let ret = json_load(contents.as_str());
    assert!(ret.is_ok(), "{}", ret.err().unwrap());
}
