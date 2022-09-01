use core::marker::PhantomData;

use crate::{Class, DynamicObjectBase};

const fn equal(a: &str, b: &str) -> bool {
      let a = a.as_bytes();
      let b = b.as_bytes();

      let mut i = 0;
      if a.len() != b.len() {
            return false;
      }
      while i < a.len() {
            if a[i] != b[i] {
                  return false;
            }
            i += 1;
      }
      true
}

pub trait ConditionalTrait {
      type Type;      
}

pub struct ConditionalTImpl<const CONDITION: bool, TrueT, FalseT> {
      true_t: PhantomData<TrueT>,
      false_t: PhantomData<FalseT>
}

impl<TrueT, FalseT> ConditionalTrait for ConditionalTImpl<true, TrueT, FalseT> {
      type Type = TrueT;
}

impl<TrueT, FalseT> ConditionalTrait for ConditionalTImpl<false, TrueT, FalseT> {
      type Type = FalseT;     
}

impl<const CONDITION: bool, TrueT, FalseT> ConditionalTImpl<CONDITION, TrueT, FalseT> {
      pub const VALUE: bool = CONDITION;
}

pub type ConditionalT<const VALUE: bool, TrueT, FalseT> = 
      <ConditionalTImpl<VALUE, TrueT, FalseT> as ConditionalTrait>::Type;

pub struct IsSameClass<A: Class, B: Class> {
      _marker1: PhantomData<A>,
      _marker2: PhantomData<B>
}

impl<A: Class, B: Class> IsSameClass<A, B> {
      pub const VALUE: bool = equal(A::NAME, B::NAME);
}

pub const fn isSubclassOf<Child: Class, Parent: Class>() -> bool {
      if IsSameClass::<Child, Parent>::VALUE {
            return true
      }
      else if IsSameClass::<Child, DynamicObjectBase>::VALUE {
            false
      }
      else {
            isSubclassOf::<Child::Parent, Parent>()
      }
}

pub fn offsetOf<Parent: Class, Child: Class>() -> isize {
      let mut offset = Child::offset();
      if !IsSameClass::<Parent, Child>::VALUE {
            offset += offsetOf::<Parent, Child::Parent>();
      }
      offset
}

#[cfg(test)]
mod test {
      use dynamic_derive::subclass;
      use crate::{self as dynamic, isSubclassOf};
      use crate::{IsSameClass, DynamicObjectBase};

      #[subclass(DynamicObjectBase)]
      struct MyClass {

      }

      #[test]
      fn isSameClass() {
            assert!(IsSameClass::<DynamicObjectBase, DynamicObjectBase>::VALUE);
            assert!(!IsSameClass::<MyClass, DynamicObjectBase>::VALUE);
            assert!(IsSameClass::<MyClass, MyClass>::VALUE);
      }

      #[test]
      fn subclassOf() {
            assert!(isSubclassOf::<MyClass, DynamicObjectBase>());
      }

      #[test]
      fn offsetof() {
            #[repr(C)]
            struct A {
                  value: u32,
                  value2: u32
            }
            let a = 0 as *mut A;
            let ptr: *mut u32 = unsafe { &mut ((*a).value2) };
            let ptr = ptr as usize;
            assert!(ptr == 4);
      }
}
