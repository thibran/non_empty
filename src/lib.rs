#![warn(missing_docs)]

//! [NonEmpty](struct.NonEmpty.html) represents a value which is not empty.
//!
//! # Examples
//! ```
//! use non_empty::{NonEmpty, TryNonEmpty};
//!
//! let s: NonEmpty<&str> = "hello".try_non_empty().unwrap();
//!
//! // use into_inner() to get the wrapped value out of a NonEmpty<T>
//! let inner: &str = s.into_inner();
//!
//! assert_eq!("hello", inner);
//! ```
//!
//! Implementing [TryNonEmpty](trait.TryNonEmpty.html) for a custom struct
//! is straightforward, just implement the [is_empty()](trait.TryNonEmpty.html#tymethod.is_empty)
//! function of the [IsEmpty](trait.IsEmpty.html) trait.
//!
//! ```
//! # use non_empty::{IsEmpty, TryNonEmpty};
//! struct Point(u32, u32);
//!
//! impl IsEmpty for Point {
//!     fn is_empty(&self) -> bool {
//!         self.0 == 0 && self.1 == 0
//!     }
//! }
//!
//! assert!(Point(0,0).try_non_empty().is_none());
//! ```
//!
//! [NonEmpty](struct.NonEmpty.html) implements the
//! [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
//! and [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) traits to easily access
//! its inner value.
//!
//! ```
//! # use non_empty::{NonEmpty, TryNonEmpty};
//! let s: NonEmpty<String> = "hello".to_string().try_non_empty().unwrap();
//!
//! // Deref NonEmpty<String> to &str
//! foobar(&s); 
//!
//! fn foobar(s: &str) {
//!     assert_eq!("hello", s);
//! }
//! ```
//!
//! **Tip**: Use the defined [type-aliases](index.html#types) to
//! shorten e.g. `NonEmpty<String>` to `StringNE`.
//!
//! ```
//! use non_empty::{StringNE, TryNonEmpty};
//!
//! let s: StringNE = "hello".to_string().try_non_empty().unwrap();
//! ```
//!
//! **Tip2**: Use the provided helper functions like [try_non_empty2](fn.try_non_empty2.html)
//! to convert multiple values at once to a tuple of [NonEmpty](struct.NonEmpty.html)'s.
//!
//! ```
//! # use non_empty::{NonEmpty, try_non_empty2};
//! let (a, b): (NonEmpty<&str>, NonEmpty<i32>) = try_non_empty2("a", 1).unwrap();
//!
//! assert_eq!("a", *a);
//! assert_eq!(1, *b);
//! ```

mod is_empty;
mod helper_try_convert;
pub use is_empty::IsEmpty;
pub use helper_try_convert::*;

/// Struct owning a non-empty value.
///
/// # Examples
/// ```
/// use non_empty::{NonEmpty, TryNonEmpty};
///
/// let s: NonEmpty<&str> = "hello".try_non_empty().unwrap();
/// assert_eq!("hello", s.into_inner());
/// ```
///
/// ```
/// use non_empty::{StringNE, TryNonEmpty};
///
/// // use alias-types like StringNE, to improve the readability of the type.
/// let s: StringNE = "hello".to_string().try_non_empty().unwrap();
/// ```
pub struct NonEmpty<T> {
    inner: T,
}

impl<T> NonEmpty<T> {
    /// Consumes `NonEmpty<T>` and returns the inner value `T`.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> AsRef<T> for NonEmpty<T> {
    /// Reference to the inner type `T`.
    #[inline]
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> std::borrow::Borrow<T> for NonEmpty<T> {
    /// Reference to the inner type `T`.
    #[inline]
    fn borrow(&self) -> &T {
        &self.inner
    }
}

/// Allows to dereference `NonEmpty<T>` on the fly.
///
/// # Examples
///
/// ```
/// # use non_empty::{NonEmpty, TryNonEmpty};
/// fn foobar(s: &str) {
///     assert_eq!("hello", s);
/// }
///
/// let s: NonEmpty<String> = "hello".to_string().try_non_empty().unwrap();
///
/// // Deref NonEmpty<String> to &str
/// foobar(&s); 
/// ```
impl<T> std::ops::Deref for NonEmpty<T> {
    type Target = T;

    /// Reference to the inner type `T` of the [NonEmpty](struct.NonEmpty.html) struct.
    fn deref(&self) -> &T {
        &self.inner
    }
}

/// The only way to create a `NonEmpty<T>` struct.
///
/// # Examples
/// ```
/// // Implementing TryNonEmpty is straightforward,
/// // all you have to do is to implement the IsEmpty trait.
///
/// # use non_empty::{IsEmpty, TryNonEmpty};
/// struct Point(u32, u32);
///
/// impl IsEmpty for Point {
///     fn is_empty(&self) -> bool {
///         self.0 == 0 && self.1 == 0
///     }
/// }
///
/// assert!(Point(0,0).try_non_empty().is_none());
/// ```
pub trait TryNonEmpty: Sized + IsEmpty {

    /// Only way to create a [NonEmpty](struct.NonEmpty.html) struct.
    fn try_non_empty(self) -> Option<NonEmpty<Self>>;
}

impl<T: IsEmpty + Sized> TryNonEmpty for T {

