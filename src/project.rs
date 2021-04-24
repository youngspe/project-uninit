/// Obtain `&MaybeUninit<_>` references to fields of a struct wrapped in `MaybeUninit<_>`.
///
/// This must be used in an `unsafe` block or function when accessing fields of unions.
///
/// ## Syntax
/// ```
/// # use core::mem::MaybeUninit;
/// # use project_uninit::project_uninit;
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Person { name: Name, age: u32, id: (usize, usize) }
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Name { first: &'static str, last: &'static str }
/// # let mut bob = MaybeUninit::new(Person {
/// #     name: Name { first: "Bob1", last: "Jones1" },
/// #     age: 34, id: (111, 222),
/// # });
/// // Access a single field:
/// let age: &MaybeUninit<u32> = project_uninit!(bob => age);
///
/// // Access multiple fields:
/// let (age, id): (&MaybeUninit<_>, &MaybeUninit<_>) = project_uninit!(
///     bob => { age, id }
/// );
///
/// // Access fields of fields:
/// let first: &MaybeUninit<&str> = project_uninit!(bob => name => first);
///
/// // Access fields of tuples (also works for tuple structs):
/// let id0: &MaybeUninit<usize> = project_uninit!(bob => id => 0);
///
/// // Access multiple fields, including nested fields:
/// let (first, last, age, id0, id1) = project_uninit!(bob => {
///     name => first,
///     name => last,
///     age,
///     id => 0,
///     id => 1,
/// });
/// ```
///
/// # Example
/// ```
/// use core::mem::MaybeUninit;
/// use project_uninit::project_uninit;
///
/// #[derive(PartialEq, Eq, Debug)]
/// struct Person { name: Name, age: u32, id: (usize, usize) }
/// #[derive(PartialEq, Eq, Debug)]
/// struct Name { first: &'static str, last: &'static str }
///
/// let alice = MaybeUninit::new(Person {
///     name: Name { first: "Alice", last: "Smith" },
///     age: 22,
///     id: (123, 456),
/// });
///
/// let first = project_uninit!(alice => name => first);
/// assert_eq!(unsafe { first.assume_init() }, "Alice");
///
/// let (last, age, id1) = project_uninit!(alice => {
///     name => last,
///     age,
///     id => 1,
/// });
///
/// assert_eq!(unsafe { last.assume_init() }, "Smith");
/// assert_eq!(unsafe { age.assume_init() }, 22);
/// assert_eq!(unsafe { id1.assume_init() }, 456);
/// ```
///
#[macro_export]
macro_rules! project_uninit {
    // project mutliple fields
    ($expr:expr => {$( $($props:tt)=>+ ),* $(,)?}) => {{
        #[allow(unused_imports)]
        use ::core::borrow::Borrow;
        let _ref: &::core::mem::MaybeUninit<_> = $expr.borrow();
        let ptr = ::core::mem::MaybeUninit::as_ptr(_ref);
        let lt = $crate::utils::bind_ref_lt(_ref);

        if false {
            // this will never be executed
            // it's only to assert that it is safe to access the fields
            #[allow(unused_unsafe)]
            let _x = unsafe { &*ptr };
            let _y = ($(&_x.$($props).+,)*);
        }

        ($({
            let ret;
            #[allow(unused_unsafe)]
            unsafe {
                let prop_ptr = ::core::ptr::addr_of!((*ptr).$($props).+);
                ret = $crate::utils::uninit_from_ptr(prop_ptr, lt);
            }
            ret
        },)*)
    }};

    // project a single field
    ($expr:expr => $($props:tt)=>+) => {
        $crate::project_uninit!($expr => {$($props)=>+}).0
    };
}

