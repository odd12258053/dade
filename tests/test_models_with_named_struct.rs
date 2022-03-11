use paste::paste;

use dade::{model, Error, Model, Result};

macro_rules! success_parse_model {
    ($model: ident, $in_string:literal, $out_string:literal) => {
        let ret = $model::parse($in_string);
        assert!(ret.is_ok());
        let empty = ret.unwrap();
        assert_eq!(empty.json(false), $out_string);
    };
}

#[test]
fn test_empty_model() {
    #[model]
    struct TestModel;
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, "{}", "{}");
    success_parse_model!(TestModel, "{\"abc\":1}", "{}");

    assert!(TestModel::parse("[]").is_err());
    assert!(TestModel::parse("1").is_err());
    assert!(TestModel::parse("1.0").is_err());
    assert!(TestModel::parse("null").is_err());
    assert!(TestModel::parse("true").is_err());
    assert!(TestModel::parse("false").is_err());
    assert!(TestModel::parse("\"abc\"").is_err());
}

#[test]
fn test_null_model() {
    #[model]
    struct TestModel {
        v1: (),
        #[field(alias = "c2", default = null)]
        v2: (),
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"properties\":{\
                        \"c2\":{\"default\":null,\"title\":\"C2\",\"type\":\"null\"},\
                        \"v1\":{\"title\":\"V1\",\"type\":\"null\"}\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, "{\"v1\":null}", "{\"c2\":null,\"v1\":null}");
    success_parse_model!(
        TestModel,
        "{\"v1\":null,\"c2\":null}",
        "{\"c2\":null,\"v1\":null}"
    );

    assert!(TestModel::parse("{}").is_err());
    assert!(TestModel::parse("{\"c2\":null}").is_err());
}

#[test]
fn test_bool_model() {
    fn validate_fn(value: bool) -> Result<bool> {
        if value {
            Ok(value)
        } else {
            Err(Error::validate_err("only true"))
        }
    }

    #[model]
    struct TestModel {
        b1: bool,
        #[field(default = false, alias = "c2")]
        b2: bool,
        #[field(validate = validate_fn)]
        b3: bool,
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"properties\":{\
                        \"b1\":{\"title\":\"B1\",\"type\":\"boolean\"},\
                        \"b3\":{\"title\":\"B3\",\"type\":\"boolean\"},\
                        \"c2\":{\"default\":false,\"title\":\"C2\",\"type\":\"boolean\"}\
                    },\
                    \"required\":[\"b1\",\"b3\"],\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );

    success_parse_model!(
        TestModel,
        "{\"b1\": true,\"b3\": true}",
        "{\"b1\":true,\"b3\":true,\"c2\":false}"
    );
    success_parse_model!(
        TestModel,
        "{\"b1\": false,\"b3\": true}",
        "{\"b1\":false,\"b3\":true,\"c2\":false}"
    );
    success_parse_model!(
        TestModel,
        "{\"c2\":false, \"b1\": true,\"b3\": true}",
        "{\"b1\":true,\"b3\":true,\"c2\":false}"
    );
    success_parse_model!(
        TestModel,
        "{\"c2\":false, \"b1\": false,\"b3\": true}",
        "{\"b1\":false,\"b3\":true,\"c2\":false}"
    );
    success_parse_model!(
        TestModel,
        "{\"c2\":true, \"b1\": true,\"b3\": true}",
        "{\"b1\":true,\"b3\":true,\"c2\":true}"
    );
    success_parse_model!(
        TestModel,
        "{\"c2\":true, \"b1\": false,\"b3\": true}",
        "{\"b1\":false,\"b3\":true,\"c2\":true}"
    );

    assert!(TestModel::parse("{\"b1\": true}").is_err());
    assert!(TestModel::parse("{\"b3\": true}").is_err());
    assert!(TestModel::parse("{\"c2\":true, \"b1\": false,\"b3\": false}").is_err());
}

macro_rules! test_signed_integer_model {
    ($ty: ident) => {
        paste! {
            #[test]
            fn [<test_ $ty _model>] () {
                fn validate_fn(value: $ty) -> Result<$ty> {
                    if value > 0 {
                        Ok(value)
                    } else {
                        Err(Error::validate_err("only positive"))
                    }
                }
                #[model]
                struct TestModel {
                    v1: $ty,
                    #[field(default = 0, alias = "c2")]
                    v2: $ty,
                    #[field(validate = validate_fn)]
                    v3: $ty,
                    #[field(ge = 1, le = 10)]
                    v4: $ty,
                    #[field(gt = 1, lt = 10)]
                    v5: $ty,
                }
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"properties\":{\
                                    \"c2\":{\"default\":0,\"title\":\"C2\",\"type\":\"integer\"},\
                                    \"v1\":{\"title\":\"V1\",\"type\":\"integer\"},\
                                    \"v3\":{\"title\":\"V3\",\"type\":\"integer\"},\
                                    \"v4\":{\"maximum\":10,\"minimum\":1,\"title\":\"V4\",\"type\":\"integer\"},\
                                    \"v5\":{\"exclusiveMaximum\":10,\"exclusiveMinimum\":1,\"title\":\"V5\",\"type\":\"integer\"}\
                                },\
                                \"required\":[\"v1\",\"v3\",\"v4\",\"v5\"],\
                                \"title\":\"TestModel\",\
                                \"type\":\"object\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2}",
                    "{\"c2\":0,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2,\"c2\":-2}",
                    "{\"c2\":-2,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                assert!(TestModel::parse("{\"v3\": 1,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": -1,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 0,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 1}").is_err());
            }
        }
    };
    ( $( $ty:ident ),* ) => { $( test_signed_integer_model!($ty); )* };
}
test_signed_integer_model!(i8, i16, i32, i64, i128, isize);

macro_rules! test_unsigned_integer_model {
    ($ty: ident) => {
        paste! {
            #[test]
            fn [<test_ $ty _model>] () {
                fn validate_fn(value: $ty) -> Result<$ty> {
                    if value > 0 {
                        Ok(value)
                    } else {
                        Err(Error::validate_err("only positive"))
                    }
                }
                #[model]
                struct TestModel {
                    v1: $ty,
                    #[field(default = 0, alias = "c2")]
                    v2: $ty,
                    #[field(validate = validate_fn)]
                    v3: $ty,
                    #[field(ge = 1, le = 10)]
                    v4: $ty,
                    #[field(gt = 1, lt = 10)]
                    v5: $ty,
                }
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"properties\":{\
                                    \"c2\":{\"default\":0,\"title\":\"C2\",\"type\":\"integer\"},\
                                    \"v1\":{\"title\":\"V1\",\"type\":\"integer\"},\
                                    \"v3\":{\"title\":\"V3\",\"type\":\"integer\"},\
                                    \"v4\":{\"maximum\":10,\"minimum\":1,\"title\":\"V4\",\"type\":\"integer\"},\
                                    \"v5\":{\"exclusiveMaximum\":10,\"exclusiveMinimum\":1,\"title\":\"V5\",\"type\":\"integer\"}\
                                },\
                                \"required\":[\"v1\",\"v3\",\"v4\",\"v5\"],\
                                \"title\":\"TestModel\",\
                                \"type\":\"object\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2}",
                    "{\"c2\":0,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2,\"c2\":10}",
                    "{\"c2\":10,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                assert!(TestModel::parse("{\"v3\": 1,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 0,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 0,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 1}").is_err());
            }
        }
    };
    ( $( $ty:ident ),* ) => { $( test_unsigned_integer_model!($ty); )* };
}

test_unsigned_integer_model!(u8, u16, u32, u64, u128, usize);

macro_rules! test_float_model {
    ($ty: ident) => {
        paste! {
            #[test]
            fn [<test_ $ty _model>] () {
                fn validate_fn(value: $ty) -> Result<$ty> {
                    if value > 0.0 {
                        Ok(value)
                    } else {
                        Err(Error::validate_err("only positive"))
                    }
                }
                #[model]
                struct TestModel {
                    v1: $ty,
                    #[field(default = 0.0, alias = "c2")]
                    v2: $ty,
                    #[field(validate = validate_fn)]
                    v3: $ty,
                    #[field(ge = 1.0, le = 10.0)]
                    v4: $ty,
                    #[field(gt = 1.0, lt = 10.0)]
                    v5: $ty,
                }
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"properties\":{\
                                    \"c2\":{\"default\":0,\"title\":\"C2\",\"type\":\"number\"},\
                                    \"v1\":{\"title\":\"V1\",\"type\":\"number\"},\
                                    \"v3\":{\"title\":\"V3\",\"type\":\"number\"},\
                                    \"v4\":{\"maximum\":10,\"minimum\":1,\"title\":\"V4\",\"type\":\"number\"},\
                                    \"v5\":{\"exclusiveMaximum\":10,\"exclusiveMinimum\":1,\"title\":\"V5\",\"type\":\"number\"}\
                                },\
                                \"required\":[\"v1\",\"v3\",\"v4\",\"v5\"],\
                                \"title\":\"TestModel\",\
                                \"type\":\"object\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2}",
                    "{\"c2\":0,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                success_parse_model!(
                    TestModel,
                    "{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 2,\"c2\":-2.1}",
                    "{\"c2\":-2.1,\"v1\":0,\"v3\":1,\"v4\":1,\"v5\":2}"
                );
                assert!(TestModel::parse("{\"v3\": 1,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 0,\"v4\": 1,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 0,\"v5\": 2}").is_err());
                assert!(TestModel::parse("{\"v1\": 0,\"v3\": 1,\"v4\": 1,\"v5\": 1}").is_err());
            }
        }
    };
    ( $( $ty:ident ),* ) => { $( test_float_model!($ty); )* };
}

