use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(le = 2)]
        value: u32
    },
}
fn main() {}
