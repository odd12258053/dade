use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(le = 2)]
        value: i32
    },
}
fn main() {}