test_float_model!(f32, f64);

#[test]
fn test_string_model() {
    fn validate_fn(value: String) -> Result<String> {
        Ok(format!("Hello, {}", value))
    }
    #[model]
    struct TestModel {
        v1: String,
        #[field(default = "DEFAULT", alias = "c2")]
        v2: String,
        #[field(validate = validate_fn)]
        v3: String,
        #[field(min_length = 1, max_length = 10)]
        v4: String,
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"properties\":{\
                        \"c2\":{\"default\":\"DEFAULT\",\"title\":\"C2\",\"type\":\"string\"},\
                        \"v1\":{\"title\":\"V1\",\"type\":\"string\"},\
                        \"v3\":{\"title\":\"V3\",\"type\":\"string\"},\
                        \"v4\":{\"maxLength\":10,\"minLength\":1,\"title\":\"V4\",\"type\":\"string\"}\
                    },\
                    \"required\":[\"v1\",\"v3\",\"v4\"],\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\":\"\",\"v3\":\"value3\",\"v4\":\"value4\"}",
        "{\"c2\":\"DEFAULT\",\"v1\":\"\",\"v3\":\"Hello, value3\",\"v4\":\"value4\"}"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\":\"value1\",\"v3\":\"value3\",\"v4\":\"value4\",\"c2\":\"any string\"}",
        "{\"c2\":\"any string\",\"v1\":\"value1\",\"v3\":\"Hello, value3\",\"v4\":\"value4\"}"
    );
    assert!(TestModel::parse("{\"v3\": \"value3\",\"v4\": \"value4\"}").is_err());
    assert!(TestModel::parse("{\"v1\": \"value1\",\"v4\": \"value4\"}").is_err());
    assert!(TestModel::parse("{\"v1\": \"value1\",\"v3\": \"value3\"}").is_err());
    assert!(TestModel::parse("{\"v1\": \"value1\",\"v3\": \"value3\",\"v4\": \"\"}").is_err());
    assert!(
        TestModel::parse("{\"v1\": \"value1\",\"v3\": \"value3\",\"v4\": \"12345678901\"}")
            .is_err()
    );
}

