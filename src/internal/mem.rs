use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use nanny_sys::raw;
use internal::value::{Any, AnyInternal, SuperType};
use internal::error::TypeError;
use internal::vm::JS;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Handle<'a, T: Any + 'a> {
    value: T,
    phantom: PhantomData<&'a T>
}

pub trait HandleInternal<'a, T: Any + 'a> {
    fn new(value: T) -> Handle<'a, T>;
    fn to_raw_mut_ref(&mut self) -> &mut raw::Local;
}

impl<'a, T: Any + 'a> HandleInternal<'a, T> for Handle<'a, T> {
    fn new(value: T) -> Handle<'a, T> {
        Handle {
            value: value,
            phantom: PhantomData
        }
    }

    fn to_raw_mut_ref(&mut self) -> &mut raw::Local {
        match self {
            &mut Handle { ref mut value, .. } => {
                value.to_raw_mut_ref()
            }
        }
    }
}

impl<'a, T: Any> Handle<'a, T> {
    // This method does not require a scope because it only copies a handle.
    pub fn upcast<U: Any + SuperType<T>>(&self) -> Handle<'a, U> {
        Handle::new(SuperType::upcast_internal(self.value))
    }
}

impl<'a, T: Any> Handle<'a, T> {
    pub fn downcast<U: Any>(&self) -> Option<Handle<'a, U>> {
        U::downcast(self.value).map(Handle::new)
    }

    pub fn check<U: Any>(&self) -> JS<'a, U> {
        match U::downcast(self.value) {
            Some(v) => Ok(Handle::new(v)),
            None => TypeError::throw("type error")
        }
    }
}

impl<'a, T: Any> Deref for Handle<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &self.value
    }
}

impl<'a, T: Any> DerefMut for Handle<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut self.value
    }
}