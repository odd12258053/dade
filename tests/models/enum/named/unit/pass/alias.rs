use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: ()
    },
}
fn main() {}
