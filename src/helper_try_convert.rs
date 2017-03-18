use super::{NonEmpty, TryNonEmpty};

/////////////////////////////////////////////////////////////////////////
// Helper functions to convert multiple values at once to NonEmpty
/////////////////////////////////////////////////////////////////////////

/// Convert two values to a tuple of `NonEmpty`'s or fail.
pub fn try_non_empty2<A, B>(a: A, b: B) -> Option<(NonEmpty<A>, NonEmpty<B>)>
    where A: TryNonEmpty,
          B: TryNonEmpty,
{
    if let (Some(a), Some(b)) = (a.try_non_empty(), b.try_non_empty()) {
        return Some((a, b))
    }
    return None
}

/// Convert three values to a tuple of `NonEmpty`'s or fail.
pub fn try_non_empty3<A, B, C>(a: A, b: B, c: C)
    -> Option<(NonEmpty<A>, NonEmpty<B>, NonEmpty<C>)>
    where A: TryNonEmpty,
          B: TryNonEmpty,
          C: TryNonEmpty,
{
    if let (Some(a), Some(b), Some(c)) = (a.try_non_empty(), b.try_non_empty(), c.try_non_empty()) {
        return Some((a, b, c))
    }
    return None
}

/// Convert four values to a tuple of `NonEmpty`'s or fail.
pub fn try_non_empty4<A, B, C, D>(a: A, b: B, c: C, d: D)
    -> Option<(NonEmpty<A>, NonEmpty<B>, NonEmpty<C>, NonEmpty<D>)>
    where A: TryNonEmpty,
          B: TryNonEmpty,
          C: TryNonEmpty,
          D: TryNonEmpty,
{
    if let (Some(a), Some(b), Some(c), Some(d)) = (
        a.try_non_empty(),
        b.try_non_empty(),
        c.try_non_empty(),
        d.try_non_empty(),
    ) {
        return Some((a, b, c, d))
    }
    return None
}

/// Convert five values to a tuple of `NonEmpty`'s or fail.
pub fn try_non_empty5<A, B, C, D, E>(a: A, b: B, c: C, d: D, e: E)
    -> Option<(NonEmpty<A>, NonEmpty<B>, NonEmpty<C>, NonEmpty<D>, NonEmpty<E>)>
    where A: TryNonEmpty,
          B: TryNonEmpty,
          C: TryNonEmpty,
          D: TryNonEmpty,
          E: TryNonEmpty,
{
    if let (Some(a), Some(b), Some(c), Some(d), Some(e)) = (
        a.try_non_empty(),
        b.try_non_empty(),
        c.try_non_empty(),
        d.try_non_empty(),
        e.try_non_empty(),
    ) {
        return Some((a, b, c, d, e))
    }
    return None
}

/// Convert six values to a tuple of `NonEmpty`'s or fail.
pub fn try_non_empty6<A, B, C, D, E, F>(a: A, b: B, c: C, d: D, e: E, f: F)
    -> Option<(NonEmpty<A>, NonEmpty<B>, NonEmpty<C>, NonEmpty<D>, NonEmpty<E>, NonEmpty<F>)>
    where A: TryNonEmpty,
          B: TryNonEmpty,
          C: TryNonEmpty,
          D: TryNonEmpty,
          E: TryNonEmpty,
          F: TryNonEmpty,
{
    if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) = (
        a.try_non_empty(),
        b.try_non_empty(),
        c.try_non_empty(),
        d.try_non_empty(),
        e.try_non_empty(),
        f.try_non_empty(),
    ) {
        return Some((a, b, c, d, e, f))
    }
    return None
}

#[allow(non_snake_case)]
/// Convert n values of the same type to a `Vec` of `NonEmpty`'s or fail.
pub fn try_non_emptyN<T, A>(a: A) -> Option<Vec<NonEmpty<T>>>
    where T: TryNonEmpty,
          A: Into<Vec<T>>
{
    let a = a.into();
    let input_len = a.len();
    let a = a.into_iter()
        .map(T::try_non_empty)
        .take_while(|v| v.is_some())
        .filter_map(|v| v)
        .collect::<Vec<_>>();
    if a.len() == input_len {
        return Some(a)
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_non_empty2() {
        assert!(try_non_empty2("a", 0).is_none());
        let (a, b) = try_non_empty2("a", 1).unwrap();
        assert_eq!("a", *a);
        assert_eq!(1, *b);
    }

    #[test]
    fn test_try_non_empty3() {
        assert!(try_non_empty3("a", 0, 5_f32).is_none());
        let (a, b, c) = try_non_empty3("a", 1, 5_f32).unwrap();
        assert_eq!("a", *a);
        assert_eq!(1, *b);
        assert_eq!(5_f32, *c);
    }

    #[test]
    fn test_try_non_empty4() {
        assert!(try_non_empty4("a", 0, 5_f32, 3).is_none());
        let (a, b, c, d) = try_non_empty4("a", 1, 5_f32, 3).unwrap();
        assert_eq!("a", *a);
        assert_eq!(1, *b);
        assert_eq!(5_f32, *c);
        assert_eq!(3, *d);
    }

    #[test]
    fn test_try_non_empty5() {
        assert!(try_non_empty5("a", 0, 5_f32, 3, "b").is_none());
        let (a, b, c, d, e) = try_non_empty5("a", 1, 5_f32, 3, "b").unwrap();
        assert_eq!("a", *a);
        assert_eq!(1, *b);
        assert_eq!(5_f32, *c);
        assert_eq!(3, *d);
        assert_eq!("b", *e);
    }

    #[test]
    fn test_try_non_empty6() {
        assert!(try_non_empty6("a", 0, 5_f32, 3, "b", 4_f64).is_none());
        let (a, b, c, d, e, f) = try_non_empty6("a", 1, 5_f32, 3, "b", 4_f64).unwrap();
        assert_eq!("a", *a);
        assert_eq!(1, *b);
        assert_eq!(5_f32, *c);
        assert_eq!(3, *d);
        assert_eq!("b", *e);
        assert_eq!(4_f64, *f);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_try_non_emptyN() {
        assert!(try_non_emptyN(vec![2,3,0]).is_none());
        let a = try_non_emptyN(vec![3,5]).unwrap();
        assert_eq!(*a[0], 3);
        assert_eq!(*a[1], 5);
    }
}