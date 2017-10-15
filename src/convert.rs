//! Traits for conversions between types.

/// A cheap reference-to-reference conversion that has a possibility to fail.
pub trait MaybeAsRef<T: ?Sized> {
    /// Performs the conversion.
    fn maybe_as_ref(&self) -> Option<&T>;
}
impl<'a, T> MaybeAsRef<T> for &'a T
where
    T: MaybeAsRef<T>,
{
    fn maybe_as_ref(&self) -> Option<&T> {
        (*self).maybe_as_ref()
    }
}
