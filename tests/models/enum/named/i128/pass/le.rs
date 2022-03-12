use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(le = 2)]
        value: i128
    },
}
fn main() {}
