use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(lt = 2)]
        value: u32
    },
}
fn main() {}
