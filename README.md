# non_empty

Create around the `NonEmpty` struct, wrapping a non-empty value.

Version: 0.2.1


## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies.non_empty]
git = "https://github.com/thibran/non_empty.git"
tag = "v0.2.1"
```

In your code add:
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
#[derive(Clone)]
struct Point(u32, u32);

impl IsEmpty for Point {
    fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

assert!(Point(0,0).try_non_empty().is_none());
```

`NonEmpty` implements the
[Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html),
[AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html),
[Borrow](https://doc.rust-lang.org/collections/borrow/trait.Borrow.html)
and [Into](https://doc.rust-lang.org/std/convert/trait.Into.html)
traits to easily access its inner value.

```rust
let s: NonEmpty<String> = "hello".to_string().try_non_empty().unwrap();

// Deref NonEmpty<String> to &str
foobar(&s); 

fn foobar(s: &str) {
    assert_eq!("hello", s);
}
```

**Tip**: Use the defined [type-aliases](https://github.com/thibran/non_empty/blob/master/src/lib.rs#L202) to
shorten e.g. `NonEmpty<String>` to `StringNE`.

```rust
use non_empty::{StringNE, TryNonEmpty};

let s: StringNE = "hello".to_string().try_non_empty().unwrap();
```

**Tip2**: Use the provided helper functions like [try_non_empty2](https://github.com/thibran/non_empty/blob/master/src/helper_try_convert.rs#L8v)
to convert multiple values at once to a tuple of [NonEmpty](https://github.com/thibran/non_empty/blob/master/src/lib.rs#L98)'s.

```rust
let (a, b): (NonEmpty<&str>, NonEmpty<i32>) = try_non_empty2("a", 1).unwrap();

assert_eq!("a", *a);
assert_eq!(1, *b);
```