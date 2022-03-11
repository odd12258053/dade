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
fn test_null_model() {
    #[model]
    struct TestModel((), #[field(default = null)] ());
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\"type\":\"null\"},\
                        {\"default\":null,\"type\":\"null\"}\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, "[null]", "[null,null]");
    success_parse_model!(TestModel, "[null, null]", "[null,null]");
    success_parse_model!(TestModel, "[null, null, null]", "[null,null]");
    assert!(TestModel::parse("[]").is_err());
    assert!(TestModel::parse("[1, null, null]").is_err());
    assert!(TestModel::parse("{}").is_err());

    #[model]
    struct TestModel1(());
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"title\":\"TestModel1\",\
                    \"type\":\"null\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "null", "null");
    assert!(TestModel1::parse("[null]").is_err());
    assert!(TestModel1::parse("{}").is_err());

    #[model]
    struct TestModel2(#[field(default = null)] ());
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"default\":null,\
                    \"title\":\"TestModel2\",\
                    \"type\":\"null\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "null", "null");
    assert!(TestModel2::parse("[null]").is_err());
    assert!(TestModel2::parse("[]").is_err());
    assert!(TestModel2::parse("{}").is_err());
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
    struct TestModel(
        bool,
        #[field(default = false)] bool,
        #[field(validate = validate_fn)] bool,
        #[field(default = false)] bool,
    );
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\"type\":\"boolean\"},\
                        {\"default\":false,\"type\":\"boolean\"},\
                        {\"type\":\"boolean\"},\
                        {\"default\":false,\"type\":\"boolean\"}\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );

    success_parse_model!(TestModel, "[true,true,true]", "[true,true,true,false]");
    success_parse_model!(TestModel, "[false,true,true]", "[false,true,true,false]");
    success_parse_model!(TestModel, "[true,false,true]", "[true,false,true,false]");
    success_parse_model!(TestModel, "[true,true,true,true]", "[true,true,true,true]");
    success_parse_model!(
        TestModel,
        "[true,true,true,false]",
        "[true,true,true,false]"
    );
    success_parse_model!(
        TestModel,
        "[true,true,true,true,true]",
        "[true,true,true,true]"
    );
    assert!(TestModel::parse("[true,true]").is_err());
    assert!(TestModel::parse("[true,true,false]").is_err());

    #[model]
    struct TestModel1(bool);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"title\":\"TestModel1\",\
                    \"type\":\"boolean\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "true", "true");
    success_parse_model!(TestModel1, "false", "false");
    assert!(TestModel1::parse("1").is_err());
    assert!(TestModel1::parse("\"abc\"").is_err());

    #[model]
    struct TestModel2(#[field(default = true, validate = validate_fn)] bool);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"default\":true,\
                    \"title\":\"TestModel2\",\
                    \"type\":\"boolean\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "true", "true");
    assert!(TestModel2::parse("false").is_err());
    assert!(TestModel2::parse("1").is_err());
    assert!(TestModel2::parse("\"abc\"").is_err());
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
                struct TestModel (
                    $ty,
                    #[field(validate = validate_fn)]
                    $ty,
                    #[field(ge = 1, le = 10)]
                    $ty,
                    #[field(gt = 1, lt = 10)]
                    $ty,
                    #[field(default = 0)]
                    $ty,
                );
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"prefixItems\":[\
                                    {\"type\":\"integer\"},\
                                    {\"type\":\"integer\"},\
                                    {\"maximum\":10,\"minimum\":1,\"type\":\"integer\"},\
                                    {\"exclusiveMaximum\":10,\"exclusiveMinimum\":1,\"type\":\"integer\"},\
                                    {\"default\":0,\"type\":\"integer\"}\
                                ],\
                                \"title\":\"TestModel\",\
                                \"type\":\"array\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel, "[0, 1, 1, 2]", "[0,1,1,2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 2, 1]", "[0,1,1,2,1]");
                success_parse_model!(TestModel, "[-1, 1, 1, 2]", "[-1,1,1,2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 2, 3, 4]", "[0,1,1,2,3]");
                assert!(TestModel::parse("[0, 1, 1]").is_err());
                assert!(TestModel::parse("[0, 0, 1, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 0, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 1, 1]").is_err());

                #[model]
                struct TestModel1 ($ty);
                assert_eq!(
                    TestModel1::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel1\",\
                        \"definitions\":{\
                            \"TestModel1\":{\
                                \"title\":\"TestModel1\",\
                                \"type\":\"integer\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel1, "0", "0");
                success_parse_model!(TestModel1, "-1", "-1");
                success_parse_model!(TestModel1, "1", "1");

                #[model]
                struct TestModel2 (#[field(validate = validate_fn, ge = 1, le = 10)] $ty);
                assert_eq!(
                    TestModel2::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel2\",\
                        \"definitions\":{\
                            \"TestModel2\":{\
                                \"maximum\":10,\
                                \"minimum\":1,\
                                \"title\":\"TestModel2\",\
                                \"type\":\"integer\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel2, "1", "1");
                success_parse_model!(TestModel2, "2", "2");
                success_parse_model!(TestModel2, "10", "10");
                assert!(TestModel2::parse("0").is_err());
                assert!(TestModel2::parse("-1").is_err());
                assert!(TestModel2::parse("11").is_err());
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
                struct TestModel (
                    $ty,
                    #[field(validate = validate_fn)]
                    $ty,
                    #[field(ge = 1, le = 10)]
                    $ty,
                    #[field(gt = 1, lt = 10)]
                    $ty,
                    #[field(default = 0)]
                    $ty,
                );
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"prefixItems\":[\
                                    {\"type\":\"integer\"},\
                                    {\"type\":\"integer\"},\
                                    {\"maximum\":10,\"minimum\":1,\"type\":\"integer\"},\
                                    {\"exclusiveMaximum\":10,\"exclusiveMinimum\":1,\"type\":\"integer\"},\
                                    {\"default\":0,\"type\":\"integer\"}\
                                ],\
                                \"title\":\"TestModel\",\
                                \"type\":\"array\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel, "[0, 1, 1, 2]", "[0,1,1,2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 2, 1]", "[0,1,1,2,1]");
                success_parse_model!(TestModel, "[1, 1, 1, 2]", "[1,1,1,2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 2, 3, 4]", "[0,1,1,2,3]");
                assert!(TestModel::parse("[-1, 1, 1, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 1]").is_err());
                assert!(TestModel::parse("[0, 0, 1, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 0, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 1, 1]").is_err());

                #[model]
                struct TestModel1 ($ty);
                assert_eq!(
                    TestModel1::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel1\",\
                        \"definitions\":{\
                            \"TestModel1\":{\
                                \"title\":\"TestModel1\",\
                                \"type\":\"integer\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel1, "0", "0");
                success_parse_model!(TestModel1, "1", "1");
                assert!(TestModel1::parse("-1").is_err());

                #[model]
                struct TestModel2 (#[field(validate = validate_fn, ge = 1, le = 10)] $ty);
                assert_eq!(
                    TestModel2::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel2\",\
                        \"definitions\":{\
                            \"TestModel2\":{\
                                \"maximum\":10,\
                                \"minimum\":1,\
                                \"title\":\"TestModel2\",\
                                \"type\":\"integer\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel2, "1", "1");
                success_parse_model!(TestModel2, "2", "2");
                success_parse_model!(TestModel2, "10", "10");
                assert!(TestModel2::parse("0").is_err());
                assert!(TestModel2::parse("-1").is_err());
                assert!(TestModel2::parse("11").is_err());
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
                struct TestModel (
                    $ty,
                    #[field(validate = validate_fn)]
                    $ty,
                    #[field(ge = 1.0, le = 10.0)]
                    $ty,
                    #[field(gt = 1.1, lt = 10.0)]
                    $ty,
                    #[field(default = 0.0)]
                    $ty,
                );
                assert_eq!(
                    TestModel::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel\",\
                        \"definitions\":{\
                            \"TestModel\":{\
                                \"prefixItems\":[\
                                    {\"type\":\"number\"},\
                                    {\"type\":\"number\"},\
                                    {\"maximum\":10,\"minimum\":1,\"type\":\"number\"},\
                                    {\"exclusiveMaximum\":10,\"exclusiveMinimum\":1.1,\"type\":\"number\"},\
                                    {\"default\":0,\"type\":\"number\"}\
                                ],\
                                \"title\":\"TestModel\",\
                                \"type\":\"array\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel, "[0, 1, 1, 1.2]", "[0,1,1,1.2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 1.2, 1]", "[0,1,1,1.2,1]");
                success_parse_model!(TestModel, "[-1, 1, 1, 1.2]", "[-1,1,1,1.2,0]");
                success_parse_model!(TestModel, "[0, 1, 1, 1.2, 3, 4]", "[0,1,1,1.2,3]");
                assert!(TestModel::parse("[0, 1, 1]").is_err());
                assert!(TestModel::parse("[0, 0, 1, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 0, 2]").is_err());
                assert!(TestModel::parse("[0, 1, 1, 1]").is_err());

                #[model]
                struct TestModel1 ($ty);
                assert_eq!(
                    TestModel1::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel1\",\
                        \"definitions\":{\
                            \"TestModel1\":{\
                                \"title\":\"TestModel1\",\
                                \"type\":\"number\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel1, "0", "0");
                success_parse_model!(TestModel1, "-1", "-1");
                success_parse_model!(TestModel1, "1", "1");

                #[model]
                struct TestModel2 (#[field(validate = validate_fn, ge = 1.5, le = 10.0)] $ty);
                assert_eq!(
                    TestModel2::schema(),
                    "{\
                        \"$ref\":\"#/definitions/TestModel2\",\
                        \"definitions\":{\
                            \"TestModel2\":{\
                                \"maximum\":10,\
                                \"minimum\":1.5,\
                                \"title\":\"TestModel2\",\
                                \"type\":\"number\"\
                            }\
                        }\
                    }"
                );
                success_parse_model!(TestModel2, "1.5", "1.5");
                success_parse_model!(TestModel2, "2", "2");
                success_parse_model!(TestModel2, "10", "10");
                assert!(TestModel2::parse("0").is_err());
                assert!(TestModel2::parse("1").is_err());
                assert!(TestModel2::parse("-1").is_err());
                assert!(TestModel2::parse("11").is_err());
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
    struct TestModel(
        String,
        #[field(validate = validate_fn)] String,
        #[field(min_length = 1, max_length = 10)] String,
        #[field(default = "DEFAULT")] String,
    );
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\"type\":\"string\"},\
                        {\"type\":\"string\"},\
                        {\"maxLength\":10,\"minLength\":1,\"type\":\"string\"},\
                        {\"default\":\"DEFAULT\",\"type\":\"string\"}\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "[\"\",\"value3\",\"value4\"]",
        "[\"\",\"Hello, value3\",\"value4\",\"DEFAULT\"]"
    );
    success_parse_model!(
        TestModel,
        "[\"abc\",\"value3\",\"value4\",\"def\"]",
        "[\"abc\",\"Hello, value3\",\"value4\",\"def\"]"
    );
    success_parse_model!(
        TestModel,
        "[\"abc\",\"value3\",\"value4\",\"def\",\"other\"]",
        "[\"abc\",\"Hello, value3\",\"value4\",\"def\"]"
    );
    assert!(TestModel::parse("[\"\",\"value3\"]").is_err());
    assert!(TestModel::parse("[\"\",\"value3\",\"\"]").is_err());
    assert!(TestModel::parse("[\"\",\"value3\",\"12345678901\"]").is_err());

    #[model]
    struct TestModel1(String);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"title\":\"TestModel1\",\
                    \"type\":\"string\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "\"\"", "\"\"");
    success_parse_model!(TestModel1, "\"value\"", "\"value\"");

    #[model]
    struct TestModel2(#[field(validate = validate_fn, min_length = 1, max_length = 10)] String);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"maxLength\":10,\
                    \"minLength\":1,\
                    \"title\":\"TestModel2\",\
                    \"type\":\"string\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "\"a\"", "\"Hello, a\"");
    success_parse_model!(TestModel2, "\"value\"", "\"Hello, value\"");
    assert!(TestModel2::parse("\"\"]").is_err());
    assert!(TestModel2::parse("\"01234567890\"]").is_err());
}

#[test]
fn test_vec_model() {
    fn validate_fn(value: Vec<()>) -> Result<Vec<()>> {
        let mut new_value = Vec::from(value);
        new_value.push(());
        Ok(new_value)
    }
    #[model]
    struct TestModel(
        Vec<()>,
        #[field(validate = validate_fn)] Vec<()>,
        #[field(min_items = 1, max_items = 10)] Vec<()>,
    );
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\
                            \"items\":{\"type\":\"null\"},\
                            \"type\":\"array\"\
                        },\
                        {\
                            \"items\":{\"type\":\"null\"},\
                            \"type\":\"array\"\
                        },\
                        {\
                            \"items\":{\"type\":\"null\"},\
                            \"maxItems\":10,\
                            \"minItems\":1,\
                            \"type\":\"array\"\
                        }\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "[[], [null], [null,null]]",
        "[[],[null,null],[null,null]]"
    );
    success_parse_model!(TestModel, "[[null], [], [null]]", "[[null],[null],[null]]");
    success_parse_model!(
        TestModel,
        "[[null], [], [null,null,null,null,null,null,null,null,null,null]]",
        "[[null],[null],[null,null,null,null,null,null,null,null,null,null]]"
    );
    success_parse_model!(
        TestModel,
        "[[], [null], [null,null], 1]",
        "[[],[null,null],[null,null]]"
    );
    assert!(TestModel::parse("[[], [null], []]").is_err());
    assert!(TestModel::parse(
        "[[], [null], [null,null,null,null,null,null,null,null,null,null,null]]"
    )
    .is_err());

    #[model]
    struct TestModel1(Vec<()>);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"items\":{\"type\":\"null\"},\
                    \"title\":\"TestModel1\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "[]", "[]");
    success_parse_model!(TestModel1, "[null]", "[null]");
    success_parse_model!(
        TestModel1,
        "[null, null, null, null]",
        "[null,null,null,null]"
    );

    #[model]
    struct TestModel2(#[field(validate = validate_fn, min_items = 1, max_items = 10)] Vec<()>);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"items\":{\"type\":\"null\"},\
                    \"maxItems\":10,\
                    \"minItems\":1,\
                    \"title\":\"TestModel2\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "[null]", "[null,null]");
    success_parse_model!(
        TestModel2,
        "[null, null, null, null, null, null, null, null, null, null]",
        "[null,null,null,null,null,null,null,null,null,null,null]"
    );
    assert!(TestModel2::parse("[]").is_err());
    assert!(TestModel2::parse("[null,null,null,null,null,null,null,null,null,null,null]").is_err());
}

#[test]
fn test_struct_model() {
    #[model]
    struct InnerModel {
        v1: u8,
    }
    fn validate_fn(value: InnerModel) -> Result<InnerModel> {
        Ok(InnerModel { v1: value.v1 + 1 })
    }
    #[model]
    struct TestModel(InnerModel, #[field(validate = validate_fn)] InnerModel);
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"InnerModel\":{\
                    \"properties\":{\
                        \"v1\":{\
                            \"title\":\"V1\",\
                            \"type\":\"integer\"\
                        }\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"InnerModel\",\
                    \"type\":\"object\"\
                },\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\
                            \"$ref\":\"#/definitions/InnerModel\"\
                        },\
                        {\
                            \"$ref\":\"#/definitions/InnerModel\"\
                        }\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "[{\"v1\": 1}, {\"v1\": 1}]",
        "[{\"v1\":1},{\"v1\":2}]"
    );
    success_parse_model!(
        TestModel,
        "[{\"v1\": 1}, {\"v1\": 1}, 1]",
        "[{\"v1\":1},{\"v1\":2}]"
    );
    assert!(TestModel::parse("[{\"v1\": 1}]").is_err());
    assert!(TestModel::parse("[]").is_err());

    #[model]
    struct TestModel1(InnerModel);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"InnerModel\":{\
                    \"properties\":{\
                        \"v1\":{\
                            \"title\":\"V1\",\
                            \"type\":\"integer\"\
                        }\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"InnerModel\",\
                    \"type\":\"object\"\
                },\
                \"TestModel1\":{\
                    \"$ref\":\"#/definitions/InnerModel\",\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "{\"v1\": 1}", "{\"v1\":1}");

    #[model]
    struct TestModel2(#[field(validate = validate_fn)] InnerModel);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"InnerModel\":{\
                    \"properties\":{\
                        \"v1\":{\
                            \"title\":\"V1\",\
                            \"type\":\"integer\"\
                        }\
                    },\
                    \"required\":[\"v1\"],\
                    \"title\":\"InnerModel\",\
                    \"type\":\"object\"\
                },\
                \"TestModel2\":{\
                    \"$ref\":\"#/definitions/InnerModel\",\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "{\"v1\": 1}", "{\"v1\":2}");
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
    struct TestModel(
        Option<Box<TestModel>>,
        #[field(validate = validate_fn)] Option<Box<TestModel>>,
    );
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\
                            \"anyOf\":[\
                                {\"type\":\"null\"},\
                                {\"$ref\":\"#/definitions/TestModel\"}\
                            ]\
                        },\
                        {\
                            \"anyOf\":[\
                                {\"type\":\"null\"},\
                                {\"$ref\":\"#/definitions/TestModel\"}\
                            ]\
                        }\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel, "[]", "[null,null]");
    success_parse_model!(TestModel, "[[]]", "[[null,null],null]");
    success_parse_model!(TestModel, "[[],null]", "[[null,null],null]");
    success_parse_model!(TestModel, "[[],[]]", "[[null,null],[null,null]]");

    #[model]
    struct TestModel1(Option<Box<TestModel1>>);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"TestModel1\":{\
                    \"anyOf\":[\
                        {\"type\":\"null\"},\
                        {\"$ref\":\"#/definitions/TestModel1\"}\
                    ],\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "null", "null");

    fn validate_fn2(value: Option<Box<TestModel2>>) -> Result<Option<Box<TestModel2>>> {
        if value.is_some() {
            Ok(value)
        } else {
            Ok(None)
        }
    }

    #[model]
    struct TestModel2(#[field(validate = validate_fn2)] Option<Box<TestModel2>>);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"TestModel2\":{\
                    \"anyOf\":[\
                        {\"type\":\"null\"},\
                        {\"$ref\":\"#/definitions/TestModel2\"}\
                    ],\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "null", "null");
}

#[test]
fn test_enum_model() {
    #[model]
    enum Pattern {
        Value1,
        Value2,
    }
    fn validate_fn(value: Pattern) -> Result<Pattern> {
        Ok(match value {
            Pattern::Value1 => Pattern::Value2,
            Pattern::Value2 => Pattern::Value1,
        })
    }
    #[model]
    struct TestModel(Pattern, #[field(validate = validate_fn)] Pattern);
    assert_eq!(
        TestModel::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel\",\
            \"definitions\":{\
                \"Pattern\":{\
                    \"anyOf\":[{\
                        \"const\":\"Value1\",\
                        \"title\":\"Value1\"\
                    },{\
                        \"const\":\"Value2\",\
                        \"title\":\"Value2\"\
                    }],\
                    \"title\":\"Pattern\"\
                },\
                \"TestModel\":{\
                    \"prefixItems\":[\
                        {\
                            \"$ref\":\"#/definitions/Pattern\"\
                        },\
                        {\
                            \"$ref\":\"#/definitions/Pattern\"\
                        }\
                    ],\
                    \"title\":\"TestModel\",\
                    \"type\":\"array\"\
                }\
            }\
        }"
    );
    success_parse_model!(
        TestModel,
        "[\"Value1\",\"Value1\"]",
        "[\"Value1\",\"Value2\"]"
    );
    success_parse_model!(
        TestModel,
        "[\"Value1\",\"Value2\"]",
        "[\"Value1\",\"Value1\"]"
    );
    success_parse_model!(
        TestModel,
        "[\"Value2\",\"Value2\"]",
        "[\"Value2\",\"Value1\"]"
    );
    success_parse_model!(
        TestModel,
        "[\"Value2\",\"Value2\", 1]",
        "[\"Value2\",\"Value1\"]"
    );
    assert!(TestModel::parse("[\"Value2\"]").is_err());
    assert!(TestModel::parse("[\"Value3\",\"Value1\"]").is_err());

    #[model]
    struct TestModel1(Pattern);
    assert_eq!(
        TestModel1::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel1\",\
            \"definitions\":{\
                \"Pattern\":{\
                    \"anyOf\":[{\
                        \"const\":\"Value1\",\
                        \"title\":\"Value1\"\
                    },{\
                        \"const\":\"Value2\",\
                        \"title\":\"Value2\"\
                    }],\
                    \"title\":\"Pattern\"\
                },\
                \"TestModel1\":{\
                    \"$ref\":\"#/definitions/Pattern\",\
                    \"title\":\"TestModel1\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel1, "\"Value1\"", "\"Value1\"");
    success_parse_model!(TestModel1, "\"Value2\"", "\"Value2\"");

    #[model]
    struct TestModel2(#[field(validate = validate_fn)] Pattern);
    assert_eq!(
        TestModel2::schema(),
        "{\
            \"$ref\":\"#/definitions/TestModel2\",\
            \"definitions\":{\
                \"Pattern\":{\
                    \"anyOf\":[{\
                        \"const\":\"Value1\",\
                        \"title\":\"Value1\"\
                    },{\
                        \"const\":\"Value2\",\
                        \"title\":\"Value2\"\
                    }],\
                    \"title\":\"Pattern\"\
                },\
                \"TestModel2\":{\
                    \"$ref\":\"#/definitions/Pattern\",\
                    \"title\":\"TestModel2\"\
                }\
            }\
        }"
    );
    success_parse_model!(TestModel2, "\"Value1\"", "\"Value2\"");
    success_parse_model!(TestModel2, "\"Value2\"", "\"Value1\"");
}
