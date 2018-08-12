//! Traits for conversions between types.

/// A cheap reference-to-reference conversion that has a possibility to fail.
pub trait MaybeAsRef<T: ?Sized> {
    /// Performs the conversion.
    fn maybe_as_ref(&self) -> Option<&T>;
}
impl<T, U> MaybeAsRef<T> for Option<U>
where
    U: MaybeAsRef<T>,
{
    fn maybe_as_ref(&self) -> Option<&T> {
        self.as_ref().and_then(|u| u.maybe_as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct Foo;
        struct Bar(Foo);
        impl MaybeAsRef<Foo> for Bar {
            fn maybe_as_ref(&self) -> Option<&Foo> {
                Some(&self.0)
            }
        }

        let bar = Bar(Foo);
        assert!(bar.maybe_as_ref().is_some());
    }
}
