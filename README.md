# Dynamic-rs

Inheritance in rust

Add this to your ```Cargo.toml```
```toml
[dependencies]
dynamic-object = "0.1.0"
```

## Usages
```rust
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

// Downcast
let object = object.cast::<Class>();
assert!(object.value == 548389);
assert!(object.foo == 72840548);

// Upcast
let object = object.cast::<Derived>();
assert!(object.field == 2153746);
assert!(object.parent.value == 548389);
assert!(object.parent.foo == 72840548);
```