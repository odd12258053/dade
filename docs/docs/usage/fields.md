---
title: field
---

# Fields
We describe a field in dade that supports types and usage conditions.

## Support types
### numeric
This numeric in dade is u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, or f64 in Rust.

We can validate greater than, less than, or equal for a numeric type.
For detail,
* The term `gt` means to validate greater than the given value.
* The term `ge` means to validate equal to or greater than the given value.
* The term `lt` means to validate less than the given value.
* The term `le` means to validate equal to or less than the given value.

#### Usage
```rust
#[model]
struct Id {
    #[field(ge = 1, lt = 100)]
    id: u64
}
```

### String
This string in dade is String in Rust.
In this version, dade does not support char and bytes.

Supported terms are below. 
* `min_length`
  * If set, a string must have a length equal to or greater than the given value.
* `max_length`
  * If set, a string must have a length equal to or less than the given value.

### Array
This array in dade is Vec in Rust.

Supported terms are below. 
* `min_items`
  * If set, an array must have a length equal to or greater than the given value.
* `max_items`
  * If set, an array must have a length equal to or less than the given value.

### Boolean
This boolean in dade is Boolean in Rust.

### Null
This null in dade is unit`()` in Rust.
This value is only null in JSON.

#### Usage
```rust
#[model]
struct Empty {
    empty: ()
}
```

### Optional
In case of value exists or does not in JSON, you can use Option in Rust.

#### Usage
```rust
#[model]
struct Item {
    value: Option<u32>
}
```

## Term of fields
### Alias
The usage of the term Alias is two methods.
If a key in JSON maps another named field in a model, you can use the term Alias.

```rust
#[model]
struct  Item {
	#[field(alias = "val")]
	value: u64,
}
```

Then, the following passed.
```rust
let item = Item::parse("{\"val\": 1}");
```

On the other hand, use in an enum.
For example,
```rust
#[model]
enum Pattern {
	#[field(alias = "p1")]
	Pattern1,
	#[field(alias = "p2")]
	Pattern2,
}
```
In this case, 
* If a JSON value is `"p1"`, the parse result will be `Pattern::Pattern1`.
* If a JSON value is `"p2"`, the parse result will be `Pattern::Pattern2`.
* If a JSON value is another string, the parse be failed.

### Default
If a key does not exist in JSON, dade set a given value corresponding to it.

For example,
```rust
#[model]
struct  Item {
	#[field(default = 1)]
	value: u64,
}
```

Then, the following passed.
```rust
let item = Item::parse("{}");
// item = {value: 1}
```

### Validate
If you want to validate value to a custom validator, you can use the term validate.

```rust
use dade::{model, Model, Result, Error};
fn custom_validate(value: i8) -> Result<i8> {
    if value > 0 {
        Ok(value)
    } else {
        Err(Error::validate_err("only positive"))
    }
}

#[model]
struct Item {
	#[field(validate = custom_validate)]
	value: i8
}
```

In this, the following is failed.
```rust
let instance = Item::parse("{\"value\": -1}")?;
```
