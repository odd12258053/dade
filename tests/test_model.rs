use dade::{model, Model};

fn test(a: usize) -> dade::Result<usize> {
    Ok(a)
}

#[model]
struct Bar {
    w1: isize,
}

#[model]
struct Foo {
    // comments
    #[doc = r" Single line doc comments"]
    #[field(le = 1000.0, gt = 0.0)]
    v1: f32,
    #[field(ge= 1, validate= test, default= 10)]
    v2: usize,
    #[field(min_length = 1, default = "abc")]
    v3: String,
    #[field()]
    v4: Option<bool>,
    #[field(alias = "bar")]
    v5: Bar,
    #[field]
    v6: (),
}

#[test]
fn test_model() {
    let json = "{\"v1\": 2.2,\"v4\": true, \"bar\": {\"w1\": 10}}";

    let ret = Foo::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());

    let foo = ret.unwrap();
    assert_eq!(foo.v1, 2.2);
    assert_eq!(foo.v2, 10);
    assert_eq!(foo.v3, "abc");
    assert_eq!(foo.v4, Some(true));
    assert_eq!(foo.v5.w1, 10);
    assert_eq!(foo.v6, ());
    assert_eq!(
        foo.json(false),
        "{\"bar\":{\"w1\":10},\"v1\":2.2,\"v2\":10,\"v3\":\"abc\",\"v4\":true,\"v6\":null}"
    );
}

#[model]
struct Nested {
    #[field]
    id: u32,
    #[field]
    child: Option<Box<Nested>>,
}

#[test]
fn test_nested_model() {
    let json = "{\"id\": 1}";
    let ret = Nested::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());
    assert_eq!(ret.unwrap().json(false), "{\"child\":null,\"id\":1}");

    let json = "{\"id\": 1, \"child\": null}";
    let ret = Nested::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());
    assert_eq!(ret.unwrap().json(false), "{\"child\":null,\"id\":1}");

    let json = "{\"id\": 1, \"child\": {\"id\": 2}}";
    let ret = Nested::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());
    assert_eq!(
        ret.unwrap().json(false),
        "{\"child\":{\"child\":null,\"id\":2},\"id\":1}"
    );

    let json = "{\"id\": 1, \"child\": {\"id\": 2, \"child\": {\"id\": 3}}}";
    let ret = Nested::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());
    assert_eq!(
        ret.unwrap().json(false),
        "{\"child\":{\"child\":{\"child\":null,\"id\":3},\"id\":2},\"id\":1}"
    );
}

#[model]
struct Simple {
    id: u32,
    key: String,
}

#[test]
fn test_simple_model() {
    let json = "{\"id\": 1,\"key\": \"value\"}";
    let ret = Simple::parse(json);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());
    assert_eq!(ret.unwrap().json(false), "{\"id\":1,\"key\":\"value\"}");
}
