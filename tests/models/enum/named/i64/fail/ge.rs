use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2.0)]
        value: i64
    },
}
fn main() {}
