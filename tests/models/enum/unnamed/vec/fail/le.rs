use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        Vec<()>
    ),
}
fn main() {}
