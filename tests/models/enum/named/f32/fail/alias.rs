use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: f32
    },
}
fn main() {}
