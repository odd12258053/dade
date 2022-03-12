use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: u32
    },
}
fn main() {}
