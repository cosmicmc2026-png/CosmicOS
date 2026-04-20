//! Schermata di avvio CosmicOS — branding, versione, motto.

use crate::gfx::{color::Color, renderer::Renderer};

pub fn render(r: &mut Renderer) {
    let w = r.width();
    let h = r.height();
    let cx = w / 2;
    let cy = h / 2;

    // ── Sfondo bianco puro ───────────────────────────────────────────────────
    r.clear(Color::WHITE);

    // ── Logo CosmicOS ────────────────────────────────────────────────────────
    // Cerchio rosso a sinistra del testo principale
    let logo_y      = cy.saturating_sub(70);
    let logo_radius = 22usize;
    let logo_cx     = cx.saturating_sub(210);
    r.fill_circle(logo_cx, logo_y + logo_radius, logo_radius, Color::COSMIC_RED);

    // Punto interno bianco (stile "stella" / pianeta)
    r.fill_circle(logo_cx + 6, logo_y + logo_radius - 6, 6, Color::WHITE);

    // ── "COSMIC" — grande, rosso ─────────────────────────────────────────────
    let title_scale = 5usize;
    let title       = "COSMIC";
    let title_w     = Renderer::string_width(title, title_scale);
    let title_x     = cx.saturating_sub(title_w / 2).saturating_sub(30);
    let title_y     = cy.saturating_sub(60);
    r.draw_string(title_x, title_y, title, Color::COSMIC_RED, title_scale);

    // ── "OS" — affiancato, scuro ─────────────────────────────────────────────
    let os_scale = 5usize;
    let os_x     = title_x + title_w + 8;
    let os_y     = title_y;
    r.draw_string(os_x, os_y, "OS", Color::COSMIC_TEXT, os_scale);

    // ── Linea sottile separatore ─────────────────────────────────────────────
    let line_w = 340usize;
    let line_x = cx.saturating_sub(line_w / 2);
    let line_y = cy + 10;
    r.fill_rect(line_x, line_y, line_w, 1, Color::COSMIC_BORDER);

    // ── Versione ─────────────────────────────────────────────────────────────
    let ver       = "Cosmic v0.0.1 Developer Beta";
    let ver_scale = 1usize;
    let ver_y     = line_y + 14;
    r.draw_string_centered(cx, ver_y, ver, Color::COSMIC_TEXT_SEC, ver_scale);

    // ── Motto ─────────────────────────────────────────────────────────────────
    let motto       = "Prestazioni spaziali";
    let motto_scale = 1usize;
    let motto_y     = ver_y + 18;
    r.draw_string_centered(cx, motto_y, motto, Color::COSMIC_RED, motto_scale);

    // ── Barra di caricamento ─────────────────────────────────────────────────
    let bar_w   = 200usize;
    let bar_h   = 3usize;
    let bar_x   = cx.saturating_sub(bar_w / 2);
    let bar_y   = h.saturating_sub(80);

    r.fill_rect(bar_x, bar_y, bar_w, bar_h, Color::COSMIC_BORDER);
    r.fill_rect(bar_x, bar_y, bar_w * 2 / 3, bar_h, Color::COSMIC_RED);

    // ── Copyright ────────────────────────────────────────────────────────────
    let copy_y = h.saturating_sub(40);
    r.draw_string_centered(cx, copy_y, "CosmicOS — Tutti i diritti riservati", Color::COSMIC_TEXT_SEC, 1);
}
