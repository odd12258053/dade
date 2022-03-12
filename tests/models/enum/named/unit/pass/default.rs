use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = null)]
        value: ()
    },
}
fn main() {}
