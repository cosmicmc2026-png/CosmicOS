//! Panic handler - schermata di errore kernel stile CosmicOS.

use core::panic::PanicInfo;
use crate::gfx::color::Color;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Disabilita interrupt per evitare race condition durante il panic
    x86_64::instructions::interrupts::disable();

    let mut lock = crate::RENDERER.lock();
    if let Some(ref mut r) = *lock {
        // Schermata di panico: sfondo rosso scuro
        r.clear(Color::rgb(0x8B, 0x00, 0x00));

        r.draw_string(40, 60,  "! KERNEL PANIC", Color::WHITE, 3);
        r.draw_string(40, 130, "CosmicOS ha riscontrato un errore critico.", Color::rgb(0xFF, 0xCC, 0xCC), 1);
        r.draw_string(40, 160, "Il sistema e' stato arrestato per sicurezza.", Color::rgb(0xFF, 0xCC, 0xCC), 1);

        // Mostra il messaggio di panico se disponibile
        if let Some(msg) = info.message().and_then(|m| m.as_str()) {
            r.draw_string(40, 210, "Errore:", Color::rgb(0xFF, 0xAA, 0xAA), 1);
            r.draw_string(40, 230, msg, Color::WHITE, 1);
        }

        // Mostra la posizione del panico
        if let Some(loc) = info.location() {
            r.draw_string(40, 270, "Posizione:", Color::rgb(0xFF, 0xAA, 0xAA), 1);

            // Stampa file e linea usando un buffer stack
            let mut buf = [0u8; 128];
            let file = loc.file().as_bytes();
            let len = file.len().min(100);
            buf[..len].copy_from_slice(&file[..len]);
            if let Ok(s) = core::str::from_utf8(&buf[..len]) {
                r.draw_string(40, 290, s, Color::WHITE, 1);
            }
        }

        r.draw_string(40, 360, "Riavvia il sistema per continuare.", Color::rgb(0xFF, 0xCC, 0xCC), 1);
        r.draw_string(40, 400, "Cosmic v0.0.1 Developer Beta", Color::rgb(0xFF, 0x88, 0x88), 1);
    }

    loop {
        x86_64::instructions::hlt();
    }
}
