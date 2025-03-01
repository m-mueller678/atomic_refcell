use atomic_pincell::AtomicRefCell;
use std::marker::PhantomPinned;
use std::mem;
use std::pin::{pin, Pin};

#[derive(Default)]
struct MyPinnedType(u64, PhantomPinned);

fn takes_pinned(_x: Pin<&mut MyPinnedType>) {}

fn takes_ref(x: &MyPinnedType) {
    println!("{}", x.0);
}

fn pin_cell() {
    let pinned_cell = pin!(AtomicRefCell::<MyPinnedType, true>::default());
    takes_pinned(pinned_cell.as_ref().borrow_pin_mut().get_pin_mut());
    takes_ref(&pinned_cell.borrow());
}

fn unpin_cell() {
    let unpinned_cell = AtomicRefCell::<MyPinnedType>::default();
    mem::take(&mut *unpinned_cell.borrow_mut());
    takes_ref(&*unpinned_cell.borrow());
}

fn main() {
    unpin_cell();
    pin_cell();
}
