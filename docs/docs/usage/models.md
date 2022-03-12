---
title: models
description: 'how to define a model with Rust structures, create a model instance from string with JSON, and export a String from a model instance.'
---

# Models

This article writes how to define a model with Rust structures(struct and enum), create a model instance from string with JSON, and export a String from a model instance.

## Definitions
The model in `dade` has to implement dade::Model.
To implement it, `dade` serve the macro dade::model.

### Basic Usage
```rust
#[model]
struct Item {
	value: u8
}
```

This example corresponds to JSON Object with kay with `value` and value for the key with an integer in dade.
So, we can get an instance for this example from the given string.
```rust
let instance: Item = Item::parse("{\"value\": 1}")?;
// instance = Item { value: 1 }
```
And then, if to export JSON string, call function `.json(bool)` likely this bellow.
```rust
let json_string: String = instance::json(false);
// json_string = "{\"value\":1}"
```

### Validation
In this example, if you need to validate a value (e.g., a value less than 100), you can use the `field` macro in this model.
```rust
#[model]
struct Item {
	#[field(lt = 100)]
	value: u8
}
```
In this, the following code is failed.
```rust
let instance = Item::parse("{\"value\": 100}")?;
```

If you need more details, see [fields](./fields).

Additionally, dade support several complex data structures.
 
## Nested Models
To define a hierarchical data structure, write a code likely the following.

```rust
#[model]
struct Child {
    id: u128,
}
#[model]
struct Parent {
    children: Vec<Child>,
}
```

## Recursive Models
For more complex, for example tree data structure.

```rust
#[model]
struct Node {
    id: u128,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
```

## Inherited Models
If you use the same condition for a type in several models, write a code likely the following.

```rust
#[model]
struct Id(
    #[filed(ge = 1, lt = 100)]
    u64
);
```

## Tuple Models
An array consisting of a different type corresponds to the following.

```rust
#[model]
struct Item(u8, u8);
```

:::caution

A tuple, which has one length, does not support in dade. If you need it, use Vec type in this version.

:::


## Pattern Models
A model with constant values or more complex types corresponds to the following.

```rust
#[model]
struct InnerModel {
    id: u128,
}
#[model]
enum InnerPattern {
    P1,
    P2,
}
#[model]
enum Pattern {
    Value1,
    #[field(alias = "val2")]
    Value2,
    Value3(f32),
    Value4(InnerModel),
    Value5 {
        id: i128,
    },
    Value6(InnerPattern),
    Other(String),
}
```

In this case,
* If a JSON value is `"Value1"`, the parse result will be `Pattern::Value1`.
* If a JSON value is `"val2"`, the parse result will be `Pattern::Value2`.
* If a JSON value is `1.0`, the parse result will be `Pattern::Value3(1.0)`.
