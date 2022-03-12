use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: u64
    },
}
fn main() {}
