use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = 1)]
        value: u32
    },
}
fn main() {}
