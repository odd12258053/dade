# dade
![Test](https://github.com/odd12258053/dade/workflows/Test/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/dade.svg)](https://crates.io/crates/dade)

`dade` is a framework for defining data structure to Rust structures, like [pydantic](https://pydantic-docs.helpmanual.io) in Python.

For the easy handle of data, the following will support it.
+ [x] validation for (primitive) types.
   + [x] numeric types; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
   + [x] boolean
   + [x] String
   + [x] Optional
   + [x] Vec
   + [x] nested model
   + [x] enum
+ [x] export a data schema conforms JsonSchema.
+ [x] load a model from JSON.
+ [x] dump a JSON-string from a model.
+ [ ] implements for useful types. e.g. URL, email and etc.
+ [ ] publish a trait that for you implements any custom type with your validation. 

See [documentation](https://odd12258053.github.io/dade/) for more details.

## Roadmap
+ ~~implements for basic idea.~~
+ ~~implements for (primitive) types, enum, and more.~~
+ support configuration of a model, for example, set a given name to a title in JsonSchema, or exclusion another key.
+ implements for useful types and a trait for custom type.

## Example
### Basic
To define a model, You need the below module.

```rust
use dade::{Model, model};
```

For example, define user-model.
```rust
#[model]
struct User {
    #[field(ge = 1)]
    id: u64,
    #[field(min_length = 1, max_length = 100)]
    name: String,
    #[field(default = "en")]
    lang: String,
    #[field(min_length = 1, max_length = 255, default = null)]
    url: Option<String>,
    #[field(default = false)]
    verified: bool,
}
```

Then you create an instance of the model by the below.

```rust
let input = "{\"id\": 1, \"name\": \"James Smith\"}";
let user = User::parse(input).unwrap();
```

And you get a Json string for the instance by the below.
```rust
let json_string = user.json(false);
// json_string = "{\"id\":1,\"name\":\"James Smith\",\"lang\":\"en\",\"url\":null,\"verified\":false}"
```

If you want to validate a value, you will get a schema that conforms JsonSchema, for the given model, by the below.

```rust
let schema = User::schema();
```

The schema is 
```json
{
  "$ref": "#/definitions/User",
  "definitions": {
    "User": {
      "title": "User",
      "type": "object",
      "properties": {
        "id": {
          "type": "integer",
          "title": "Id",
          "minimum": 1
        },
        "name": {
          "type": "string",
          "title": "Name",
          "minLength": 1,
          "maxLength": 100
        },
        "lang": {
          "type": "string",
          "title": "Lang",
          "default": "en"
        },
        "url": {
          "type": "string",
          "title": "Url",
          "default": null,
          "minLength": 1,
          "maxLength": 255
        },
        "verified": {
          "type": "boolean",
          "title": "Verified",
          "default": false
        }
      },
      "required": ["id", "name"]
    }
  }
}
```


### Advance
* If you want to bind other name
```rust
#[model]
struct User {
    id: u64,
    #[field(alias = "FirstName")]
    first_name: String,
    #[field(alias = "LastName")]
    last_name: String,
}
```

* If you need a nested model 

```rust
#[model]
struct Name {
    first_name: String,
    last_name: String,
}

#[model]
struct User {
    id: u64,
    name: Name,
}
```

* If you need a self-reference model

```rust
#[model]
struct Item {
    id: u64,
    name: String,
    value: u128,
    related_items: Option<Vec<Box<Item>>>,
}
```
