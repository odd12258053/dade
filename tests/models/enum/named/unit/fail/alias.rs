use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = 1)]
        value: ()
    },
}
fn main() {}
