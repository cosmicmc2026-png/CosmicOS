//! Tipo colore RGB e palette ufficiale CosmicOS.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converte in valore a 32 bit per il framebuffer,
    /// rispettando le maschere di canale restituite da Limine.
    #[inline(always)]
    pub fn to_u32(self, rs: u8, gs: u8, bs: u8) -> u32 {
        ((self.r as u32) << rs) | ((self.g as u32) << gs) | ((self.b as u32) << bs)
    }

    /// Interpola linearmente tra due colori (t: 0–255).
    pub fn lerp(a: Color, b: Color, t: u8) -> Color {
        let t = t as u16;
        let inv = 255 - t;
        Color {
            r: ((a.r as u16 * inv + b.r as u16 * t) / 255) as u8,
            g: ((a.g as u16 * inv + b.g as u16 * t) / 255) as u8,
            b: ((a.b as u16 * inv + b.b as u16 * t) / 255) as u8,
        }
    }
}

// ─── Palette CosmicOS ────────────────────────────────────────────────────────

impl Color {
    /// Sfondo desktop / schermata principale
    pub const COSMIC_BG: Color          = Color::rgb(0xEB, 0xEB, 0xEB);
    /// Sfondo pannelli / finestre
    pub const COSMIC_SURFACE: Color     = Color::rgb(0xFF, 0xFF, 0xFF);
    /// Sfondo menu bar
    pub const COSMIC_MENUBAR: Color     = Color::rgb(0xFA, 0xFA, 0xFA);
    /// Accento rosso premium CosmicOS
    pub const COSMIC_RED: Color         = Color::rgb(0xC9, 0x40, 0x40);
    /// Variante rossa più chiara (hover/focus)
    pub const COSMIC_RED_LIGHT: Color   = Color::rgb(0xE0, 0x60, 0x60);
    /// Testo principale
    pub const COSMIC_TEXT: Color        = Color::rgb(0x1A, 0x1A, 0x1A);
    /// Testo secondario / placeholder
    pub const COSMIC_TEXT_SEC: Color    = Color::rgb(0x88, 0x88, 0x88);
    /// Bordi e separatori
    pub const COSMIC_BORDER: Color      = Color::rgb(0xDE, 0xDE, 0xDE);
    /// Grigio chiaro (sfondo input)
    pub const COSMIC_INPUT_BG: Color    = Color::rgb(0xF4, 0xF4, 0xF4);
    /// Bianco puro
    pub const WHITE: Color              = Color::rgb(0xFF, 0xFF, 0xFF);
    /// Nero puro
    pub const BLACK: Color              = Color::rgb(0x00, 0x00, 0x00);
    /// Traffic light: rosso (close)
    pub const TL_RED: Color             = Color::rgb(0xFF, 0x5F, 0x57);
    /// Traffic light: giallo (minimize)
    pub const TL_YELLOW: Color          = Color::rgb(0xFF, 0xBD, 0x2E);
    /// Traffic light: verde (maximize)
    pub const TL_GREEN: Color           = Color::rgb(0x28, 0xC8, 0x41);
}
