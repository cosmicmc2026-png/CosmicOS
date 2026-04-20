//! Pipeline UI CosmicOS.
//! State machine: BootScreen → Login → Desktop

pub mod boot_screen;
pub mod desktop;
pub mod login;
pub mod window;

use crate::input::keyboard;
use x86_64::instructions::interrupts;

// ─── Stati UI ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiState {
    Boot,
    Login,
    Desktop,
}

// ─── Durata boot screen (iterazioni spin loop) ────────────────────────────────
// ~2 s su hardware 3 GHz in QEMU; regola se necessario.
const BOOT_DURATION: u64 = 180_000_000;

// ─── Entry point UI ──────────────────────────────────────────────────────────

pub fn run() -> ! {
    let mut state        = UiState::Boot;
    let mut counter: u64 = 0;
    let mut needs_redraw = true;
    let mut login_ctx    = login::LoginCtx::new();
    let mut desktop_ctx  = desktop::DesktopCtx::new();

    loop {
        // ── Redraw ──────────────────────────────────────────────────────────
        if needs_redraw {
            let mut lock = crate::RENDERER.lock();
            if let Some(ref mut r) = *lock {
                match state {
                    UiState::Boot    => boot_screen::render(r),
                    UiState::Login   => login::render(r, &login_ctx),
                    UiState::Desktop => desktop::render(r, &desktop_ctx),
                }
            }
            needs_redraw = false;
        }

        // ── Avanzamento automatico boot screen ───────────────────────────────
        if state == UiState::Boot {
            counter += 1;
            if counter >= BOOT_DURATION {
                state        = UiState::Login;
                needs_redraw = true;
                counter      = 0;
            }
            core::hint::spin_loop();
            continue;
        }

        // ── Input ────────────────────────────────────────────────────────────
        let ev = interrupts::without_interrupts(|| keyboard::pop_event());

        if let Some(event) = ev {
            needs_redraw = true;
            match state {
                UiState::Login => {
                    if let Some(next) = login::handle_input(&mut login_ctx, event) {
                        state = match next {
                            login::LoginResult::Success => UiState::Desktop,
                        };
                        desktop_ctx = desktop::DesktopCtx::new();
                    }
                }
                UiState::Desktop => {
                    desktop::handle_input(&mut desktop_ctx, event);
                }
                UiState::Boot => {}
            }
        } else {
            // Nessun input - risparmio CPU con HLT
            x86_64::instructions::hlt();
        }
    }
}
