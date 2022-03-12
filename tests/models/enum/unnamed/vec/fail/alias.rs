use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        Vec<()>
    ),
}
fn main() {}
