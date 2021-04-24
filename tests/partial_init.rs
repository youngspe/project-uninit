use core::mem::MaybeUninit;

use project_uninit::partial_init;

#[derive(Debug, PartialEq, Eq)]
struct Foo {
    a: usize,
    b: (i32, (u8, i8), &'static str),
}

#[test]
fn partial_init_single() {
    let mut x = MaybeUninit::new(Foo {
        a: 1,
        b: (2, (3, 4), "hey"),
    });

    let a = partial_init!(x => a = 10);
    assert_eq!(*a, 10);
    let b10 = partial_init!(x => b => 1 => 0 = 8);
    assert_eq!(*b10, 8);
    assert_eq!(
        unsafe { x.assume_init() },
        Foo {
            a: 10,
            b: (2, (8, 4), "hey"),
        }
    );
}

#[test]
fn partial_init_and_mutate_field() {
    let mut x = MaybeUninit::<Foo>::uninit();

    let (a, b10, b2) = partial_init!(x => {
        a: 7,
        b => 1 => 0: 240,
        b => 2: "hello",
    });

    assert_eq!((*a, *b10, *b2), (7, 240, "hello"));

    *a = 12;

    let (b0, b1) = partial_init!(x => {
        b => 0: -10,
        b => 1: (17, -29),
    });

    assert_eq!((*b0, *b1), (-10, (17, -29)));

    assert_eq!(
        unsafe { x.assume_init() },
        Foo {
            a: 12,
            b: (-10, (17, -29), "hello"),
        }
    );
}

#[test]
fn init_escaping_reference() {
    let mut x = MaybeUninit::uninit();

    fn inner<'a>(x: &'a mut MaybeUninit<(i32, u32)>) -> (&'a mut i32, &'a mut u32) {
        partial_init!(x => { 0: 10, 1: 20 })
    }

    let (a, b) = inner(&mut x);
    assert_eq!((*a, *b), (10, 20));

    *a = 100;
    assert_eq!(unsafe { x.assume_init() }, (100, 20));
}
