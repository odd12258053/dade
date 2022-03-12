use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(lt = 2)]
        value: f32
    },
}
fn main() {}
