use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2.0)]
        value: usize
    },
}
fn main() {}
