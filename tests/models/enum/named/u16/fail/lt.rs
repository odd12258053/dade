use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(lt = 2.0)]
        value: u16
    },
}
fn main() {}