    /// The only way to create a [NonEmpty](struct.NonEmpty.html) struct.
    #[inline]
    fn try_non_empty(self) -> Option<NonEmpty<T>> {
        if ! &self.is_empty() {
            Some(NonEmpty { inner: self })
        } else {
            None
        }
    }
}


/////////////////////////////////////////////////////////////////////////
// TYPE-ALIASES
/////////////////////////////////////////////////////////////////////////

/// Non-empty `String`
pub type StringNE = NonEmpty<String>;

/// Non-empty `str`
pub type StrNE = NonEmpty<str>;

/// Non-empty `OsStr`
pub type OsStrNE = NonEmpty<std::ffi::OsStr>;

/// Non-empty `Path`
pub type PathNE = NonEmpty<std::path::Path>;

/// Non-empty `PathBuf`
pub type PathBufNE = NonEmpty<std::path::PathBuf>;

/// Non-empty `Vec<T>`
pub type VecNE<T> = NonEmpty<Vec<T>>;

/// Non-empty `[T]`
pub type SliceNE<T> = NonEmpty<[T]>;

/// Non-empty `HashMap<K, V>`
pub type HashMapNE<K, V> = NonEmpty<std::collections::HashMap<K, V>>;

/// Non-empty `HashSet<T, S>`
pub type HashSetNE<T, S> = NonEmpty<std::collections::HashSet<T, S>>;

/// Non-empty `LinkedList<T>`
pub type LinkedListNE<T> = NonEmpty<std::collections::LinkedList<T>>;

/// Non-empty `VecDeque<T>`
pub type VecDequeNE<T> = NonEmpty<std::collections::VecDeque<T>>;

/// Non-empty `BTreeMap<K, V>`
pub type BTreeMapNE<K, V> = NonEmpty<std::collections::BTreeMap<K, V>>;

/// Non-empty `BTreeSet<T>`
pub type BTreeSetNE<T> = NonEmpty<std::collections::BTreeSet<T>>;

/// Non-empty `BinaryHeap<T>`
pub type BinaryHeapNE<T> = NonEmpty<std::collections::BinaryHeap<T>>;

/// Non-empty `i8`, number != 0
#[allow(non_camel_case_types)]
pub type i8NE = NonEmpty<i8>;

/// Non-empty `i16`, number != 0
#[allow(non_camel_case_types)]
pub type i16NE = NonEmpty<i16>;

/// Non-empty `i32`, number != 0
#[allow(non_camel_case_types)]
pub type i32NE = NonEmpty<i32>;

/// Non-empty `i64`, number != 0
#[allow(non_camel_case_types)]
pub type i64NE = NonEmpty<i64>;

/// Non-empty `u8`, number != 0
#[allow(non_camel_case_types)]
pub type u8NE = NonEmpty<u8>;

/// Non-empty `u16`, number != 0
#[allow(non_camel_case_types)]
pub type u16NE = NonEmpty<u16>;

/// Non-empty `u32`, number != 0
#[allow(non_camel_case_types)]
pub type u32NE = NonEmpty<u32>;

/// Non-empty `u64`, number != 0
#[allow(non_camel_case_types)]
pub type u64NE = NonEmpty<u64>;

/// Non-empty `isize`, number != 0
#[allow(non_camel_case_types)]
pub type isizeNE = NonEmpty<isize>;

/// Non-empty `usize`, number != 0
#[allow(non_camel_case_types)]
pub type usizeNE = NonEmpty<usize>;

/// Non-empty `f32`, number != 0
#[allow(non_camel_case_types)]
pub type f32NE = NonEmpty<f32>;

/// Non-empty `f64`, number != 0
#[allow(non_camel_case_types)]
pub type f64NE = NonEmpty<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn okay() {
        assert!("".try_non_empty().is_none());
        assert_eq!("bar", "bar".try_non_empty().unwrap().into_inner());
    }

    // #[test]
    // fn test_try_convert2() {
    //     assert!(try_convert2("a", 0).is_none());
    //     let (a, b) = try_convert2("a", 1).unwrap();
    //     assert_eq!("a", *a);
    //     assert_eq!(1, *b);
    // }

    // #[test]
    // fn test_try_convert3() {
    //     assert!(try_convert3("a", 0, 5_f32).is_none());
    //     let (a, b, c) = try_convert3("a", 1, 5_f32).unwrap();
    //     assert_eq!("a", *a);
    //     assert_eq!(1, *b);
    //     assert_eq!(5_f32, *c);
    // }

    // #[test]
    // fn test_try_convert4() {
    //     assert!(try_convert4("a", 0, 5_f32, 3).is_none());
    //     let (a, b, c, d) = try_convert4("a", 1, 5_f32, 3).unwrap();
    //     assert_eq!("a", *a);
    //     assert_eq!(1, *b);
    //     assert_eq!(5_f32, *c);
    //     assert_eq!(3, *d);
    // }

    // #[test]
    // fn test_try_convert5() {
    //     assert!(try_convert5("a", 0, 5_f32, 3, "b").is_none());
    //     let (a, b, c, d, e) = try_convert5("a", 1, 5_f32, 3, "b").unwrap();
    //     assert_eq!("a", *a);
    //     assert_eq!(1, *b);
    //     assert_eq!(5_f32, *c);
    //     assert_eq!(3, *d);
    //     assert_eq!("b", *e);
    // }

    // #[test]
    // fn test_try_convert6() {
    //     assert!(try_convert6("a", 0, 5_f32, 3, "b", 4_f64).is_none());
    //     let (a, b, c, d, e, f) = try_convert6("a", 1, 5_f32, 3, "b", 4_f64).unwrap();
    //     assert_eq!("a", *a);
    //     assert_eq!(1, *b);
    //     assert_eq!(5_f32, *c);
    //     assert_eq!(3, *d);
    //     assert_eq!("b", *e);
    //     assert_eq!(4_f64, *f);
    // }
}
