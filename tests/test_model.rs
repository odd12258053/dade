use dade::Model;
use dade_derive::model;

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
    let foo_str = "{\"v1\": 2.2,\"v4\": true, \"bar\": {\"w1\": 10}}";

    let ret = Foo::parse(foo_str);
    assert!(ret.is_ok(), "{}", ret.err().unwrap().to_string());

    let foo = ret.unwrap();
    assert_eq!(foo.v1, 2.2);
    assert_eq!(foo.v2, 10);
    assert_eq!(foo.v3, "abc");
    assert_eq!(foo.v4, Some(true));
    assert_eq!(foo.v5.w1, 10);
    assert_eq!(foo.v6, ());
    assert_eq!(
        foo.json(),
        "{\"v1\":2.2,\"v2\":10,\"v3\":\"abc\",\"v4\":true,\"bar\":{\"w1\":10},\"v6\":null}"
    );
}
