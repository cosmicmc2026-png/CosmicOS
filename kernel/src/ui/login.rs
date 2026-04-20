//! Schermata di login CosmicOS.
//! Credenziali default: utente = "admin", password = "cosmic"

use crate::{
    gfx::{color::Color, renderer::Renderer},
    input::InputEvent,
};

// ─── Credenziali utente default ───────────────────────────────────────────────

const ADMIN_USER: &str = "admin";
const ADMIN_PASS: &str = "cosmic";

// ─── Stato del login ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveField { Username, Password }

pub struct LoginCtx {
    pub username:     [u8; 64],
    pub username_len: usize,
    pub password:     [u8; 64],
    pub password_len: usize,
    pub active:       ActiveField,
    pub error:        bool,
    pub blink_count:  u64,
}

impl LoginCtx {
    pub fn new() -> Self {
        Self {
            username:     [0; 64],
            username_len: 0,
            password:     [0; 64],
            password_len: 0,
            active:       ActiveField::Username,
            error:        false,
            blink_count:  0,
        }
    }

    fn username_str(&self) -> &str {
        core::str::from_utf8(&self.username[..self.username_len]).unwrap_or("")
    }

    fn password_str(&self) -> &str {
        core::str::from_utf8(&self.password[..self.password_len]).unwrap_or("")
    }
}

// ─── Risultato login ──────────────────────────────────────────────────────────

pub enum LoginResult {
    Success,
}

// ─── Handler input ────────────────────────────────────────────────────────────

pub fn handle_input(ctx: &mut LoginCtx, ev: InputEvent) -> Option<LoginResult> {
    ctx.error = false;

    match ev {
        InputEvent::Tab => {
            ctx.active = match ctx.active {
                ActiveField::Username => ActiveField::Password,
                ActiveField::Password => ActiveField::Username,
            };
        }
        InputEvent::Enter => {
            let user = ctx.username_str();
            let pass = ctx.password_str();
            if user == ADMIN_USER && pass == ADMIN_PASS {
                return Some(LoginResult::Success);
            } else {
                ctx.error        = true;
                ctx.password_len = 0;
                ctx.password     = [0; 64];
            }
        }
        InputEvent::Backspace => {
            match ctx.active {
                ActiveField::Username if ctx.username_len > 0 => {
                    ctx.username_len -= 1;
                    ctx.username[ctx.username_len] = 0;
                }
                ActiveField::Password if ctx.password_len > 0 => {
                    ctx.password_len -= 1;
                    ctx.password[ctx.password_len] = 0;
                }
                _ => {}
            }
        }
        InputEvent::Char(c) => {
            let byte = c as u8;
            match ctx.active {
                ActiveField::Username if ctx.username_len < 63 => {
                    ctx.username[ctx.username_len] = byte;
                    ctx.username_len += 1;
                }
                ActiveField::Password if ctx.password_len < 63 => {
                    ctx.password[ctx.password_len] = byte;
                    ctx.password_len += 1;
                }
                _ => {}
            }
        }
        _ => {}
    }
    None
}

// ─── Rendering ────────────────────────────────────────────────────────────────

