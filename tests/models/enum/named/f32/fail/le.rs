use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(le = 2)]
        value: f32
    },
}
fn main() {}
