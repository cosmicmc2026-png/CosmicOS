//! Sistema finestre CosmicOS — v0.0.1
//! Finestre con chrome stile macOS: traffic lights, titolo, contenuto.

use crate::gfx::{color::Color, renderer::Renderer};

/// Descrive una finestra sullo schermo.
pub struct Window {
    pub x:      usize,
    pub y:      usize,
    pub width:  usize,
    pub height: usize,
    pub title:  &'static str,
    pub open:   bool,
}

pub const TITLEBAR_H: usize = 36;
pub const TL_RADIUS:  usize = 6;
pub const TL_SPACING: usize = 18;
pub const TL_MARGIN:  usize = 14;

impl Window {
    pub const fn new(
        x: usize, y: usize, width: usize, height: usize, title: &'static str,
    ) -> Self {
        Self { x, y, width, height, title, open: true }
    }

    /// Disegna la finestra completa sul renderer.
    pub fn draw(&self, r: &mut Renderer) {
        if !self.open { return; }

        let total_h = self.height + TITLEBAR_H;

        // ── Ombra ────────────────────────────────────────────────────────────
        r.draw_shadow(self.x, self.y, self.width, total_h, 8);

        // ── Corpo finestra ────────────────────────────────────────────────────
        r.fill_rounded_rect(self.x, self.y, self.width, total_h, 10, Color::COSMIC_SURFACE);

        // ── Title bar ────────────────────────────────────────────────────────
        // Sfondo title bar (bianco, leggermente diverso dal corpo)
        r.fill_rect(
            self.x + 1,
            self.y + 1,
            self.width - 2,
            TITLEBAR_H - 1,
            Color::COSMIC_MENUBAR,
        );

        // Separatore titolo/contenuto
        r.fill_rect(
            self.x,
            self.y + TITLEBAR_H,
            self.width,
            1,
            Color::COSMIC_BORDER,
        );

        // ── Traffic lights ────────────────────────────────────────────────────
        let tly = self.y + TITLEBAR_H / 2;

        // Close — rosso
        r.fill_circle(self.x + TL_MARGIN, tly, TL_RADIUS, Color::TL_RED);
        // Minimize — giallo
        r.fill_circle(self.x + TL_MARGIN + TL_SPACING, tly, TL_RADIUS, Color::TL_YELLOW);
        // Maximize — verde
        r.fill_circle(self.x + TL_MARGIN + TL_SPACING * 2, tly, TL_RADIUS, Color::TL_GREEN);

        // ── Titolo centrato ───────────────────────────────────────────────────
        let title_cx = self.x + self.width / 2;
        let title_y  = self.y + (TITLEBAR_H / 2).saturating_sub(4);
        r.draw_string_centered(title_cx, title_y, self.title, Color::COSMIC_TEXT, 1);

        // ── Bordo esterno arrotondato ────────────────────────────────────────
        r.draw_rect(self.x, self.y, self.width, total_h, Color::COSMIC_BORDER);
    }

    /// Restituisce l'area contenuto (esclusa la title bar).
    pub fn content_rect(&self) -> (usize, usize, usize, usize) {
        (
            self.x,
            self.y + TITLEBAR_H,
            self.width,
            self.height,
        )
    }
}
