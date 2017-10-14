pub trait MaybeAsRef<T: ?Sized> {
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
