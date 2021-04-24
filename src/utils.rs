use core::{marker::PhantomData, mem::MaybeUninit};

/// Invariant lifetime used to constrain the lifetime of a projected field reference.
#[derive(Clone, Copy)]
pub struct Lifetime<'a>(PhantomData<*mut &'a ()>);

pub fn bind_ref_lt<'a, T>(_: &'a T) -> Lifetime<'a> {
    Lifetime(PhantomData)
}

pub fn bind_mut_lt<'a, T>(_: &'a mut T) -> Lifetime<'a> {
    Lifetime(PhantomData)
}

pub unsafe fn uninit_from_ptr<'a, T>(
    ptr: *const T,
    _lt: Lifetime<'a>,
) -> &'a MaybeUninit<T> {
    &*(ptr as *const MaybeUninit<T>)
}

pub unsafe fn uninit_from_mut_ptr<'a, T>(
    ptr: *mut T,
    _lt: Lifetime<'a>,
) -> &'a mut MaybeUninit<T> {
    &mut *(ptr as *mut MaybeUninit<T>)
}

pub unsafe fn deref_ptr_with_lt<'a, T>(ptr: *mut T, _lt: Lifetime<'a>) -> &'a mut T {
    &mut *ptr
}
