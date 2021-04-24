/// Partially initialize a struct wrapped in `MaybeUninit`.
///
/// The specified fields will be updated with the given values, and mutable references to those
/// fields will be returned.
/// This statically ensures that the same field is not set mutltiple times in the same macro call,
/// and that multiple references to the same value are not returned.
///
/// This must be used in an `unsafe` block or function when accessing fields of unions.
///
/// ## Syntax
/// ```
/// # use core::mem::MaybeUninit;
/// # use project_uninit::partial_init;
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Person { name: Name, age: u32, id: (usize, usize) }
/// # #[derive(PartialEq, Eq, Debug)]
/// # struct Name { first: &'static str, last: &'static str }
/// # let mut bob = MaybeUninit::new(Person {
/// #     name: Name { first: "Bob1", last: "Jones1" },
/// #     age: 34, id: (111, 222),
/// # });
/// // Initialize a single field:
/// let age: &mut u32 = partial_init!(bob => age = 35);
/// assert_eq!(*age, 35);
///
/// // Initialize multiple fields:
/// let (age, id) = partial_init!(bob => {
///     age: 36,
///     id: (111, 222),
/// });
/// assert_eq!(*age, 36);
/// assert_eq!(*id, (111, 222));
///
/// // Initialize fields of fields:
/// let first: &mut &'static str = partial_init!(bob => name => first = "Bob");
/// assert_eq!(*first, "Bob");
///
/// // Initialize fields of tuples (also works for tuple structs):
/// let id0: &mut usize = partial_init!(bob => id => 0 = 444);
/// assert_eq!(*id0, 444);
///
/// // Initialize multiple fields, including nested fields:
/// let (last, age, id1) = partial_init!(bob => {
///     name => last: "Jones",
///     age: 37,
///     id => 1: 888,
/// });
///
/// assert_eq!(*last, "Jones");
/// assert_eq!(*age, 37);
/// assert_eq!(*id1, 888);
/// ```
///
///
/// ## Example
/// ```
/// use core::mem::MaybeUninit;
/// use project_uninit::partial_init;
///
/// #[derive(PartialEq, Eq, Debug)]
/// struct Person { name: Name, age: u32, id: (usize, usize) }
/// #[derive(PartialEq, Eq, Debug)]
/// struct Name { first: &'static str, last: &'static str }
///
/// let mut alice = MaybeUninit::<Person>::uninit();
///
/// let age = partial_init!(alice => age = 20);
/// assert_eq!(*age, 20);
/// *age = 22;
///
/// let (first, last, id) = partial_init!(alice => {
///     name => first: "Alice",
///     name => last: "Smith",
///     id: (123, 456),
/// });
/// assert_eq!(*first, "Alice");
/// assert_eq!(*last, "Smith");
/// assert_eq!(*id, (123, 456));
///
/// partial_init!(alice => id => 1 = 789);
///
/// assert_eq!(unsafe { alice.assume_init() }, Person {
///     name: Name { first: "Alice", last: "Smith" },
///     age: 22,
///     id: (123, 789),
/// });
/// ```
#[macro_export]
macro_rules! partial_init {
    // intialize multiple fields
    ($expr:expr => {$($($props:tt)=>+ : $val:expr),* $(,)?}) => {{
        // generate an error message if a field is used more than once
        $crate::__assert_unique!($expr, [ $( [ $($props).+ ] )* ]);
        #[allow(unused_imports)]
        use ::core::borrow::BorrowMut;
        let _ref: &mut ::core::mem::MaybeUninit<_> = $expr.borrow_mut();
        let ptr = _ref.as_mut_ptr();
        let lt = $crate::utils::bind_mut_lt(_ref);

        if false {
            // this will never be executed
            // it's only to assert that it is safe to access the fields
            #[allow(unused_unsafe)]
            let _x = unsafe { &mut *ptr };
            let _y = ($(&mut _x.$($props).+,)*);
        }
        ($({
            let prop_ref;
            #[allow(unused_unsafe)]
            unsafe {
                let prop_ptr = ::core::ptr::addr_of_mut!((*ptr).$($props).+);
                ::core::ptr::write(prop_ptr, $val);
                prop_ref = $crate::utils::deref_ptr_with_lt(prop_ptr, lt);
            }
            prop_ref
        },)*)
    }};

    // initialize a single field
    ($expr:expr => $($props:tt)=>+ = $val:expr) => {
        $crate::partial_init!($expr => { $($props)=>+: $val }).0
    };
}

///```compile_fail
/// use project_uninit::partial_init;
/// use core::mem::MaybeUninit;
/// struct Foo { a: i32, b: u32 }
/// let mut x = MaybeUninit::<Foo>::uninit();
/// let _ = partial_init!(x => { a: 1, b: 6, a: -1 });
///```
fn _test_multiple_per_macro_call_fails() {}

///```compile_fail
/// use project_uninit::{partial_init, project_uninit};
/// use core::mem::MaybeUninit;
/// struct Foo { a: i32, b: u32 }
/// let mut x = MaybeUninit::<Foo>::uninit();
/// let a = project_uninit!(x => a);
/// partial_init!(x => { b: 6 });
/// let a1 = a;
///```
fn _partial_init_with_existing_field_borrow_fails() {}

///```compile_fail
/// use project_uninit::{partial_init, project_uninit_mut};
/// use core::mem::MaybeUninit;
/// struct Foo { a: i32, b: u32 }
/// let mut x = MaybeUninit::<Foo>::uninit();
/// let a = project_uninit_mut!(x => a);
/// *a = MaybeUninit::new(1);
/// partial_init!(x => { b: 6 });
/// *a = MaybeUninit::new(3);
///```
fn _partial_init_with_existing_mut_field_borrow_fails() {}
