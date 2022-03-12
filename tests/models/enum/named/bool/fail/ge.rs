use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: bool
    },
}
fn main() {}
