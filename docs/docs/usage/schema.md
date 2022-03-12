---
title: schema
---

# Schema
The model in dade can export string with JsonSchema.
To get it, we will call `Model::schema()`.
For instance,
```rust
#[model]
struct Item {
   value: u8
}
```

Then the schema of Item is to get the following.
```rust
let schema = Item::schema();
```
The return is 
```json
{
    "$ref":"#/definitions/Item",
    "definitions":{
        "TestModel":{
            "properties": {
              "value": {
                "title":"Value",
                "type":"integer"
              }
            },
            "title":"Item",
            "type":"object"
        }
    }
}
```
