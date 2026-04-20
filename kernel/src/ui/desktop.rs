//! Desktop CosmicOS - menu bar, scrivania, dock, finestre di sistema.

use crate::{
    gfx::{color::Color, renderer::Renderer},
    input::InputEvent,
};
use super::window::Window;

// ─── Contesto desktop ────────────────────────────────────────────────────────

pub struct DesktopCtx {
    pub show_info_win:  bool,
    pub show_files_win: bool,
    pub show_settings_win: bool,
}

impl DesktopCtx {
    pub fn new() -> Self {
        Self {
            show_info_win:     true,  // Finestra di benvenuto aperta all'avvio
            show_files_win:    false,
            show_settings_win: false,
        }
    }
}

// ─── Input desktop ────────────────────────────────────────────────────────────

pub fn handle_input(ctx: &mut DesktopCtx, ev: InputEvent) {
    match ev {
        InputEvent::Char('i') | InputEvent::Char('I') => {
            ctx.show_info_win = !ctx.show_info_win;
        }
        InputEvent::Char('f') | InputEvent::Char('F') => {
            ctx.show_files_win = !ctx.show_files_win;
        }
        InputEvent::Char('s') | InputEvent::Char('S') => {
            ctx.show_settings_win = !ctx.show_settings_win;
        }
        _ => {}
    }
}

// ─── Rendering ────────────────────────────────────────────────────────────────

pub fn render(r: &mut Renderer, ctx: &DesktopCtx) {
    let w = r.width();
    let h = r.height();

    // ── Sfondo desktop ───────────────────────────────────────────────────────
    r.clear(Color::COSMIC_BG);

    // ── Menu bar ─────────────────────────────────────────────────────────────
    draw_menubar(r, w);

    // ── Desktop area ─────────────────────────────────────────────────────────
    draw_desktop_icons(r, w, h);

    // ── Dock ─────────────────────────────────────────────────────────────────
    draw_dock(r, w, h);

    // ── Finestre ─────────────────────────────────────────────────────────────
    if ctx.show_info_win {
        draw_info_window(r, w, h);
    }
    if ctx.show_files_win {
        draw_files_window(r, w, h);
    }
    if ctx.show_settings_win {
        draw_settings_window(r, w, h);
    }

    // ── Hint tasti ───────────────────────────────────────────────────────────
    let hint_y = h.saturating_sub(24);
    r.draw_string(
        10, hint_y,
        "I=Info  F=File  S=Impostazioni",
        Color::COSMIC_TEXT_SEC, 1,
    );
}

// ─── Menu bar ────────────────────────────────────────────────────────────────

const MENUBAR_H: usize = 32;

fn draw_menubar(r: &mut Renderer, w: usize) {
    // Sfondo bianco semi-opaco (simulato con bianco puro)
    r.fill_rect(0, 0, w, MENUBAR_H, Color::COSMIC_MENUBAR);
    r.fill_rect(0, MENUBAR_H, w, 1, Color::COSMIC_BORDER);

    // Logo CosmicOS (cerchietto rosso + "CosmicOS")
    r.fill_circle(14, MENUBAR_H / 2, 7, Color::COSMIC_RED);
    r.draw_string(26, 10, "CosmicOS", Color::COSMIC_TEXT, 1);

    // Voci menu
    let menus = ["File", "Visualizza", "Applicazioni", "Finestra"];
    let mut mx = 130usize;
    for menu in &menus {
        r.draw_string(mx, 10, menu, Color::COSMIC_TEXT, 1);
        mx += Renderer::string_width(menu, 1) + 20;
    }

    // Lato destro: ora statica + indicatori
    let time_str = "10:00";
    let time_w   = Renderer::string_width(time_str, 1);
    r.draw_string(w.saturating_sub(time_w + 14), 10, time_str, Color::COSMIC_TEXT, 1);

    // Icona Wi-Fi placeholder
    r.draw_string(w.saturating_sub(time_w + 60), 10, "WiFi", Color::COSMIC_TEXT_SEC, 1);
}

// ─── Desktop icons ────────────────────────────────────────────────────────────

