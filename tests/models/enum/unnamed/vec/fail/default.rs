use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        Vec<()>
    ),
}
fn main() {}
