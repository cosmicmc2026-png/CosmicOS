//! CosmicOS - Cosmic v0.0.1 Developer Beta
//! Prestazioni spaziali
//!
//! Entry point del kernel. Riceve il controllo da Limine (UEFI),
//! inizializza tutti i sottosistemi e avvia la pipeline grafica.

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod gdt;
pub mod gfx;
pub mod input;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod ui;
pub mod vfs;

use limine::{
    request::{
        FramebufferRequest, HhdmRequest, MemoryMapRequest,
        RequestsEndMarker, RequestsStartMarker,
    },
    BaseRevision,
};
use spin::Mutex;

// ─── Limine request section markers ──────────────────────────────────────────

#[used]
#[link_section = ".requests_start_marker"]
static _REQ_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests_end_marker"]
static _REQ_END: RequestsEndMarker = RequestsEndMarker::new();

// ─── Global renderer ─────────────────────────────────────────────────────────

/// Renderer globale: inizializzato una volta in kmain, poi usato da tutto il kernel.
pub static RENDERER: Mutex<Option<gfx::renderer::Renderer>> = Mutex::new(None);

// ─── Kernel entry point ───────────────────────────────────────────────────────

#[no_mangle]
extern "C" fn kmain() -> ! {
    // 1. Verifica che Limine supporti la revisione richiesta
    assert!(BASE_REVISION.is_supported(), "Limine: revisione non supportata");

    // 2. Ottieni il framebuffer da Limine
    let fb_resp = FRAMEBUFFER_REQUEST
        .get_response()
        .expect("Limine: nessuna risposta framebuffer");

    let fb = fb_resp
        .framebuffers()
        .next()
        .expect("Limine: nessun framebuffer disponibile");

    // 3. Inizializza il renderer globale
    {
        let renderer = gfx::renderer::Renderer::new(
            fb.addr() as *mut u8,
            fb.width() as usize,
            fb.height() as usize,
            fb.pitch() as usize,
            fb.bpp() as usize,
            fb.red_mask_shift(),
            fb.green_mask_shift(),
            fb.blue_mask_shift(),
        );
        *RENDERER.lock() = Some(renderer);
    }

    // 4. Inizializza GDT (Global Descriptor Table)
    gdt::init();

    // 5. Inizializza IDT + PIC (interrupt controller)
    interrupts::init();

    // 6. Inizializza il memory allocator (heap Rust)
    let mm_resp   = MEMORY_MAP_REQUEST.get_response().expect("Limine: nessuna mappa memoria");
    let hhdm_resp = HHDM_REQUEST.get_response().expect("Limine: nessun HHDM");
    memory::init(mm_resp, hhdm_resp.offset());

    // 7. Inizializza il Virtual File System
    vfs::init();

    // 8. Abilita gli interrupt hardware
    x86_64::instructions::interrupts::enable();

    // 9. Avvia la pipeline UI (boot screen → login → desktop)
    ui::run()
}
