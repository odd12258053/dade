use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(lt = 2)]
        value: i8
    },
}
fn main() {}