pub fn render(r: &mut Renderer, ctx: &LoginCtx) {
    let w = r.width();
    let h = r.height();
    let cx = w / 2;
    let cy = h / 2;

    // ── Sfondo desktop leggero ───────────────────────────────────────────────
    r.clear(Color::COSMIC_BG);

    // ── Pannello login (ombra + pannello bianco) ─────────────────────────────
    let panel_w = 380usize;
    let panel_h = 420usize;
    let panel_x = cx.saturating_sub(panel_w / 2);
    let panel_y = cy.saturating_sub(panel_h / 2);

    // Ombra
    r.draw_shadow(panel_x, panel_y, panel_w, panel_h, 6);

    // Pannello bianco
    r.fill_rounded_rect(panel_x, panel_y, panel_w, panel_h, 12, Color::COSMIC_SURFACE);
    r.draw_rect(panel_x, panel_y, panel_w, panel_h, Color::COSMIC_BORDER);

    // ── Logo nel pannello ────────────────────────────────────────────────────
    let logo_y = panel_y + 40;
    r.fill_circle(cx.saturating_sub(4), logo_y + 12, 12, Color::COSMIC_RED);
    r.fill_circle(cx + 4,              logo_y + 12, 12, Color::COSMIC_RED);
    r.fill_circle(cx,                  logo_y + 12, 12, Color::COSMIC_RED);
    // Effetto "C" sovrapposto
    r.fill_circle(cx.saturating_sub(4), logo_y + 12, 12, Color::COSMIC_RED);
    r.fill_rounded_rect(cx - 18, logo_y, 36, 24, 12, Color::COSMIC_RED);

    let brand_y = logo_y + 34;
    r.draw_string_centered(cx, brand_y, "CosmicOS", Color::COSMIC_TEXT, 2);

    let sub_y = brand_y + 26;
    r.draw_string_centered(cx, sub_y, "v0.0.1 Developer Beta", Color::COSMIC_TEXT_SEC, 1);

    // ── Separatore ───────────────────────────────────────────────────────────
    let sep_y = sub_y + 24;
    r.fill_rect(panel_x + 30, sep_y, panel_w - 60, 1, Color::COSMIC_BORDER);

    // ── Label e campo Username ───────────────────────────────────────────────
    let field_x  = panel_x + 30;
    let field_w  = panel_w - 60;
    let field_h  = 36usize;
    let label_h  = 14usize;

    let user_label_y = sep_y + 18;
    r.draw_string(field_x, user_label_y, "Nome utente", Color::COSMIC_TEXT_SEC, 1);

    let user_field_y = user_label_y + label_h + 4;
    draw_input_field(
        r, field_x, user_field_y, field_w, field_h,
        ctx.username_str(),
        false,
        ctx.active == ActiveField::Username,
    );

    // ── Label e campo Password ───────────────────────────────────────────────
    let pass_label_y = user_field_y + field_h + 18;
    r.draw_string(field_x, pass_label_y, "Password", Color::COSMIC_TEXT_SEC, 1);

    let pass_field_y = pass_label_y + label_h + 4;
    let pass_display = mask_password(ctx.password_len);
    draw_input_field(
        r, field_x, pass_field_y, field_w, field_h,
        &pass_display,
        false,
        ctx.active == ActiveField::Password,
    );

    // ── Messaggio di errore ───────────────────────────────────────────────────
    if ctx.error {
        let err_y = pass_field_y + field_h + 10;
        r.draw_string_centered(
            cx, err_y,
            "Credenziali non valide. Riprova.",
            Color::COSMIC_RED, 1,
        );
    }

    // ── Pulsante Accedi ──────────────────────────────────────────────────────
    let btn_y = pass_field_y + field_h + (if ctx.error { 36 } else { 24 });
    let btn_w = field_w;
    let btn_h = 40usize;
    r.fill_rounded_rect(field_x, btn_y, btn_w, btn_h, 8, Color::COSMIC_RED);
    r.draw_string_centered(cx, btn_y + 14, "Accedi", Color::WHITE, 2);

    // ── Hint Tab ──────────────────────────────────────────────────────────────
    let hint_y = panel_y + panel_h + 16;
    r.draw_string_centered(cx, hint_y, "Tab per cambiare campo  |  Invio per accedere", Color::COSMIC_TEXT_SEC, 1);
}

// ─── Utility ─────────────────────────────────────────────────────────────────

fn draw_input_field(
    r: &mut Renderer,
    x: usize, y: usize, w: usize, h: usize,
    text: &str,
    _is_password: bool,
    focused: bool,
) {
    let bg     = Color::COSMIC_INPUT_BG;
    let border = if focused { Color::COSMIC_RED } else { Color::COSMIC_BORDER };

    r.fill_rounded_rect(x, y, w, h, 6, bg);
    r.draw_rect(x, y, w, h, border);

    if focused {
        // doppio bordo per focus
        r.draw_rect(x + 1, y + 1, w - 2, h - 2, Color::COSMIC_RED_LIGHT);
    }

    let text_y = y + (h.saturating_sub(10)) / 2;
    let text_x = x + 10;

    if text.is_empty() {
        // Placeholder
    } else {
        // Tronca il testo se troppo lungo
        let max_chars = (w.saturating_sub(20)) / 9; // ~9px per char a scale=1
        let display: &str = if text.len() > max_chars {
            &text[text.len().saturating_sub(max_chars)..]
        } else {
            text
        };
        r.draw_string(text_x, text_y, display, Color::COSMIC_TEXT, 1);
    }

    // Cursore lampeggiante (sempre visibile in questa versione)
    if focused {
        let cur_x = text_x + Renderer::string_width(text, 1);
        let cur_y = y + 6;
        r.fill_rect(cur_x.min(x + w - 16), cur_y, 2, h - 12, Color::COSMIC_RED);
    }
}

fn mask_password(len: usize) -> alloc::string::String {
    let mut s = alloc::string::String::new();
    for _ in 0..len {
        s.push('*');
    }
    s
}
