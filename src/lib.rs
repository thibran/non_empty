#![warn(missing_docs)]

//! [NonEmpty](struct.NonEmpty.html) represents a value that is not empty.
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
//! is straightforward, just implement [is_empty()](trait.TryNonEmpty.html#tymethod.is_empty).
//!
//! ```
//! use non_empty::TryNonEmpty;
//!
//! struct Point(u32, u32);
//!
//! impl TryNonEmpty for Point {
//!     fn is_empty(&self) -> bool {
//!         self.0 == 0 && self.1 == 0
//!     }
//! }
//!
//! assert!(Point(0,0).try_non_empty().is_none());
//! ```
//!
//! **Tip**: Use the defined [type-aliases](index.html#types) to
//! shorten e.g. `NonEmpty<String>` to `StringNE`.
//!
//! ```
//! use non_empty::{StringNE, NonEmpty, TryNonEmpty};
//!
//! let s: StringNE = "hello".to_string().try_non_empty().unwrap();
//! ```


/// A struct owning a non-empty value.
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
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

/// The only way to create a `NonEmpty<T>` struct.
///
/// # Examples
/// ```
/// // Implementing the TryNonEmpty trait is straightforward,
/// // all you have to do is to implement TryNonEmpty::is_empty.
///
/// # use non_empty::TryNonEmpty;
/// struct Point(u32, u32);
///
/// impl TryNonEmpty for Point {
///     fn is_empty(&self) -> bool {
///         self.0 == 0 && self.1 == 0
///     }
/// }
///
/// assert!(Point(0,0).try_non_empty().is_none());
/// ```
pub trait TryNonEmpty {
    /// Decides if `try_non_empty()` returns
    /// [Some](https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some)
    /// or [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
    fn is_empty(&self) -> bool;

    /// Default implementation to create a [NonEmpty](struct.NonEmpty.html) struct.
    /// Can not be overwritten.
    fn try_non_empty(self) -> Option<NonEmpty<Self>>
        where Self: Sized
    {
        if !&self.is_empty() {
            Some(NonEmpty { inner: self })
        } else {
            None
        }
    }
}

/// Non-empty `String`
pub type StringNE = NonEmpty<String>;

impl TryNonEmpty for String {
    #[inline]
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<'a> TryNonEmpty for &'a String {
    #[inline]
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

/// Non-empty `str`
pub type StrNE = NonEmpty<str>;

impl<'a> TryNonEmpty for &'a str {
    #[inline]
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

/// Non-empty `OsStr`
pub type OsStrNE = NonEmpty<std::ffi::OsStr>;

impl TryNonEmpty for std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        std::ffi::OsStr::is_empty(self)
    }
}

impl<'a> TryNonEmpty for &'a std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        std::ffi::OsStr::is_empty(self)
    }
}

/// Non-empty `Path`
pub type PathNE = NonEmpty<std::path::Path>;

