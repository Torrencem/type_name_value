//! Exposes the ``name!()`` macro, which can be used to assign unique types to values. This can be
//! used to implement the "Ghosts of Departed Proofs" pattern.

use std::marker::PhantomData;

#[macro_use]
extern crate derivative;

/// An owned value which has name Name. This type
/// is unique to this Named, so it can be used to
/// enforce compile-time coherency
#[derive(Derivative)]
#[derivative(Debug(bound="T: std::fmt::Debug"), PartialEq(bound="T: PartialEq"), Eq(bound="T: Eq"), Hash(bound="T: std::hash::Hash"), PartialOrd(bound="T: PartialOrd"), Ord(bound="T: Ord"))]
pub struct Named<T, Name> {
    inner: T,
    _phantom: PhantomData<Name>,
}

/// Create a named value. You probably don't want to use
/// this, but instead want to use the name!() macro that
/// calls this.
///
/// # Safety
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

    /// # Safety
    /// Must uphold whatever invariant the Named protects
    pub unsafe fn unname_ref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// Create a ``Named`` with a given value. This will create an anonymous type that's unique to the
/// macro invokation. Note that since this gives a type that's impossible to name properly, use
/// wildcards and generics when passing them around.
///
/// # Examples
///
/// ```
/// # use type_name_value::*;
/// let x: Named<u32, _> = name!(5);
/// ```
///
/// ```
/// # use type_name_value::*;
/// fn is_five<Name>(val: Named<u32, Name>) -> bool {
///     *val.unname_ref() == 5
/// }
/// ```
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
