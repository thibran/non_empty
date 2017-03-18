# non_empty

Create around the `NonEmpty` struct, wrapping a non-empty value.

Version: 0.1.1


## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
non_empty = { git = "https://github.com/thibran/non_empty.git" }
```

In your code, add:
```rust
extern crate non_empty;
```


## Examples

```rust
use non_empty::{NonEmpty, TryNonEmpty};

let s: NonEmpty<&str> = "hello".try_non_empty().unwrap();

// use into_inner() to get the wrapped value out of a NonEmpty<T>
let inner: &str = s.into_inner();

assert_eq!("hello", inner);
```

Implementing `TryNonEmpty` for a custom struct
is straightforward, just implement the `is_empty()`
function of the `IsEmpty` trait.

```rust
struct Point(u32, u32);

impl IsEmpty for Point {
    fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

assert!(Point(0,0).try_non_empty().is_none());
```

`NonEmpty` implements the
[AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
and [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) traits to easily access its inner value.

```rust
let s: NonEmpty<String> = "hello".to_string().try_non_empty().unwrap();

// Deref NonEmpty<String> to &str
foobar(&s); 

fn foobar(s: &str) {
    assert_eq!("hello", s);
}
```

**Tip**: Use the defined [type-aliases](index.html#types) to
shorten e.g. `NonEmpty<String>` to `StringNE`.

```rust
use non_empty::{StringNE, TryNonEmpty};

let s: StringNE = "hello".to_string().try_non_empty().unwrap();
```

**Tip2**: Use the provided helper functions like [try_non_empty2](fn.try_non_empty2.html)
to convert multiple values at once to a tuple of [NonEmpty](struct.NonEmpty.html)'s.

```rust
let (a, b): (NonEmpty<&str>, NonEmpty<i32>) = try_non_empty2("a", 1).unwrap();

assert_eq!("a", *a);
assert_eq!(1, *b);
```