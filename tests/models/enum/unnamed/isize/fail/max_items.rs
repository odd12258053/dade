use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_items = 2)]
        isize
    ),
}
fn main() {}
