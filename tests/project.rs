use core::mem::MaybeUninit;

use project_uninit::{project_ptr, project_ptr_mut, project_uninit, project_uninit_mut};

#[derive(Debug, PartialEq, Eq)]
struct Foo {
    a: usize,
    b: (i32, (u8, i8), &'static str),
}

#[test]
fn project_uninit_single() {
    let mut x = MaybeUninit::new(Foo {
        a: 12,
        b: (123, (45, 67), "goodbye"),
    });

    let a = project_uninit_mut!(x => a);
    assert_eq!(unsafe { a.assume_init() }, 12);
    *a = MaybeUninit::new(13);

    let a = project_uninit!(x => a);
    assert_eq!(unsafe { a.assume_init() }, 13);
}

#[test]
fn project_uninit() {
    let x = MaybeUninit::new(Foo {
        a: 12,
        b: (123, (45, 67), "goodbye"),
    });

    let (a, b, b1, b11, b2) = project_uninit!(x => {
        a,
        b,
        b => 1,
        b => 1 => 1,
        b => 2,
    });

    unsafe {
        assert_eq!(a.assume_init(), 12);
        assert_eq!(b.assume_init(), (123, (45, 67), "goodbye"));
        assert_eq!(b1.assume_init(), (45, 67));
        assert_eq!(b11.assume_init(), 67);
        assert_eq!(b2.assume_init(), "goodbye");
    }
}

#[test]
fn project_uninit_mut() {
    let mut x = MaybeUninit::<Foo>::uninit();

    let (a, b1, b2) = project_uninit_mut!(x => {
        a,
        b => 1,
        b => 2,
    });

    *a = MaybeUninit::new(7);
    *b1 = MaybeUninit::new((17, 18));
    *b2 = MaybeUninit::new("hello");

    let (aa, b0, b11) = project_uninit_mut!(x => {
        a,
        b => 0,
        b => 1 => 1,
    });

    assert_eq!(unsafe { aa.assume_init() }, 7);
    *aa = MaybeUninit::new(12);
    *b0 = MaybeUninit::new(-100);
    *b11 = MaybeUninit::new(19);

    assert_eq!(
        unsafe { x.assume_init() },
        Foo {
            a: 12,
            b: (-100, (17, 19), "hello"),
        }
    );
}

#[test]
fn project_ptr() {
    let x = Foo {
        a: 0,
        b: (1, (2, 3), "foo"),
    };

    unsafe {
        let (a, b, b11, b2) = project_ptr!(&x => {
            a,
            b,
            b => 1 => 1,
            b => 2,
        });

        assert_eq!(*a, 0);
        assert_eq!(*b, (1, (2, 3), "foo"));
        assert_eq!(*b11, 3);
        assert_eq!(*b2, "foo");
    }
}

#[test]
fn project_ptr_mut() {
    let mut x = Foo {
        a: 0,
        b: (1, (2, 3), "foo"),
    };

    unsafe {
        let (a, b11, b2) = project_ptr_mut!(&mut x => {
            a,
            b => 1 => 1,
            b => 2,
        });

        assert_eq!(*a, 0);
        assert_eq!(*b11, 3);
        assert_eq!(*b2, "foo");

        *a = 8;
        *b11 = 10;
        *b2 = "bar";

        assert_eq!(
            x,
            Foo {
                a: 8,
                b: (1, (2, 10), "bar"),
            }
        );
    }
}

#[test]
fn project_ptr_single() {
    let mut x = Foo {
        a: 0,
        b: (1, (2, 3), "foo"),
    };

    unsafe {
        let a = project_ptr_mut!(&mut x => a);
        assert_eq!(*a, 0);
        *a = 100;
        let b11 = project_ptr_mut!(&mut x => b => 1 => 1);
        assert_eq!(*b11, 3);
        *b11 = 22;

        let a = project_ptr!(&x => a);
        assert_eq!(*a, 100);
        let b11 = project_ptr!(&x => b => 1 => 1);
        assert_eq!(*b11, 22);

        assert_eq!(
            x,
            Foo {
                a: 100,
                b: (1, (2, 22), "foo"),
            }
        );
    }
}

#[test]
fn escaping_reference() {
    let mut x = MaybeUninit::new((1, 2));

    fn inner<'a>(
        x: &'a mut MaybeUninit<(i32, u32)>,
    ) -> (&'a mut MaybeUninit<i32>, &'a mut MaybeUninit<u32>) {
        project_uninit_mut!(x => { 0, 1 })
    }

    let (a, b) = inner(&mut x);
    *a = MaybeUninit::new(100);
    *b = MaybeUninit::new(200);

    assert_eq!(unsafe { x.assume_init() }, (100, 200));
}
