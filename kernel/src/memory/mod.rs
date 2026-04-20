//! Sottosistema memoria - inizializza l'heap Rust dal memory map di Limine.

pub mod allocator;

use limine::{memory_map::EntryType, response::MemoryMapResponse};

pub fn init(memory_map: &MemoryMapResponse, hhdm_offset: u64) {
    // Cerca la prima regione USABLE >= 8 MiB per il nostro heap
    for entry in memory_map.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }
        if entry.length < 8 * 1024 * 1024 {
            continue;
        }

        // Indirizzo virtuale = fisico + offset HHDM
        let heap_start = (entry.base + hhdm_offset) as usize;
        // Usiamo al massimo 32 MiB per l'heap v0.0.1
        let heap_size  = (entry.length as usize).min(32 * 1024 * 1024);

        unsafe {
            allocator::HEAP.lock().init(heap_start as *mut u8, heap_size);
        }

        return;
    }

    panic!("Impossibile trovare memoria usabile per l'heap del kernel");
}
