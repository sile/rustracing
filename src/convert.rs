//! Traits for conversions between types.

/// A cheap reference-to-reference conversion that has a possibility to fail.
pub trait MaybeAsRef<T: ?Sized> {
    /// Performs the conversion.
    fn maybe_as_ref(&self) -> Option<&T>;
}

#[cfg(test)]
mod test {
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
