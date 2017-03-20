use super::NonEmpty;

impl Into<String> for NonEmpty<String> {
    #[inline]
    fn into(self) -> String {
        self.into_inner()
    }
}

impl<'a> Into<&'a String> for NonEmpty<&'a String> {
    #[inline]
    fn into(self) -> &'a String {
        self.into_inner()
    }
}

impl<'a> Into<&'a str> for NonEmpty<&'a str> {
    #[inline]
    fn into(self) -> &'a str {
        self.into_inner()
    }
}

impl<'a> Into<&'a ::std::ffi::OsStr> for NonEmpty<&'a ::std::ffi::OsStr> {
    #[inline]
    fn into(self) -> &'a ::std::ffi::OsStr {
        self.into_inner()
    }
}

impl<'a> Into<&'a ::std::path::Path> for NonEmpty<&'a ::std::path::Path> {
    #[inline]
    fn into(self) -> &'a ::std::path::Path {
        self.into_inner()
    }
}

impl Into<::std::path::PathBuf> for NonEmpty<::std::path::PathBuf> {
    #[inline]
    fn into(self) -> ::std::path::PathBuf {
        self.into_inner()
    }
}

impl<'a> Into<&'a ::std::path::PathBuf> for NonEmpty<&'a ::std::path::PathBuf> {
    #[inline]
    fn into(self) -> &'a ::std::path::PathBuf {
        self.into_inner()
    }
}

impl<T> Into<Vec<T>> for NonEmpty<Vec<T>> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a Vec<T>> for NonEmpty<&'a Vec<T>> {
    #[inline]
    fn into(self) -> &'a Vec<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a [T]> for NonEmpty<&'a [T]> {
    #[inline]
    fn into(self) -> &'a [T] {
        self.into_inner()
    }
}

impl<K, V, S> Into<::std::collections::HashMap<K, V, S>> for
    NonEmpty<::std::collections::HashMap<K, V, S>>
{
    #[inline]
    fn into(self) -> ::std::collections::HashMap<K, V, S> {
        self.into_inner()
    }
}

impl<'a, K, V, S> Into<&'a ::std::collections::HashMap<K, V, S>> for
    NonEmpty<&'a ::std::collections::HashMap<K, V, S>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::HashMap<K, V, S> {
        self.into_inner()
    }
}

impl<T, S> Into<::std::collections::HashSet<T, S>> for
    NonEmpty<::std::collections::HashSet<T, S>>
{
    #[inline]
    fn into(self) -> ::std::collections::HashSet<T, S> {
        self.into_inner()
    }
}

impl<'a, T, S> Into<&'a ::std::collections::HashSet<T, S>> for
    NonEmpty<&'a ::std::collections::HashSet<T, S>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::HashSet<T, S> {
        self.into_inner()
    }
}

impl<T> Into<::std::collections::LinkedList<T>> for
    NonEmpty<::std::collections::LinkedList<T>>
{
    #[inline]
    fn into(self) -> ::std::collections::LinkedList<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a ::std::collections::LinkedList<T>> for
    NonEmpty<&'a ::std::collections::LinkedList<T>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::LinkedList<T> {
        self.into_inner()
    }
}

impl<T> Into<::std::collections::VecDeque<T>> for
    NonEmpty<::std::collections::VecDeque<T>>
{
    #[inline]
    fn into(self) -> ::std::collections::VecDeque<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a ::std::collections::VecDeque<T>> for
    NonEmpty<&'a ::std::collections::VecDeque<T>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::VecDeque<T> {
        self.into_inner()
    }
}

impl<K, V> Into<::std::collections::BTreeMap<K, V>> for
    NonEmpty<::std::collections::BTreeMap<K, V>>
{
    #[inline]
    fn into(self) -> ::std::collections::BTreeMap<K, V> {
        self.into_inner()
    }
}

impl<'a, K, V> Into<&'a ::std::collections::BTreeMap<K, V>> for
    NonEmpty<&'a ::std::collections::BTreeMap<K, V>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::BTreeMap<K, V> {
        self.into_inner()
    }
}

impl<T> Into<::std::collections::BTreeSet<T>> for
    NonEmpty<::std::collections::BTreeSet<T>>
{
    #[inline]
    fn into(self) -> ::std::collections::BTreeSet<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a ::std::collections::BTreeSet<T>> for
    NonEmpty<&'a ::std::collections::BTreeSet<T>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::BTreeSet<T> {
        self.into_inner()
    }
}

impl<T> Into<::std::collections::BinaryHeap<T>> for
    NonEmpty<::std::collections::BinaryHeap<T>>
{
    #[inline]
    fn into(self) -> ::std::collections::BinaryHeap<T> {
        self.into_inner()
    }
}

impl<'a, T> Into<&'a ::std::collections::BinaryHeap<T>> for
    NonEmpty<&'a ::std::collections::BinaryHeap<T>>
{
    #[inline]
    fn into(self) -> &'a ::std::collections::BinaryHeap<T> {
        self.into_inner()
    }
}

impl Into<i8> for NonEmpty<i8> {
    #[inline]
    fn into(self) -> i8 {
        self.into_inner()
    }
}

impl Into<i16> for NonEmpty<i16> {
    #[inline]
    fn into(self) -> i16 {
        self.into_inner()
    }
}

impl Into<i32> for NonEmpty<i32> {
    #[inline]
    fn into(self) -> i32 {
        self.into_inner()
    }
}

impl Into<i64> for NonEmpty<i64> {
    #[inline]
    fn into(self) -> i64 {
        self.into_inner()
    }
}

impl Into<u8> for NonEmpty<u8> {
    #[inline]
    fn into(self) -> u8 {
        self.into_inner()
    }
}

impl Into<u16> for NonEmpty<u16> {
    #[inline]
    fn into(self) -> u16 {
        self.into_inner()
    }
}

impl Into<u32> for NonEmpty<u32> {
    #[inline]
    fn into(self) -> u32 {
        self.into_inner()
    }
}

impl Into<u64> for NonEmpty<u64> {
    #[inline]
    fn into(self) -> u64 {
        self.into_inner()
    }
}

impl Into<isize> for NonEmpty<isize> {
    #[inline]
    fn into(self) -> isize {
        self.into_inner()
    }
}

impl Into<usize> for NonEmpty<usize> {
    #[inline]
    fn into(self) -> usize {
        self.into_inner()
    }
}

impl Into<f32> for NonEmpty<f32> {
    #[inline]
    fn into(self) -> f32 {
        self.into_inner()
    }
}

impl Into<f64> for NonEmpty<f64> {
    #[inline]
    fn into(self) -> f64 {
        self.into_inner()
    }
}