//! Renderer software su framebuffer lineare.
//! Fornisce primitive 2D: pixel, rettangoli, cerchi, testo scalato.

use super::{
    color::Color,
    font::{self, CHAR_H, CHAR_W},
};

pub struct Renderer {
    addr:   *mut u8,
    width:  usize,
    height: usize,
    pitch:  usize,   // byte per riga
    bpp:    usize,   // bit per pixel (di solito 32)
    rs:     u8,      // red   shift
    gs:     u8,      // green shift
    bs:     u8,      // blue  shift
}

// SAFETY: il framebuffer è memoria dedicata; accesso single-threaded in v0.0.1
unsafe impl Send for Renderer {}

impl Renderer {
    pub fn new(
        addr:   *mut u8,
        width:  usize,
        height: usize,
        pitch:  usize,
        bpp:    usize,
        rs: u8, gs: u8, bs: u8,
    ) -> Self {
        Self { addr, width, height, pitch, bpp, rs, gs, bs }
    }

    pub fn width(&self)  -> usize { self.width  }
    pub fn height(&self) -> usize { self.height }

    // ─── Primitiva base: singolo pixel ───────────────────────────────────────

    #[inline(always)]
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height { return; }
        let offset = y * self.pitch + x * (self.bpp / 8);
        let pixel  = color.to_u32(self.rs, self.gs, self.bs);
        unsafe {
            (self.addr.add(offset) as *mut u32).write_volatile(pixel);
        }
    }

    // ─── Rettangolo pieno ────────────────────────────────────────────────────

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        let pixel = color.to_u32(self.rs, self.gs, self.bs);
        let bytes = self.bpp / 8;

        for row in y..y_end {
            let row_base = row * self.pitch + x * bytes;
            for col in 0..(x_end - x) {
                unsafe {
                    (self.addr.add(row_base + col * bytes) as *mut u32)
                        .write_volatile(pixel);
                }
            }
        }
    }

    // ─── Rettangolo solo bordo ───────────────────────────────────────────────

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        self.fill_rect(x,         y,         w, 1, color); // top
        self.fill_rect(x,         y + h - 1, w, 1, color); // bottom
        self.fill_rect(x,         y,         1, h, color); // left
        self.fill_rect(x + w - 1, y,         1, h, color); // right
    }

    /// Rettangolo con bordo arrotondato (semplice: taglia angoli di r px)
    pub fn fill_rounded_rect(
        &mut self, x: usize, y: usize, w: usize, h: usize,
        r: usize, color: Color,
    ) {
        // Corpo centrale
        self.fill_rect(x + r, y,     w - 2 * r, h,         color);
        self.fill_rect(x,     y + r, r,          h - 2 * r, color);
        self.fill_rect(x + w - r, y + r, r,     h - 2 * r, color);

        // Angoli arrotondati con algoritmo del cerchio
        self.fill_quarter_circle(x + r,         y + r,         r, 2, color); // TL
        self.fill_quarter_circle(x + w - r - 1, y + r,         r, 1, color); // TR
        self.fill_quarter_circle(x + r,         y + h - r - 1, r, 3, color); // BL
        self.fill_quarter_circle(x + w - r - 1, y + h - r - 1, r, 0, color); // BR
    }

    fn fill_quarter_circle(
        &mut self, cx: usize, cy: usize, r: usize,
        quadrant: u8, color: Color,
    ) {
        let r2 = (r * r) as i64;
        for dy in 0..=(r as i64) {
            for dx in 0..=(r as i64) {
                if dx * dx + dy * dy <= r2 {
                    let (px, py) = match quadrant {
                        0 => (cx as i64 + dx, cy as i64 + dy), // BR
                        1 => (cx as i64 - dx, cy as i64 + dy), // BL (tr visuale)
                        2 => (cx as i64 + dx, cy as i64 - dy), // TR (tl visuale)
                        _ => (cx as i64 - dx, cy as i64 - dy), // TL
                    };
                    if px >= 0 && py >= 0 {
                        self.draw_pixel(px as usize, py as usize, color);
                    }
                }
            }
        }
    }

    // ─── Cerchio pieno ───────────────────────────────────────────────────────

    pub fn fill_circle(&mut self, cx: usize, cy: usize, r: usize, color: Color) {
        let r2 = (r * r) as i64;
        let ri = r as i64;
        let cxi = cx as i64;
        let cyi = cy as i64;
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                if dx * dx + dy * dy <= r2 {
                    let px = cxi + dx;
                    let py = cyi + dy;
                    if px >= 0 && py >= 0 {
                        self.draw_pixel(px as usize, py as usize, color);
                    }
                }
            }
        }
    }

    // ─── Linea orizzontale ───────────────────────────────────────────────────

    pub fn draw_hline(&mut self, x: usize, y: usize, len: usize, color: Color) {
        self.fill_rect(x, y, len, 1, color);
    }

    // ─── Pulisci lo schermo ──────────────────────────────────────────────────

    pub fn clear(&mut self, color: Color) {
        self.fill_rect(0, 0, self.width, self.height, color);
    }

    // ─── Testo bitmap scalato ────────────────────────────────────────────────

    /// Disegna un singolo carattere con scala 1:scale.
    pub fn draw_char(&mut self, x: usize, y: usize, c: char, color: Color, scale: usize) {
        let glyph = font::glyph(c);
        for row in 0..CHAR_H {
            for col in 0..CHAR_W {
                if glyph[row] & (0x80 >> col) != 0 {
                    self.fill_rect(
                        x + col * scale,
                        y + row * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
    }

    /// Disegna una stringa ASCII.
    pub fn draw_string(&mut self, x: usize, y: usize, s: &str, color: Color, scale: usize) {
        let stride = CHAR_W * scale + scale / 2 + 1;
        let mut cx = x;
        for c in s.chars() {
            self.draw_char(cx, y, c, color, scale);
            cx += stride;
        }
    }

    /// Larghezza in pixel di una stringa con dato scale.
    pub fn string_width(s: &str, scale: usize) -> usize {
        let stride = CHAR_W * scale + scale / 2 + 1;
        s.len() * stride
    }

    /// Disegna una stringa centrata orizzontalmente attorno a `cx`.
    pub fn draw_string_centered(
        &mut self, cx: usize, y: usize, s: &str, color: Color, scale: usize,
    ) {
        let w  = Self::string_width(s, scale);
        let x  = cx.saturating_sub(w / 2);
        self.draw_string(x, y, s, color, scale);
    }

    /// Testo con sfondo (per etichette, bottoni, input fields).
    pub fn draw_string_bg(
        &mut self, x: usize, y: usize, s: &str,
        color: Color, bg: Color, scale: usize, pad_x: usize, pad_y: usize,
    ) {
        let w = Self::string_width(s, scale);
        let h = CHAR_H * scale;
        self.fill_rect(
            x.saturating_sub(pad_x),
            y.saturating_sub(pad_y),
            w + pad_x * 2,
            h + pad_y * 2,
            bg,
        );
        self.draw_string(x, y, s, color, scale);
    }

    // ─── Gradient verticale semplice ─────────────────────────────────────────

    /// Sfumatura verticale da `top` a `bottom` su un rettangolo.
    pub fn fill_gradient_v(
        &mut self, x: usize, y: usize, w: usize, h: usize,
        top: Color, bottom: Color,
    ) {
        for row in 0..h {
            let t = ((row * 255) / h.max(1)) as u8;
            let c = Color::lerp(top, bottom, t);
            self.fill_rect(x, y + row, w, 1, c);
        }
    }

    // ─── Ombra sottile ───────────────────────────────────────────────────────

    /// Disegna una piccola ombra a destra/sotto di un rettangolo.
    pub fn draw_shadow(
        &mut self, x: usize, y: usize, w: usize, h: usize, offset: usize,
    ) {
        let shadow = Color::rgb(0xCC, 0xCC, 0xCC);
        self.fill_rect(x + offset, y + offset, w, h, shadow);
    }
}
