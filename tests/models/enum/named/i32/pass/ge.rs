use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: i32
    },
}
fn main() {}