/// Obtain `&mut MaybeUninit<_>` references to fields of a struct wrapped in `MaybeUninit<_>`.
///
/// This statically ensures that multiple references to the same value are not returned.
///
/// This must be used in an `unsafe` block or function when accessing fields of unions.
///
/// ## Syntax
/// ```
/// # use core::mem::MaybeUninit;
/// # use project_uninit::project_uninit_mut;
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Person { name: Name, age: u32, id: (usize, usize) }
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Name { first: &'static str, last: &'static str }
/// # let mut bob = MaybeUninit::new(Person {
/// #     name: Name { first: "Bob1", last: "Jones1" },
/// #     age: 34, id: (111, 222),
/// # });
/// // Access a single field:
/// let age: &mut MaybeUninit<u32> = project_uninit_mut!(bob => age);
///
/// // Access multiple fields:
/// let (age, id): (&mut MaybeUninit<_>, &mut MaybeUninit<_>) = project_uninit_mut!(
///     bob => { age, id }
/// );
///
/// // Access fields of fields:
/// let first: &mut MaybeUninit<&str> = project_uninit_mut!(bob => name => first);
///
/// // Access fields of tuples (also works for tuple structs):
/// let id0: &mut MaybeUninit<usize> = project_uninit_mut!(bob => id => 0);
///
/// // Access multiple fields, including nested fields:
/// let (first, last, age, id0, id1) = project_uninit_mut!(bob => {
///     name => first,
///     name => last,
///     age,
///     id => 0,
///     id => 1,
/// });
/// ```
///
/// # Example
/// ```
/// use core::mem::MaybeUninit;
/// use project_uninit::project_uninit_mut;
///
/// #[derive(PartialEq, Eq, Debug)]
/// struct Person { name: Name, age: u32, id: (usize, usize) }
/// #[derive(PartialEq, Eq, Debug)]
/// struct Name { first: &'static str, last: &'static str }
///
/// let mut alice = MaybeUninit::<Person>::uninit();
///
/// let first = project_uninit_mut!(alice => name => first);
/// *first = MaybeUninit::new("Alice");
///
/// let (last, age, id0, id1) = project_uninit_mut!(alice => {
///     name => last,
///     age,
///     id => 0,
///     id => 1,
/// });
///
/// *last = MaybeUninit::new("Smith");
/// *age = MaybeUninit::new(22);
/// *id0 = MaybeUninit::new(123);
/// *id1 = MaybeUninit::new(456);
///
/// assert_eq!(unsafe { alice.assume_init() }, Person {
///     name: Name { first: "Alice", last: "Smith" },
///     age: 22,
///     id: (123, 456),
/// });
/// ```
///
#[macro_export]
macro_rules! project_uninit_mut {
    // project mutliple fields
    ($expr:expr => {$( $($props:tt)=>+ ),* $(,)?}) => {{
        // generate an error message if a field is used more than once
        $crate::__assert_unique!($expr, [ $( [ $($props).+ ] )* ]);
        #[allow(unused_imports)]
        use ::core::borrow::BorrowMut;
        let _ref: &mut ::core::mem::MaybeUninit<_> = $expr.borrow_mut();
        let ptr = ::core::mem::MaybeUninit::as_mut_ptr(_ref);
        let lt = $crate::utils::bind_mut_lt(_ref);

        if false {
            // this will never be executed
            // it's only to assert that it is safe to access the fields
            #[allow(unused_unsafe)]
            let _x = unsafe { &mut *ptr };
            let _y = ($(&mut _x.$($props).+,)*);
        }
        ($({
            let ret;
            #[allow(unused_unsafe)]
            unsafe {
                let prop_ptr = ::core::ptr::addr_of_mut!((*ptr).$($props).+);
                ret = $crate::utils::uninit_from_mut_ptr(prop_ptr, lt);
            }
            ret
        },)*)
    }};

    // project a single field
    ($expr:expr => $($props:tt)=>+) => {
        $crate::project_uninit_mut!($expr => {$($props)=>+}).0
    };
}

