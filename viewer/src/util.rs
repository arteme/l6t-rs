use std::cell::Ref;

/// A `Ref::map()` variant that lets us escape the borrow checker.
///
/// Given `r: Ref<T>` and `v: &U`, which is a part of borrowed `&T`,
/// there's no way we can use `Ref::map(r, ...)` to create a `Ref<U>`
/// without this trickery.
pub fn ref_remap<'a, T, U>(r: &Ref<'a, T>, v: &U) -> Ref<'a, U> {
    let r = Ref::clone(&r);
    let v = v as *const U;
    // SAFETY: r is borrowed and v is part of r
    Ref::map(r, |_| unsafe { &*v })
}