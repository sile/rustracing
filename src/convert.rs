pub trait MaybeAsRef<T: ?Sized> {
    fn maybe_as_ref(&self) -> Option<&T>;
}
