use std::marker::PhantomData;

/// An owned value which has type name Name. This type
/// is unique to this Named, so it can be used to
/// enforce compile-time coherency
pub struct Named<T, Name> {
    inner: T,
    _phantom: PhantomData<Name>,
}

/// Create a named value. You probably don't want to use
/// this, but instead want to use the name!() macro that
/// calls this.
///
/// Safety:
/// Must make sure Name is not used as the name for any
/// other value of type Named<T, Name>
pub unsafe fn name<Name, T>(val: T) -> Named<T, Name> {
    Named {
        inner: val,
        _phantom: PhantomData,
    }
}

impl<T, Name> Named<T, Name> {
    pub fn unname(self) -> T {
        self.inner
    }

    pub fn unname_ref(&self) -> &T {
        &self.inner
    }

    /// Safety:
    /// Must uphold whatever invariant the Named protects
    pub unsafe fn unname_ref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

#[macro_export]
macro_rules! name {
    ($val:expr) => {{
        struct UniqueName {};

        unsafe {
            // Nothing else is named UniqueName because
            // we just defined it
            name::<UniqueName, _>($val)
        }
    }}
}
