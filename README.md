# dade
![Test](https://github.com/odd12258053/dade/workflows/Test/badge.svg)

dade is data definition for Rust structures.

For the easy handle of data, the following will support it.
+ [ ] Data validation.
+ [ ] Data schema conforms JsonSchema.


## Example
### Basic
To define a model, You need the below module.

```rust
use dade::Model;
use dade_derive::model;
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
