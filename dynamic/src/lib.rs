//! # dynamic
//! Inheritance in rust
#![no_std]
#![allow(non_snake_case)]

use core::{marker::PhantomData, ops::{Deref, DerefMut}};
extern crate alloc;
use alloc::boxed::Box;
pub mod typing;
pub use typing::*;

pub trait Dyn {

}

impl<T> Dyn for T {

}

/// Implemented by #[subclass] macro
pub trait Class {
      type Parent: Sized + Class;
      const NAME: &'static str;

      fn id() -> usize;
      fn offset() -> isize;
      fn isa(id: usize) -> bool;
}

/// An object
/// The ContainerT generic parameter controls the container of the object's pointer
/// Dereference it (*object) to get the inner class
pub struct Object<T: Class, ContainerT = Box<dyn Dyn>> {
      // A pointer to the object created
      // It should *always* point to the object created so it can be correctly freed
      object: ContainerT,
      isa: fn(id: usize) -> bool,
      offset: i16,
      _marker: PhantomData<T>
}

impl<T: Class, ContainerT> Object<T, ContainerT> {
      /// Constructs an object from a container
      /// 
      /// # Example:
      /// ```
      /// #[subclass(DynamicObjectBase)]
      /// struct MyObject;
      /// 
      /// let object = Object::<MyObject>::new(Box::new(MyObject {}));
      /// ```
      pub fn new(object: ContainerT) -> Self {
            Self {
                  object,
                  isa: T::isa,
                  offset: 0,
                  _marker: PhantomData
            }
      }

      /// Check if the object is a child of Other or is type Other
      /// 
      /// # Example:
      /// ```
      ///  #[subclass(DynamicObjectBase)]
      /// struct MyObject;
      /// 
      /// let object = Object::<MyObject>::new(Box::new(MyObject {}));
      /// assert!(object.isa::<DynamicObjectBase::>());
      /// ```
      pub fn isa<Other: Class>(&self) -> bool {
            (self.isa)(Other::id())
      }

      /// Cast to type 'Cast'
      /// panic if 'self' does not inherit from 'Cast'/ is not 'Cast'
      /// 
      ///  # Example:
      /// ```
      /// #[subclass(DynamicObjectBase)]
      /// struct Class {
      ///       value: u32,
      ///       foo: u32
      /// }
      /// #[subclass(Class, parent)]
      /// struct Derived {
      ///       field: u32,
      ///       parent: Class,
      /// }
      /// 
      /// let object = Derived {
      ///       parent: Class {
      ///             value: 548389,
      ///             foo: 72840548
      ///       },
      ///       field: 2153746,
      /// };
      /// 
      /// let object = Object::<Derived>::new(Box::new(object));
      /// 
      /// assert!(object.field == 2153746);
      /// assert!(object.parent.value == 548389);
      /// assert!(object.parent.foo == 72840548);
      /// 
      /// let object = object.cast::<Class>();
      /// assert!(object.value == 548389);
      /// assert!(object.foo == 72840548);
      /// 
      /// let object = object.cast::<Derived>();
      /// assert!(object.field == 2153746);
      /// assert!(object.parent.value == 548389);
      /// assert!(object.parent.foo == 72840548);
      /// ```
      pub fn cast<Cast: Class>(self) -> Object<Cast, ContainerT> {
            if isSubclassOf::<Cast, T>() {
                  assert!(self.isa::<Cast>());
            }
            let offset = if isSubclassOf::<Cast, T>() {
                  -typing::offsetOf::<T, Cast>()
            }
            else {
                  typing::offsetOf::<Cast, T>()
            } as i16;
            Object {
                  object: self.object,
                  isa: self.isa,
                  _marker: PhantomData,
                  offset: self.offset + offset
            }
      }

      /// Try to cast to 'Cast'
      pub fn try_cast<Cast: Class>(self) -> Option<Object<Cast, ContainerT>> {
            if isSubclassOf::<Cast, T>() {
                  if !self.isa::<Cast>() {
                        return None
                  }
            }
            
            let offset = if isSubclassOf::<Cast, T>() {
                  -typing::offsetOf::<T, Cast>()
            }
            else {
                  typing::offsetOf::<Cast, T>()
            } as i16;
            Some(Object {
                  object: self.object,
                  isa: self.isa,
                  _marker: PhantomData,
                  offset: self.offset + offset
            })
      }
}

impl<T: Class, Container: Deref> Deref for Object<T, Container> {
      type Target = T;

      fn deref(&self) -> &Self::Target {
            let inner = &*self.object as *const Container::Target as *const ();
            let inner = inner as usize;
            let inner = inner.wrapping_add(self.offset as usize);
            let inner = inner as *const T;
            unsafe {
                  &*inner
            }
      }
}

impl<T: Class, Container: DerefMut> DerefMut for Object<T, Container> {
      fn deref_mut(&mut self) -> &mut Self::Target {
            let inner = &mut *self.object as *mut Container::Target as *mut ();
            let inner = inner as usize;
            let inner = inner.wrapping_add(self.offset as usize);
            let inner = inner as *mut T;
            unsafe {
                  &mut *inner
            }
      }
}

impl<T: Class, Container: Clone> Clone for Object<T, Container> {
      fn clone(&self) -> Self {
            Self { 
                  object: self.object.clone(), 
                  isa: self.isa, 
                  offset: self.offset, 
                  _marker: PhantomData
            }
      }
}

impl<T: Class, Container: Copy> Copy for Object<T, Container> {
      
}

/// Base class for all objects
/// Inherit from it for your base classes
/// 
/// # Example: 
/// ```
/// #[subclass(DynamicObjectBase)]
/// struct MyObject;
/// ```
pub struct DynamicObjectBase;

impl DynamicObjectBase {
      const ID: u8 = 0;
}

impl Class for DynamicObjectBase {
      type Parent = Self;
      const NAME: &'static str = "dynamic::ObjectBase";

      fn isa(id: usize) -> bool {
            id == Self::id()
      }
      
      fn offset() -> isize {
            0
      }

      fn id() -> usize {
            &Self::ID as *const u8 as usize
      }
}

#[cfg(test)]
mod test {
      #![allow(unused_imports)]
      #![allow(dead_code)]

      extern crate std;
      use dynamic_derive::*;
      // hack: macro uses dynamic::Object
      use crate as dynamic;
      use crate::*;
      // use std only when testing
      use std::{println, prelude::*};

      #[subclass(DynamicObjectBase)]
      struct Class {
            value: u32,
            foo: u32
      }

      #[subclass(Class, parent)]
      struct Derived {
            field: u32,
            parent: Class,
      }

      #[test]
      fn isa() {
            let object = Derived { 
                  parent: Class {
                        value: 0,
                        foo: 0
                  },
                  field: 0
            };
            let object = Object::<Derived>::new(Box::new(object));
            assert!(object.isa::<DynamicObjectBase>());
      }

      #[test]
      fn casting() {
            let object = Derived {
                  parent: Class {
                        value: 548389,
                        foo: 72840548
                  },
                  field: 2153746,
            };
            
            let object = Object::<Derived>::new(Box::new(object));
            assert!(object.field == 2153746);
            assert!(object.parent.value == 548389);
            assert!(object.parent.foo == 72840548);
            
            let object = object.cast::<Class>();
            println!("Parent offset: {}", object.offset);
            println!("After cast: {}", object.value);
            assert!(object.value == 548389);
            assert!(object.foo == 72840548);
            
            let object = object.cast::<Derived>();
            assert!(object.field == 2153746);
            assert!(object.parent.value == 548389);
            assert!(object.parent.foo == 72840548);
      }
}
