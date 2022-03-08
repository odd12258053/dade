use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_items = 2)]
        value: f32
    },
}
fn main() {}
