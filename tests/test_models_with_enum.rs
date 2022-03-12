use dade::{model, Error, Model, Result};

macro_rules! success_parse_model {
    ($model: ident, $expected: pat, $in_string:literal, $out_string:literal) => {
        let ret = $model::parse($in_string);
        assert!(ret.is_ok());
        let val = ret.unwrap();
        assert!(matches!(val, $expected));
        assert_eq!(val.json(false), $out_string);
    };
}

#[test]
fn test_literal_model() {
    #[model]
    enum TestModel1 { Value1 }
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"anyOf\":[\
                        {\
                            \"const\":\"Value1\",\
                            \"title\":\"Value1\"\
                        }\
                    ],\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, TestModel1::Value1, "\"Value1\"", "\"Value1\"");
    assert!(TestModel1::parse("\"value1\"").is_err());

    #[model]
    enum TestModel2 {
        Value1,
        #[field(alias = "val2")]
        Value2
    }
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"anyOf\":[\
                        {\
                            \"const\":\"Value1\",\
                            \"title\":\"Value1\"\
                        },\
                        {\
                            \"const\":\"val2\",\
                            \"title\":\"Value2\"\
                        }\
                    ],\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, TestModel2::Value1, "\"Value1\"", "\"Value1\"");
    success_parse_model!(TestModel2, TestModel2::Value2, "\"val2\"", "\"val2\"");
    assert!(TestModel2::parse("\"value1\"").is_err());
    assert!(TestModel2::parse("\"Value2\"").is_err());
}