fn draw_desktop_icons(r: &mut Renderer, w: usize, _h: usize) {
    // Icona "Benvenuto.costxt" - angolo in alto a destra
    let icon_x = w.saturating_sub(80);
    let icon_y = MENUBAR_H + 20;
    draw_file_icon(r, icon_x, icon_y, "Benvenuto", ".costxt", Color::rgb(0x4A, 0x90, 0xD9));
}

fn draw_file_icon(r: &mut Renderer, x: usize, y: usize, name: &str, _ext: &str, color: Color) {
    let iw = 48usize;
    let ih = 52usize;

    // Corpo icona
    r.fill_rounded_rect(x, y, iw, ih, 6, Color::WHITE);
    r.draw_rect(x, y, iw, ih, Color::COSMIC_BORDER);

    // Angolo piegato (top-right)
    r.fill_rect(x + iw - 14, y, 14, 14, Color::COSMIC_BG);
    r.draw_rect(x + iw - 14, y, 14, 14, Color::COSMIC_BORDER);

    // Colore tipo file
    r.fill_rect(x + 6, y + 18, iw - 12, 4, color);
    r.fill_rect(x + 6, y + 26, iw - 12, 2, Color::COSMIC_BORDER);
    r.fill_rect(x + 6, y + 32, iw - 12, 2, Color::COSMIC_BORDER);

    // Etichetta (nome sotto icona)
    let lbl_x = x.saturating_sub(4);
    r.draw_string(lbl_x, y + ih + 4, name, Color::COSMIC_TEXT, 1);
}

// ─── Dock ────────────────────────────────────────────────────────────────────

const DOCK_H:    usize = 60;
const DOCK_ICON: usize = 44;
const DOCK_PAD:  usize = 12;

fn draw_dock(r: &mut Renderer, w: usize, h: usize) {
    let apps = ["File", "Info", "Impost.", "Term."];
    let dock_w   = apps.len() * (DOCK_ICON + DOCK_PAD) + DOCK_PAD;
    let dock_x   = w / 2 - dock_w / 2;
    let dock_y   = h.saturating_sub(DOCK_H + 12);

    // Sfondo dock (bianco, arrotondato, con bordo)
    r.draw_shadow(dock_x, dock_y, dock_w, DOCK_H, 4);
    r.fill_rounded_rect(dock_x, dock_y, dock_w, DOCK_H, 14, Color::WHITE);
    r.draw_rect(dock_x, dock_y, dock_w, DOCK_H, Color::COSMIC_BORDER);

    // Icone dock
    let colors = [
        Color::rgb(0x4A, 0x90, 0xD9), // File - blu
        Color::rgb(0x7E, 0x57, 0xC2), // Info - viola
        Color::rgb(0x26, 0xA6, 0x9A), // Impostazioni - teal
        Color::rgb(0x42, 0x42, 0x42), // Terminal - grigio scuro
    ];

    let mut ix = dock_x + DOCK_PAD;
    let iy = dock_y + (DOCK_H - DOCK_ICON) / 2;

    for (i, app) in apps.iter().enumerate() {
        // Icona app (cerchio colorato con lettera)
        r.fill_rounded_rect(ix, iy, DOCK_ICON, DOCK_ICON, 10, colors[i]);
        let label_x = ix + DOCK_ICON / 2 - Renderer::string_width(&app[..1], 2) / 2;
        r.draw_string(label_x, iy + DOCK_ICON / 2 - 8, &app[..1], Color::WHITE, 2);

        ix += DOCK_ICON + DOCK_PAD;
    }
}

// ─── Finestra Info di sistema ────────────────────────────────────────────────

