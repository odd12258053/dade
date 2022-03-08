use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2.0)]
        f64
    ),
}
fn main() {}
