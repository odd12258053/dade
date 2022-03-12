use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: Vec<()>
    },
}
fn main() {}
