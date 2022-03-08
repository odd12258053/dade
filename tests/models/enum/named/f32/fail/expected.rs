use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(expected = "val")]
        value: f32
    },
}
fn main() {}
