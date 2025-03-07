# atomic_pincell
This is a fork of [atomic_refcell](https://github.com/bholley/atomic_refcell) that adds the ability to pin cell contents like the non-thread-safe [pin-cell](https://github.com/dignifiedquire/pin-cell).

```rust
fn pin_cell() {
    let pinned_cell = pin!(AtomicRefCell::<MyPinnedType, true>::default());
    takes_pinned(pinned_cell.as_ref().borrow_pin_mut().get_pin_mut());
    takes_ref(&pinned_cell.borrow());
}
```

This crate also intends to be a drop in replacement for `atomic_refcell`.
```rust
fn unpin_cell() {
    let unpinned_cell = AtomicRefCell::<MyPinnedType>::default();
    mem::take(&mut *unpinned_cell.borrow_mut());
    takes_ref(&*unpinned_cell.borrow());
}
```