/// **Unsafe:** Given a `*const` pointer to a struct, obtain `*const` pointers to one or more of its fields.
///
/// This does **not** statically check whether multiple pointers to the same data are returned.
/// This must be used in an `unsafe` block or function.
///
/// ## Usage
/// ```
/// # use core::mem::MaybeUninit;
/// # use project_uninit::project_ptr;
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Person { name: Name, age: u32, id: (usize, usize) }
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Name { first: &'static str, last: &'static str }
///
/// let bob = Person {
///     name: Name { first: "Bob", last: "Jones" },
///     age: 35,
///     id: (111, 222),
/// };
/// let bob_ptr: *const Person = &bob;
///
/// unsafe {
///     // Pointer to a single field:
///     let age: *const u32 = project_ptr!(bob_ptr => age);
///     assert_eq!(*age, 35);
///
///     // Pointers to multiple fields:
///     let (first, name, id0): (*const &str, *const Name, *const usize) = project_ptr!(
///         bob_ptr => { name => first, name, id => 0 }
///     );
///     assert_eq!(*first, "Bob");
///     assert_eq!(*name, Name { first: "Bob", last: "Jones" });
///     assert_eq!(*id0, 111);
/// }
///
/// ```
#[macro_export]
macro_rules! project_ptr {
    // project mutliple fields
    ($expr:expr => {$( $($props:tt)=>+ ),* $(,)?}) => {{
        let ptr: *const _ = $expr;
        ($(
            ::core::ptr::addr_of!((*ptr).$($props).+),
        )*)
    }};

    // project a single field
    ($expr:expr => $($props:tt)=>+) => {
        $crate::project_ptr!($expr => {$($props)=>+}).0
    };
}

/// **Unsafe:** Given a `*mut` pointer to a struct, obtain `*mut` pointers to one or more of its fields.
///
/// This does **not** statically check whether multiple pointers to the same data are returned.
/// This must be used in an `unsafe` block or function.
///
/// ## Usage
/// ```
/// # use core::mem::MaybeUninit;
/// # use project_uninit::project_ptr_mut;
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Person { name: Name, age: u32, id: (usize, usize) }
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Name { first: &'static str, last: &'static str }
///
/// let mut bob = Person {
///     name: Name { first: "Bob", last: "Jones" },
///     age: 35,
///     id: (111, 222),
/// };
/// let bob_ptr: *mut Person = &mut bob;
///
/// unsafe {
///     // Pointer to a single field:
///     let age: *mut u32 = project_ptr_mut!(bob_ptr => age);
///     *age = 36;
///
///     // Pointers to multiple fields:
///     let (first, last, id0): (*mut &str, *mut &str, *mut usize) = project_ptr_mut!(
///         bob_ptr => { name => first, name => last, id => 0 }
///     );
///     *first = "Robert";
///     *last = "Johns";
///     *id0 = 444;
///     assert_eq!(bob, Person {
///         name: Name { first: "Robert", last: "Johns" },
///         age: 36,
///         id: (444, 222),
///     });
/// }
/// ```
#[macro_export]
macro_rules! project_ptr_mut {
    // project mutliple fields
    ($expr:expr => {$( $($props:tt)=>+ ),* $(,)?}) => {{
        let ptr: *mut _ = $expr;
        ($(
            ::core::ptr::addr_of_mut!((*ptr).$($props).+),
        )*)
    }};

    // project a single field
    ($expr:expr => $($props:tt)=>+) => {
        $crate::project_ptr_mut!($expr => {$($props)=>+}).0
    };
}

///```compile_fail
/// use project_uninit::project_uninit_mut;
/// use core::mem::MaybeUninit;
/// struct Foo { a: i32, b: u32 }
/// let mut x = MaybeUninit::<Foo>::uninit();
/// let (a, b, a2) = project_uninit_mut!(x => { a, b, a });
///```
fn _test_multiple_per_mut_macro_call_fails() {}

///```compile_fail
/// use project_uninit::{project_uninit, project_uninit_mut};
/// use core::mem::MaybeUninit;
/// struct Foo { a: i32, b: u32 }
/// let mut x = MaybeUninit::<Foo>::uninit();
/// let a = project_uninit!(x => a);
/// let (a2, b) = project_uninit_mut!(x => { a, b });
/// let aa = a;
///```
fn _project_mut_with_existing_borrow_fails() {}