#[test]
fn test_named_field_model() {
    #[model]
    enum TestModel1 {
        Value1 { v1: u64 }
    }
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"anyOf\":[\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value1\",\
                            \"type\":\"object\"\
                        }\
                    ],\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, TestModel1::Value1{..}, "{\"v1\": 1}", "{\"v1\":1}");
    assert!(TestModel1::parse("{\"v1\": -1}").is_err());

    #[model]
    enum TestModel2 {
        Value1 { v1: u64 },
        Value2 { v1: i64 },
    }
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"anyOf\":[\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value1\",\
                            \"type\":\"object\"\
                        },\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value2\",\
                            \"type\":\"object\"\
                        }\
                    ],\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, TestModel2::Value1{..}, "{\"v1\": 1}", "{\"v1\":1}");
    success_parse_model!(TestModel2, TestModel2::Value2{..}, "{\"v1\": -1}", "{\"v1\":-1}");

    fn validate_fn(value: i64) -> Result<i64> {
        if value > 0 {
            Ok(value)
        } else {
            Err(Error::validate_err("only positive"))
        }
    }
    #[model]
    enum TestModel3 {
        Value1 { v1: u64 },
        Value2 { #[field(validate = validate_fn)] v1: i64 },
        Value3 { v1: i64, v2: Option<u64> },
    }
    assert_eq!(
        TestModel3::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel3\",\
            \"definitions\":{\
                \"TestModel3\":{\
                    \"anyOf\":[\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value1\",\
                            \"type\":\"object\"\
                        },\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value2\",\
                            \"type\":\"object\"\
                        },\
                        {\
                            \"properties\":{\
                                \"v1\":{\
                                    \"title\":\"V1\",\
                                    \"type\":\"integer\"\
                                },\
                                \"v2\":{\
                                    \"anyOf\":[\
                                        {\"type\":\"null\"},\
                                        {\"type\":\"integer\"}\
                                    ],\
                                    \"title\":\"V2\"\
                                }\
                            },\
                            \"required\":[\"v1\"],\
                            \"title\":\"Value3\",\
                            \"type\":\"object\"\
                        }\
                    ],\
                    \"title\":\"TestModel3\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel3, TestModel3::Value1{..}, "{\"v1\": 1}", "{\"v1\":1}");
    success_parse_model!(TestModel3, TestModel3::Value3{..}, "{\"v1\": -1}", "{\"v1\":-1,\"v2\":null}");
}

#[test]
fn test_unnamed_field_model() {
    #[model]
    enum TestModel1 {
        Value1(u64)
    }
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"anyOf\":[\
                        {\
                            \"title\":\"Value1\",\
                            \"type\":\"integer\"\
                        }\
                    ],\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, TestModel1::Value1(_), "0", "0");
    success_parse_model!(TestModel1, TestModel1::Value1(_), "1", "1");
    assert!(TestModel1::parse("-1").is_err());

    #[model]
    enum TestModel2 {
        Value1(u64),
        Value2(i64),
    }
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"anyOf\":[\
                        {\
                            \"title\":\"Value1\",\
                            \"type\":\"integer\"\
                        },\
                        {\
                            \"title\":\"Value2\",\
                            \"type\":\"integer\"\
                        }\
                    ],\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, TestModel2::Value1(_), "0", "0");
    success_parse_model!(TestModel2, TestModel2::Value1(_), "1", "1");
    success_parse_model!(TestModel2, TestModel2::Value2(_), "-1", "-1");

    #[model]
    enum TestModel3 {
        Value1(u64),
        Value2(#[field(ge = -10)] i64),
        Value3(i64),
    }
    assert_eq!(
        TestModel3::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel3\",\
            \"definitions\":{\
                \"TestModel3\":{\
                    \"anyOf\":[\
                        {\
                            \"title\":\"Value1\",\
                            \"type\":\"integer\"\
                        },\
                        {\
                            \"minimum\":-10,\
                            \"title\":\"Value2\",\
                            \"type\":\"integer\"\
                        },\
                        {\
                            \"title\":\"Value3\",\
                            \"type\":\"integer\"\
                        }\
                    ],\
                    \"title\":\"TestModel3\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel3, TestModel3::Value1(_), "0", "0");
    success_parse_model!(TestModel3, TestModel3::Value1(_), "1", "1");
    success_parse_model!(TestModel3, TestModel3::Value2(_), "-1", "-1");
    success_parse_model!(TestModel3, TestModel3::Value2(_), "-10", "-10");
    success_parse_model!(TestModel3, TestModel3::Value3(_), "-11", "-11");
}

#[test]
fn test_complex_model() {
    #[model]
    struct InnerModel {
        id: u128
    }
    #[model]
    enum InnerPattern {
        P1,
        P2,
    }
    #[model]
    enum TestModel {
        Value1,
        #[field(alias = "val2")]
        Value2,
        Value3(f32),
        Value4(InnerModel),
        Value5{id: i128},
        Value6(InnerPattern),
        Other(String)
    }
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"InnerModel\":{\
                    \"properties\":{\
                        \"id\":{\"title\":\"Id\",\"type\":\"integer\"}\
                    },\
                    \"required\":[\"id\"],\
                    \"title\":\"InnerModel\",\
                    \"type\":\"object\"\
                },\
                \"InnerPattern\":{\
                    \"anyOf\":[\
                        {\
                            \"const\":\"P1\",\
                            \"title\":\"P1\"\
                        },\
                        {\
                            \"const\":\"P2\",\
                            \"title\":\"P2\"\
                        }\
                    ],\
                    \"title\":\"InnerPattern\"\
                },\
                \"TestModel\":{\
                    \"anyOf\":[\
                        {\
                            \"const\":\"Value1\",\
                            \"title\":\"Value1\"\
                        },\
                        {\
                            \"const\":\"val2\",\
                            \"title\":\"Value2\"\
                        },\
                        {\
                            \"title\":\"Value3\",\
                            \"type\":\"number\"\
                        },\
                        {\
                            \"$ref\":\"#/definitions/InnerModel\",\
                            \"title\":\"Value4\"\
                        },\
                        {\
                            \"properties\":{\
                                \"id\":{\"title\":\"Id\",\"type\":\"integer\"}\
                            },\
                            \"required\":[\"id\"],\
                            \"title\":\"Value5\",\
                            \"type\":\"object\"\
                        },\
                        {\
                            \"$ref\":\"#/definitions/InnerPattern\",\
                            \"title\":\"Value6\"\
                        },\
                        {\
                            \"title\":\"Other\",\
                            \"type\":\"string\"\
                        }\
                    ],\
                    \"title\":\"TestModel\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, TestModel::Value1, "\"Value1\"", "\"Value1\"");
    success_parse_model!(TestModel, TestModel::Value2, "\"val2\"", "\"val2\"");
    success_parse_model!(TestModel, TestModel::Value3(_), "1", "1");
    success_parse_model!(TestModel, TestModel::Value3(_), "1.7", "1.7");
    success_parse_model!(TestModel, TestModel::Value4(_), "{\"id\": 1}", "{\"id\":1}");
    success_parse_model!(TestModel, TestModel::Value5{..}, "{\"id\": -1}", "{\"id\":-1}");
    success_parse_model!(TestModel, TestModel::Value6(_), "\"P1\"", "\"P1\"");
    success_parse_model!(TestModel, TestModel::Value6(_), "\"P2\"", "\"P2\"");
    success_parse_model!(TestModel, TestModel::Other(_), "\"abc\"", "\"abc\"");
}
