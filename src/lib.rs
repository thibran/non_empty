pub struct NonEmpty<T> {
    inner: T,
}

impl<T> NonEmpty<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> AsRef<T> for NonEmpty<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

pub trait TryNonEmpty {
    fn is_empty(&self) -> bool;

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

pub type StrNE = NonEmpty<str>;
impl<'a> TryNonEmpty for &'a str {
    #[inline]
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

pub type OsStrNE = NonEmpty<std::ffi::OsStr>;
impl TryNonEmpty for std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        std::ffi::OsStr::is_empty(self)
    }
}

impl<'a>  TryNonEmpty for &'a std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        std::ffi::OsStr::is_empty(self)
    }
}

pub type PathNE = NonEmpty<std::path::Path>;
impl<'a> TryNonEmpty for &'a std::path::Path {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

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

pub type SliceNE<T> = NonEmpty<[T]>;
impl<T> TryNonEmpty for [T] {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

pub type HashMapNE<K, V> = NonEmpty<std::collections::HashMap<K, V>>;
impl<K, V, S> TryNonEmpty for std::collections::HashMap<K,V, S>
    where S: std::hash::BuildHasher,
          K: std::hash::Hash + Eq,
{
    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

impl<'a, K, V, S> TryNonEmpty for &'a std::collections::HashMap<K,V, S>
    where S: std::hash::BuildHasher,
          K: std::hash::Hash + Eq,
{
    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

pub type HashSetNE<T, S> = NonEmpty<std::collections::HashSet<T, S>>;
impl<T, S> TryNonEmpty for std::collections::HashSet<T, S>
    where S: std::hash::BuildHasher,
          T: std::hash::Hash + Eq,
{
    fn is_empty(&self) -> bool {
        std::collections::HashSet::is_empty(self)
    }
}

impl<'a, T, S> TryNonEmpty for &'a std::collections::HashSet<T, S>
    where S: std::hash::BuildHasher,
          T: std::hash::Hash + Eq,
{
    fn is_empty(&self) -> bool {
        std::collections::HashSet::is_empty(self)
    }
}

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

#[allow(non_camel_case_types)]
pub type i8NE = NonEmpty<i8>;

impl TryNonEmpty for i8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type i16NE = NonEmpty<i16>;

impl TryNonEmpty for i16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type i32NE = NonEmpty<i32>;

impl TryNonEmpty for i32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type i64NE = NonEmpty<i64>;

impl TryNonEmpty for i64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type u8NE = NonEmpty<u8>;

impl TryNonEmpty for u8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type u16NE = NonEmpty<u16>;

impl TryNonEmpty for u16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type u32NE = NonEmpty<u32>;

impl TryNonEmpty for u32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type u64NE = NonEmpty<u64>;

impl TryNonEmpty for u64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type isizeNE = NonEmpty<isize>;

impl TryNonEmpty for isize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type usizeNE = NonEmpty<usize>;

impl TryNonEmpty for usize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[allow(non_camel_case_types)]
pub type f32NE = NonEmpty<f32>;

impl TryNonEmpty for f32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0_f32
    }
}

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
