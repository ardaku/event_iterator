#![allow(unsafe_code)]

use core::{ops::DerefMut, pin::Pin};

pub(crate) struct PinIntoDerefMut<'a, Ptr>(pub(crate) Pin<&'a mut Pin<Ptr>>)
where
    Ptr: DerefMut;

impl<'a, Ptr> PinIntoDerefMut<'a, Ptr>
where
    Ptr: DerefMut,
{
    pub(crate) fn into_deref_mut(self) -> Pin<&'a mut Ptr::Target> {
        unsafe { self.0.get_unchecked_mut() }.as_mut()
    }
}
