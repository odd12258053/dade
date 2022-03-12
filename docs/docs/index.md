---
title: ''
description: dade is a framework for defining data structure to Rust structures
---

# dade

*dade* is a framework for defining data structure to Rust structures, like [pydantic](https://pydantic-docs.helpmanual.io) in Python.

For the easy handle of data, the following will support it.

+ [x] validation for (primitive) types.
  + numeric types; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
  + boolean
  + String
  + Optional
  + Vec
  + nested model
  + enum


+ [x] export a data schema conforms JsonSchema.
+ [x] load a model from JSON.
+ [x] dump a JSON-string from a model.
+ [ ] implements for useful types. e.g. URL, email and etc.
+ [ ] publish a trait that for you implements any custom type with your validation. 

For example, define book-model.

```rust
use dade::{model, Model};

#[model]
struct Year(
  #[field(ge = 1000, le = 9999)]
  u16
);

#[model]
struct Tag(String);

#[model]
enum Category {
  C1,
  C2,
  C3
}

#[model]
struct Book {
  published: Year,
  #[field(min_length = 1, max_length = 30)]
  publisher: String,
  #[field(min_length = 1, max_length = 30)]
  author: String,
  #[field(min_length = 1)]
  title: String,
  #[field(min_length = 1)]
  description: String,
  #[field(ge = 0.0)]
  price: f32,
  tags: Option<Vec<Tag>>,
  category: Category
}
```

---

If you want to validate a value, you will get a schema that conforms JsonSchema, for the given model, by the below.
```rust
let schema = Book::schema()
```

The schema is 
```json
{
  "$ref": "#/definitions/Book",
  "definitions": {
    "Book": {
        "properties": {
            "author": {
                "maxLength": 30,
                "minLength": 1,
                "title": "Author",
                "type": "string"
            },
            "category": {
                "$ref": "#/definitions/Category",
                "title": "Category"
            },
            "description": {
                "minLength": 1,
                "title": "Description",
                "type": "string"
            },
            "price": {
                "minimum": 0,
                "title": "Price",
                "type": "number"
            },
            "published": {
                "$ref": "#/definitions/Year",
                "title": "Published"
            },
            "publisher": {
                "maxLength": 30,
                "minLength": 1,
                "title": "Publisher",
                "type": "string"
            },
            "tags": {
                "anyOf": [
                  { "type": "null" },
                  {
                    "items": { "$ref": "#/definitions/Tag" },
                    "type": "array"
                  }
                ],
                "title": "Tags"
            },
            "title": {
                "minLength": 1,
                "title": "Title",
                "type": "string"
            }
        },
        "required": ["published", "publisher", "author", "title", "description", "price", "category"],
        "title": "Book",
        "type": "object"
    },
    "Category": {
        "anyOf": [
          {"const": "C1", "title": "C1"},
          {"const": "C2", "title": "C2"},
          {"const": "C3", "title": "C3"}
        ],
        "title": "Category"
    },
    "Tag": {
        "title": "Tag",
        "type": "string"
    },
    "Year": {
        "maximum": 9999,
        "minimum": 1000,
        "title": "Year",
        "type": "integer"
    }
  }
}
```
