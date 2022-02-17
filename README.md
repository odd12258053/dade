# dade

dade is data definition for Rust structures.

For the easy handle of data, the following will support it.
+ [ ] Data validation.
+ [ ] Data schema conforms JsonSchema.


## Example

```rust
use dade::Model;
use dade_derive::model;

#[model]
struct User {
  #[field(ge= 1)]
  id: u64,
  #[field(min_length = 1, max_length = 100)]
  name: String,
  #[field(default = "en")]
  lang: String,
  url: Option<String>,
  #[field(default = false)]
  verified: bool
}

fn main() {
    let input = "{\"id\": 1, \"name\": \"James Smith\"}";
    let user = User::parse(input).unwrap();
    let output = user.json();
    assert_eq!(output, "{\"id\":1,\"name\":\"James Smith\",\"lang\":\"en\",\"url\":null,\"verified\":false}")
}
```