fn draw_info_window(r: &mut Renderer, w: usize, h: usize) {
    let win = Window::new(
        w / 2 - 220, h / 4,
        440, 280,
        "Informazioni su CosmicOS",
    );
    win.draw(r);

    let (cx, cy, _, _) = win.content_rect();
    let tx = cx + 20;
    let mut ty = cy + 20;

    let rows: &[(&str, &str)] = &[
        ("Sistema:",     "CosmicOS"),
        ("Versione:",    "Cosmic v0.0.1 Developer Beta"),
        ("Architettura:","x86_64"),
        ("Boot:",        "UEFI via Limine"),
        ("Kernel:",      "Rust (no_std) - bare metal"),
        ("Grafica:",     "Framebuffer lineare software"),
        ("Memoria:",     "Heap dinamico (linked_list_allocator)"),
        ("Motto:",       "Prestazioni spaziali"),
    ];

    for (label, value) in rows {
        r.draw_string(tx, ty, label, Color::COSMIC_TEXT_SEC, 1);
        r.draw_string(tx + 120, ty, value, Color::COSMIC_TEXT, 1);
        ty += 18;
    }

    // Linea decorativa rossa in fondo
    r.fill_rect(cx + 20, cy + 258, 400, 2, Color::COSMIC_RED);
}

// ─── Finestra File Manager ────────────────────────────────────────────────────

fn draw_files_window(r: &mut Renderer, w: usize, h: usize) {
    let win = Window::new(
        w / 2 - 250, h / 4 + 60,
        500, 320,
        "Gestione file",
    );
    win.draw(r);

    let (cx, cy, _, _) = win.content_rect();

    // Sidebar VFS
    r.fill_rect(cx, cy, 140, 320, Color::COSMIC_INPUT_BG);
    r.fill_rect(cx + 140, cy, 1, 320, Color::COSMIC_BORDER);

    let sidebar_items = ["Desktop", "Download", "Documenti", "Immagini", "Sistema", "Cestino"];
    let mut sy = cy + 12;
    for item in &sidebar_items {
        r.draw_string(cx + 12, sy, item, Color::COSMIC_TEXT, 1);
        sy += 20;
    }

    // Area principale (contenuto Desktop)
    let main_x = cx + 152;
    let mut my  = cy + 16;

    r.draw_string(main_x, my, "Desktop", Color::COSMIC_TEXT_SEC, 1);
    my += 24;
    r.fill_rect(main_x, my, 340, 1, Color::COSMIC_BORDER);
    my += 10;

    // File
    let files: &[(&str, &str)] = &[
        ("Benvenuto.costxt", "23 B"),
    ];
    for (name, size) in files {
        r.fill_circle(main_x + 6, my + 5, 5, Color::rgb(0x4A, 0x90, 0xD9));
        r.draw_string(main_x + 16, my, name, Color::COSMIC_TEXT, 1);
        r.draw_string(main_x + 300, my, size, Color::COSMIC_TEXT_SEC, 1);
        my += 22;
    }
}

// ─── Finestra Impostazioni ────────────────────────────────────────────────────

fn draw_settings_window(r: &mut Renderer, w: usize, h: usize) {
    let win = Window::new(
        w / 2 + 40, h / 4,
        400, 300,
        "Impostazioni",
    );
    win.draw(r);

    let (cx, cy, _, _) = win.content_rect();
    let tx = cx + 20;
    let mut ty = cy + 20;

    r.draw_string(tx, ty, "Aspetto", Color::COSMIC_TEXT, 2);
    ty += 30;

    let settings: &[(&str, &str)] = &[
        ("Tema:",       "Chiaro (Light)"),
        ("Accento:",    "Rosso CosmicOS"),
        ("Font:",       "Bitmap 8x8"),
        ("Lingua:",     "Italiano"),
    ];

    for (k, v) in settings {
        r.draw_string(tx, ty, k, Color::COSMIC_TEXT_SEC, 1);
        r.draw_string(tx + 100, ty, v, Color::COSMIC_TEXT, 1);
        ty += 20;
    }

    ty += 16;
    r.draw_hline(tx, ty, 360, Color::COSMIC_BORDER);
    ty += 16;

    r.draw_string(tx, ty, "Account", Color::COSMIC_TEXT, 2);
    ty += 30;
    r.draw_string(tx, ty, "Utente:", Color::COSMIC_TEXT_SEC, 1);
    r.draw_string(tx + 100, ty, "admin", Color::COSMIC_TEXT, 1);
    ty += 20;
    r.draw_string(tx, ty, "Ruolo:", Color::COSMIC_TEXT_SEC, 1);
    r.draw_string(tx + 100, ty, "Amministratore", Color::COSMIC_TEXT, 1);
}
