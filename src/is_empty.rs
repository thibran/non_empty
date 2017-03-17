/// IsEmpty allows objects to clarify that they are empty.
pub trait IsEmpty {
    /// True if the value is empty, e.g. a zero sized String or an empty vector.
    fn is_empty(&self) -> bool;
}

impl IsEmpty for String {
    #[inline]
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<'a> IsEmpty for &'a String {
    #[inline]
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<'a> IsEmpty for &'a str {
    #[inline]
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for ::std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::ffi::OsStr::is_empty(self)
    }
}

impl<'a> IsEmpty for &'a ::std::ffi::OsStr {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::ffi::OsStr::is_empty(self)
    }
}

impl<'a> IsEmpty for &'a ::std::path::Path {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl IsEmpty for ::std::path::PathBuf {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl<'a> IsEmpty for &'a ::std::path::PathBuf {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

impl<T> IsEmpty for Vec<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<'a, T> IsEmpty for &'a Vec<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T> IsEmpty for [T] {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V, S> IsEmpty for ::std::collections::HashMap<K, V, S>
    where S: ::std::hash::BuildHasher,
          K: ::std::hash::Hash + Eq
{
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::HashMap::is_empty(self)
    }
}

impl<'a, K, V, S> IsEmpty for &'a ::std::collections::HashMap<K, V, S>
    where S: ::std::hash::BuildHasher,
          K: ::std::hash::Hash + Eq
{
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::HashMap::is_empty(self)
    }
}

impl<T, S> IsEmpty for ::std::collections::HashSet<T, S>
    where S: ::std::hash::BuildHasher,
          T: ::std::hash::Hash + Eq
{
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::HashSet::is_empty(self)
    }
}

impl<'a, T, S> IsEmpty for &'a ::std::collections::HashSet<T, S>
    where S: ::std::hash::BuildHasher,
          T: ::std::hash::Hash + Eq
{
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::HashSet::is_empty(self)
    }
}

impl<T> IsEmpty for ::std::collections::LinkedList<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::LinkedList::is_empty(self)
    }
}

impl<'a, T> IsEmpty for &'a ::std::collections::LinkedList<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::LinkedList::is_empty(self)
    }
}

impl<T> IsEmpty for ::std::collections::VecDeque<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::VecDeque::is_empty(self)
    }
}

impl<'a, T> IsEmpty for &'a ::std::collections::VecDeque<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::VecDeque::is_empty(self)
    }
}

impl<K, V> IsEmpty for ::std::collections::BTreeMap<K, V> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BTreeMap::is_empty(self)
    }
}

impl<'a, K, V> IsEmpty for &'a ::std::collections::BTreeMap<K, V> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BTreeMap::is_empty(self)
    }
}

impl<T: Ord> IsEmpty for ::std::collections::BTreeSet<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BTreeSet::is_empty(self)
    }
}

impl<'a, T: Ord> IsEmpty for &'a ::std::collections::BTreeSet<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BTreeSet::is_empty(self)
    }
}

impl<T: Ord> IsEmpty for ::std::collections::BinaryHeap<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BinaryHeap::is_empty(self)
    }
}

impl<'a, T: Ord> IsEmpty for &'a ::std::collections::BinaryHeap<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        ::std::collections::BinaryHeap::is_empty(self)
    }
}

impl IsEmpty for i8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for isize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for usize {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for f32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0_f32
    }
}

impl IsEmpty for f64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0_f64
    }
}