impl<'a> TryNonEmpty for &'a std::path::Path {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

/// Non-empty `PathBuf`
pub type PathBufNE = NonEmpty<std::path::PathBuf>;

impl TryNonEmpty for std::path::PathBuf {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl<'a> TryNonEmpty for &'a std::path::PathBuf {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

/// Non-empty `Vec<T>`
pub type VecNE<T> = NonEmpty<Vec<T>>;

impl<T> TryNonEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<'a, T> TryNonEmpty for &'a Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

/// Non-empty `[T]`
pub type SliceNE<T> = NonEmpty<[T]>;

impl<T> TryNonEmpty for [T] {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Non-empty `HashMap<K, V>`
pub type HashMapNE<K, V> = NonEmpty<std::collections::HashMap<K, V>>;

impl<K, V, S> TryNonEmpty for std::collections::HashMap<K, V, S>
    where S: std::hash::BuildHasher,
          K: std::hash::Hash + Eq
{
    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

impl<'a, K, V, S> TryNonEmpty for &'a std::collections::HashMap<K, V, S>
    where S: std::hash::BuildHasher,
          K: std::hash::Hash + Eq
{
    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

/// Non-empty `HashSet<T, S>`
pub type HashSetNE<T, S> = NonEmpty<std::collections::HashSet<T, S>>;

impl<T, S> TryNonEmpty for std::collections::HashSet<T, S>
    where S: std::hash::BuildHasher,
          T: std::hash::Hash + Eq
{
    fn is_empty(&self) -> bool {
        std::collections::HashSet::is_empty(self)
    }
}

impl<'a, T, S> TryNonEmpty for &'a std::collections::HashSet<T, S>
    where S: std::hash::BuildHasher,
          T: std::hash::Hash + Eq
{
    fn is_empty(&self) -> bool {
        std::collections::HashSet::is_empty(self)
    }
}

/// Non-empty `LinkedList<T>`
pub type LinkedListNE<T> = NonEmpty<std::collections::LinkedList<T>>;

impl<T> TryNonEmpty for std::collections::LinkedList<T> {
    fn is_empty(&self) -> bool {
        std::collections::LinkedList::is_empty(self)
    }
}

impl<'a, T> TryNonEmpty for &'a std::collections::LinkedList<T> {
    fn is_empty(&self) -> bool {
        std::collections::LinkedList::is_empty(self)
    }
}

/// Non-empty `VecDeque<T>`
pub type VecDequeNE<T> = NonEmpty<std::collections::VecDeque<T>>;

impl<T> TryNonEmpty for std::collections::VecDeque<T> {
    fn is_empty(&self) -> bool {
        std::collections::VecDeque::is_empty(self)
    }
}

impl<'a, T> TryNonEmpty for &'a std::collections::VecDeque<T> {
    fn is_empty(&self) -> bool {
        std::collections::VecDeque::is_empty(self)
    }
}

/// Non-empty `BTreeMap<K, V>`
pub type BTreeMapNE<K, V> = NonEmpty<std::collections::BTreeMap<K, V>>;

impl<K, V> TryNonEmpty for std::collections::BTreeMap<K, V> {
    fn is_empty(&self) -> bool {
        std::collections::BTreeMap::is_empty(self)
    }
}

impl<'a, K, V> TryNonEmpty for &'a std::collections::BTreeMap<K, V> {
    fn is_empty(&self) -> bool {
        std::collections::BTreeMap::is_empty(self)
    }
}

/// Non-empty `BTreeSet<T>`
pub type BTreeSetNE<T> = NonEmpty<std::collections::BTreeSet<T>>;

impl<T: Ord> TryNonEmpty for std::collections::BTreeSet<T> {
    fn is_empty(&self) -> bool {
        std::collections::BTreeSet::is_empty(self)
    }
}

impl<'a, T: Ord> TryNonEmpty for &'a std::collections::BTreeSet<T> {
    fn is_empty(&self) -> bool {
        std::collections::BTreeSet::is_empty(self)
    }
}

/// Non-empty `BinaryHeap<T>`
pub type BinaryHeapNE<T> = NonEmpty<std::collections::BinaryHeap<T>>;

impl<T: Ord> TryNonEmpty for std::collections::BinaryHeap<T> {
    fn is_empty(&self) -> bool {
        std::collections::BinaryHeap::is_empty(self)
    }
}

impl<'a, T: Ord> TryNonEmpty for &'a std::collections::BinaryHeap<T> {
    fn is_empty(&self) -> bool {
        std::collections::BinaryHeap::is_empty(self)
    }
}

/// Non-empty `i8`, number != 0
#[allow(non_camel_case_types)]
pub type i8NE = NonEmpty<i8>;

impl TryNonEmpty for i8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `i16`, number != 0
#[allow(non_camel_case_types)]
pub type i16NE = NonEmpty<i16>;

impl TryNonEmpty for i16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `i32`, number != 0
#[allow(non_camel_case_types)]
pub type i32NE = NonEmpty<i32>;

impl TryNonEmpty for i32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `i64`, number != 0
#[allow(non_camel_case_types)]
pub type i64NE = NonEmpty<i64>;

impl TryNonEmpty for i64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `u8`, number != 0
#[allow(non_camel_case_types)]
pub type u8NE = NonEmpty<u8>;

impl TryNonEmpty for u8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `u16`, number != 0
#[allow(non_camel_case_types)]
pub type u16NE = NonEmpty<u16>;

impl TryNonEmpty for u16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `u32`, number != 0
#[allow(non_camel_case_types)]
pub type u32NE = NonEmpty<u32>;

impl TryNonEmpty for u32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `u64`, number != 0
#[allow(non_camel_case_types)]
pub type u64NE = NonEmpty<u64>;

impl TryNonEmpty for u64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `isize`, number != 0
#[allow(non_camel_case_types)]
pub type isizeNE = NonEmpty<isize>;

impl TryNonEmpty for isize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `usize`, number != 0
#[allow(non_camel_case_types)]
pub type usizeNE = NonEmpty<usize>;

impl TryNonEmpty for usize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

/// Non-empty `f32`, number != 0
#[allow(non_camel_case_types)]
pub type f32NE = NonEmpty<f32>;

impl TryNonEmpty for f32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0_f32
    }
}

/// Non-empty `f64`, number != 0
#[allow(non_camel_case_types)]
pub type f64NE = NonEmpty<f64>;

impl TryNonEmpty for f64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0_f64
    }
}

#[cfg(test)]
mod tests {
    use super::TryNonEmpty;

    #[test]
    fn okay() {
        assert!("".try_non_empty().is_none());
        assert_eq!("bar", "bar".try_non_empty().unwrap().into_inner());
    }
}