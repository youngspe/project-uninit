# project-uninit

Rust macros for safe (and unsafe) access to and initialization of fields of structs wrapped in `MaybeUninit<_>`

This crate uses the
[`ptr::addr_of!`](https://doc.rust-lang.org/nightly/core/ptr/macro.addr_of.html)
and [`ptr::addr_of_mut!`](https://doc.rust-lang.org/nightly/core/ptr/macro.addr_of_mut.html)
macros introduced in Rust 1.51 to avoid undefined behavior.

See the [docs](https://youngspe.github.io/project-uninit/project_uninit/) for more information and examples.

## Examples
### Initialize a struct one field at a time
```rust
use core::mem::MaybeUninit;
use project_uninit::partial_init;
#[derive(PartialEq, Eq, Debug)]
struct Inner { value1: u8, value2: (i32, bool) }
#[derive(PartialEq, Eq, Debug)]
struct MyStruct { name: &'static str, inner: Inner }

let mut target = MaybeUninit::<MyStruct>::uninit();

let name: &mut &str = partial_init!(target => name = "Foo");
assert_eq!(*name, "Foo");
*name = "Bar";

let (value1, value2_0): (&mut u8, &mut i32) = partial_init!(target => {
    inner => value1: 0xff,
    inner => value2 => 0: 1000,
});
assert_eq!(*value1, 0xff);
assert_eq!(*value2_0, 1000);
*value2_0 *= 2;

let value2_1: &mut bool = partial_init!(target => inner => value2 => 1 = true);
assert_eq!(*value2_1, true);

assert_eq!(unsafe { target.assume_init() }, MyStruct {
    name: "Bar",
    inner: Inner { value1: 0xff, value2: (2000, true) },
});
```

### Obtain references to fields of a `MaybeUninit<_>` struct
```rust
# use core::mem::MaybeUninit;
use project_uninit::project_uninit;

#[derive(PartialEq, Eq, Debug)]
struct Person { name: &'static str, age: u32 }
let person = MaybeUninit::new(Person {
    name: "Alice",
    age: 22,
});
let (name, age): (&MaybeUninit<&str>, &MaybeUninit<u32>) = project_uninit!(
    person => { name, age }
);

assert_eq!(unsafe { name.assume_init() }, "Alice");
assert_eq!(unsafe { age.assume_init() }, 22);

```

### Obtain mutable references to fields of a `MaybeUninit<_>` struct
```rust
# use core::mem::MaybeUninit;
# #[derive(PartialEq, Eq, Debug)]
# struct Person { name: &'static str, age: u32 }
use project_uninit::project_uninit_mut;

let mut person = MaybeUninit::new(Person {
    name: "Alice",
    age: 22,
});

let (name, age): (&mut MaybeUninit<&str>, &mut MaybeUninit<u32>) = project_uninit_mut!(
    person => { name, age }
);

*name = MaybeUninit::new("Alicia");
*age = MaybeUninit::new(24);

assert_eq!(unsafe { person.assume_init() }, Person {
    name: "Alicia",
    age: 24,
});
```