#[test]
fn test_vec_model() {
    fn validate_fn(value: Vec<()>) -> Result<Vec<()>> {
        let mut new_value = Vec::from(value);
        new_value.push(());
        Ok(new_value)
    }
    #[model]
    struct TestModel {
        v1: Vec<()>,
        #[field(alias = "c2")]
        v2: Vec<()>,
        #[field(validate = validate_fn)]
        v3: Vec<()>,
        #[field(min_items = 1, max_items = 10)]
        v4: Vec<()>,
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"properties\":{\
                        \"c2\":{\
                            \"items\":{\"type\":\"null\"},\
                            \"title\":\"C2\",\
                            \"type\":\"array\"\
                        },\
                        \"v1\":{\
                            \"items\":{\"type\":\"null\"},\
                            \"title\":\"V1\",\
                            \"type\":\"array\"\
                        },\
                        \"v3\":{\
                            \"items\":{\"type\":\"null\"},\
                            \"title\":\"V3\",\
                            \"type\":\"array\"\
                        },\
                        \"v4\":{\
                            \"items\":{\"type\":\"null\"},\
                            \"maxItems\":10,\
                            \"minItems\":1,\
                            \"title\":\"V4\",\
                            \"type\":\"array\"\
                        }\
                    },\
                    \"required\":[\"v1\",\"c2\",\"v3\",\"v4\"],\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\":[],\"c2\": [null],\"v3\":[null,null],\"v4\":[null,null,null]}",
        "{\"c2\":[null],\"v1\":[],\"v3\":[null,null,null],\"v4\":[null,null,null]}"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\":[],\"c2\": [null],\"v3\":[null,null],\"v4\":[null]}",
        "{\"c2\":[null],\"v1\":[],\"v3\":[null,null,null],\"v4\":[null]}"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\":[],\"c2\": [null],\"v3\":[null,null],\"v4\":[null,null,null,null,null,null,null,null,null,null]}",
        "{\"c2\":[null],\"v1\":[],\"v3\":[null,null,null],\"v4\":[null,null,null,null,null,null,null,null,null,null]}"
    );
    assert!(
        TestModel::parse("{\"c2\": [null],\"v3\":[null,null],\"v4\":[null,null,null]}").is_err()
    );
    assert!(TestModel::parse("{\"v1\":[],\"v3\":[null,null],\"v4\":[null,null,null]}").is_err());
    assert!(TestModel::parse("{\"v1\":[],\"c2\": [null],\"v4\":[null,null,null]}").is_err());
    assert!(TestModel::parse("{\"v1\":[],\"c2\": [null],\"v3\":[null,null],}").is_err());
    assert!(TestModel::parse("{\"v1\":[],\"c2\": [null],\"v3\":[null,null],\"v4\":[]}").is_err());
    assert!(TestModel::parse("{\"v1\":[],\"c2\": [null],\"v3\":[null,null],\"v4\":[null,null,null,null,null,null,null,null,null,null,null]}").is_err());
}

