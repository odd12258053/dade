use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_length = 2)]
        value: f32
    },
}
fn main() {}
