use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        f64
    ),
}
fn main() {}
