extern crate atomic_pincell;

#[cfg(feature = "serde")]
extern crate serde;

use atomic_pincell::{AtomicRef, AtomicRefCell, AtomicRefMut};

#[derive(Debug)]
struct Foo {
    u: u32,
}

#[derive(Debug)]
struct Bar {
    f: Foo,
}

impl Default for Bar {
    fn default() -> Self {
        Bar { f: Foo { u: 42 } }
    }
}

// FIXME(bholley): Add tests to exercise this in concurrent scenarios.

#[test]
fn immutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow();
    let _second = a.borrow();
}

#[test]
fn try_immutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.try_borrow().unwrap();
    let _second = a.try_borrow().unwrap();
}

#[test]
fn mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _ = a.borrow_mut();
}

#[test]
fn try_mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _ = a.try_borrow_mut().unwrap();
}

#[test]
fn get_mut() {
    let mut a = AtomicRefCell::new(Bar::default());
    let _ = a.get_mut();
}

#[test]
fn interleaved() {
    let a = AtomicRefCell::new(Bar::default());
    {
        let _ = a.borrow_mut();
    }
    {
        let _first = a.borrow();
        let _second = a.borrow();
    }
    {
        let _ = a.borrow_mut();
    }
}

#[test]
fn try_interleaved() {
    let a = AtomicRefCell::new(Bar::default());
    {
        let _ = a.try_borrow_mut().unwrap();
    }
    {
        let _first = a.try_borrow().unwrap();
        let _second = a.try_borrow().unwrap();
        let _ = a.try_borrow_mut().unwrap_err();
    }
    {
        let _first = a.try_borrow_mut().unwrap();
        let _ = a.try_borrow().unwrap_err();
    }
}

// For Miri to catch issues when calling a function.
//
// See how this scenerio affects std::cell::RefCell implementation:
// https://github.com/rust-lang/rust/issues/63787
//
// Also see relevant unsafe code guidelines issue:
// https://github.com/rust-lang/unsafe-code-guidelines/issues/125
#[test]
fn drop_and_borrow_in_fn_call() {
    fn drop_and_borrow(cell: &AtomicRefCell<Bar>, borrow: AtomicRef<'_, Bar>) {
        drop(borrow);
        *cell.borrow_mut() = Bar::default();
    }

    let a = AtomicRefCell::new(Bar::default());
    let borrow = a.borrow();
    drop_and_borrow(&a, borrow);
}

#[test]
#[should_panic(expected = "already immutably borrowed")]
fn immutable_then_mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow();
    let _second = a.borrow_mut();
}

#[test]
fn immutable_then_try_mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow();
    let _second = a.try_borrow_mut().unwrap_err();
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn mutable_then_immutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow_mut();
    let _second = a.borrow();
}

#[test]
fn mutable_then_try_immutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow_mut();
    let _second = a.try_borrow().unwrap_err();
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn double_mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow_mut();
    let _second = a.borrow_mut();
}

#[test]
fn mutable_then_try_mutable() {
    let a = AtomicRefCell::new(Bar::default());
    let _first = a.borrow_mut();
    let _second = a.try_borrow_mut().unwrap_err();
}

#[test]
fn map() {
    let a = AtomicRefCell::new(Bar::default());
    let b = a.borrow();
    assert_eq!(b.f.u, 42);
    let c = AtomicRef::map(b, |x| &x.f);
    assert_eq!(c.u, 42);
    let d = AtomicRef::map(c, |x| &x.u);
    assert_eq!(*d, 42);
}

#[test]
fn map_mut() {
    let a = AtomicRefCell::new(Bar::default());
    let mut b = a.borrow_mut();
    assert_eq!(b.f.u, 42);
    b.f.u = 43;
    let mut c = AtomicRefMut::map(b, |x| &mut x.f);
    assert_eq!(c.u, 43);
    c.u = 44;
    let mut d = AtomicRefMut::map(c, |x| &mut x.u);
    assert_eq!(*d, 44);
    *d = 45;
    assert_eq!(*d, 45);
}

#[test]
fn debug_fmt() {
    let a = AtomicRefCell::new(Foo { u: 42 });
    assert_eq!(format!("{:?}", a), "AtomicRefCell { value: Foo { u: 42 } }");
}

#[test]
fn debug_fmt_borrowed() {
    let a = AtomicRefCell::new(Foo { u: 42 });
    let _b = a.borrow();
    assert_eq!(format!("{:?}", a), "AtomicRefCell { value: Foo { u: 42 } }");
}

#[test]
fn debug_fmt_borrowed_mut() {
    let a = AtomicRefCell::new(Foo { u: 42 });
    let _b = a.borrow_mut();
    assert_eq!(format!("{:?}", a), "AtomicRefCell { value: <borrowed> }");
}

#[test]
#[cfg(feature = "serde")]
fn serde() {
    let value = 10;
    let cell = AtomicRefCell::new(value);

    let serialized = serde_json::to_string(&cell).unwrap();
    let deserialized = serde_json::from_str::<AtomicRefCell<usize>>(&serialized).unwrap();

    assert_eq!(*deserialized.borrow(), value);
}
