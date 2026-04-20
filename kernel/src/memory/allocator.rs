//! Allocatore globale Rust — utilizza linked_list_allocator.

use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::empty();