#[test]
fn test_nested_model() {
    #[model]
    struct TestModel1 {
        v1: u8,
    }
    fn validate_fn(value: TestModel1) -> Result<TestModel1> {
        Ok(TestModel1 { v1: value.v1 + 1 })
    }
    #[model]
    struct TestModel2 {
        v1: TestModel1,
        #[field(alias = "c2", validate = validate_fn)]
        v2: TestModel1,
    }
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"properties\":{\
                        \"v1\":{\
                            \"title\":\"V1\",\
                            \"type\":\"integer\"\
                        }\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"TestModel1\",\
                    \"type\":\"object\"\
                },\
                \"TestModel2\":{\
                    \"properties\":{\
                        \"c2\":{\
                            \"$ref\":\"#/definitions/TestModel1\",\
                            \"title\":\"C2\"\
                        },\
                        \"v1\":{\
                            \"$ref\":\"#/definitions/TestModel1\",\
                            \"title\":\"V1\"\
                        }\
                    },\
                    \"required\":[\"v1\",\"c2\"],\
                    \"title\":\"TestModel2\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel2,
        "{\"v1\":{\"v1\": 1},\"c2\":{\"v1\": 2}}",
        "{\"c2\":{\"v1\":3},\"v1\":{\"v1\":1}}"
    );
    assert!(TestModel2::parse("{\"c2\":{\"v1\": 2}}").is_err());
    assert!(TestModel2::parse("{\"v1\":{\"v1\": 1}}").is_err());
    assert!(TestModel2::parse("{\"v1\":{},\"c2\":{\"v1\": 2}}").is_err());
}

#[test]
fn test_self_reference_model() {
    fn validate_fn(value: Option<Box<TestModel>>) -> Result<Option<Box<TestModel>>> {
        if value.is_some() {
            Ok(value)
        } else {
            Ok(None)
        }
    }
    #[model]
    struct TestModel {
        v1: u8,
        #[field(alias = "self", validate = validate_fn)]
        v2: Option<Box<TestModel>>,
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"properties\":{\
                        \"self\":{\
                            \"anyOf\":[\
                                {\"type\":\"null\"},\
                                {\"$ref\":\"#/definitions/TestModel\"}\
                            ],\
                            \"title\":\"Self\"\
                        },\
                        \"v1\":{\
                            \"title\":\"V1\",\
                            \"type\":\"integer\"\
                        }\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"TestModel\",\
                    \"type\":\"object\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, "{\"v1\": 1}", "{\"self\":null,\"v1\":1}");
    success_parse_model!(
        TestModel,
        "{\"v1\": 1,\"self\":{\"v1\": 2}}",
        "{\"self\":{\"self\":null,\"v1\":2},\"v1\":1}"
    );
    success_parse_model!(
        TestModel,
        "{\"v1\": 1,\"self\":{\"v1\": 2,\"self\":{\"v1\": 3}}}",
        "{\"self\":{\"self\":{\"self\":null,\"v1\":3},\"v1\":2},\"v1\":1}"
    );
    assert!(TestModel::parse("{}").is_err());
    assert!(TestModel::parse("{\"self\":{\"v1\": 2}}").is_err());
}
