//! Driver tastiera PS/2 - Set 1 (XT) scan codes.
//! Gestisce shift, capslock; produce InputEvent tramite ring buffer.

use spin::Mutex;
use crate::input::InputEvent;

// ─── Ring buffer ─────────────────────────────────────────────────────────────

const BUF_SIZE: usize = 256;

struct RingBuffer {
    buf:   [Option<InputEvent>; BUF_SIZE],
    read:  usize,
    write: usize,
}

impl RingBuffer {
    const fn new() -> Self {
        Self {
            buf:   [None; BUF_SIZE],
            read:  0,
            write: 0,
        }
    }

    fn push(&mut self, ev: InputEvent) {
        let next = (self.write + 1) % BUF_SIZE;
        if next != self.read {
            self.buf[self.write] = Some(ev);
            self.write = next;
        }
        // Se il buffer è pieno, l'evento viene scartato silenziosamente
    }

    fn pop(&mut self) -> Option<InputEvent> {
        if self.read == self.write {
            return None;
        }
        let ev = self.buf[self.read].take();
        self.read = (self.read + 1) % BUF_SIZE;
        ev
    }
}

static KEYBOARD_BUFFER: Mutex<RingBuffer> = Mutex::new(RingBuffer::new());

// ─── Stato modifier keys ─────────────────────────────────────────────────────

struct ModState {
    shift:    bool,
    capslock: bool,
}

static MOD_STATE: Mutex<ModState> = Mutex::new(ModState {
    shift:    false,
    capslock: false,
});

// ─── API pubblica ─────────────────────────────────────────────────────────────

/// Chiamata dall'interrupt handler. Processa il scancode e mette l'evento nel buffer.
pub fn handle_scancode(sc: u8) {
    let release = sc & 0x80 != 0;
    let sc_base = sc & 0x7F;

    let mut mods = MOD_STATE.lock();

    // Gestione tasti modificatori
    match sc_base {
        0x2A | 0x36 => { mods.shift = !release; return; } // L/R Shift
        0x3A if !release => {                               // Caps Lock (toggle)
            mods.capslock = !mods.capslock;
            return;
        }
        0x3A => return, // Caps Lock release - ignora
        _ => {}
    }

    if release { return; } // Ignora tutti gli altri key-release

    let shift = mods.shift;
    let caps  = mods.capslock;
    drop(mods);

    let ev = scancode_to_event(sc_base, shift, caps);
    if let Some(e) = ev {
        KEYBOARD_BUFFER.lock().push(e);
    }
}

/// Preleva il prossimo evento dal buffer (chiamata dal main loop).
/// Da usare dentro `without_interrupts` per sicurezza.
pub fn pop_event() -> Option<InputEvent> {
    KEYBOARD_BUFFER.lock().pop()
}

// ─── Tabella scan code Set 1 → InputEvent ────────────────────────────────────

fn scancode_to_event(sc: u8, shift: bool, caps: bool) -> Option<InputEvent> {
    // Tasti speciali
    let special = match sc {
        0x01 => Some(InputEvent::Escape),
        0x0E => Some(InputEvent::Backspace),
        0x0F => Some(InputEvent::Tab),
        0x1C => Some(InputEvent::Enter),
        0x48 => Some(InputEvent::Up),
        0x50 => Some(InputEvent::Down),
        0x4B => Some(InputEvent::Left),
        0x4D => Some(InputEvent::Right),
        _    => None,
    };
    if special.is_some() { return special; }

    // Caratteri ASCII
    let ch = sc_to_char(sc, shift, caps)?;
    Some(InputEvent::Char(ch))
}

fn sc_to_char(sc: u8, shift: bool, caps: bool) -> Option<char> {
    // Tabella: [sc] = (unshifted, shifted)
    let (lo, hi): (char, char) = match sc {
        0x02 => ('1','!'),  0x03 => ('2','@'),  0x04 => ('3','#'),
        0x05 => ('4','$'),  0x06 => ('5','%'),  0x07 => ('6','^'),
        0x08 => ('7','&'),  0x09 => ('8','*'),  0x0A => ('9','('),
        0x0B => ('0',')'),  0x0C => ('-','_'),  0x0D => ('=','+'),

        0x10 => ('q','Q'),  0x11 => ('w','W'),  0x12 => ('e','E'),
        0x13 => ('r','R'),  0x14 => ('t','T'),  0x15 => ('y','Y'),
        0x16 => ('u','U'),  0x17 => ('i','I'),  0x18 => ('o','O'),
        0x19 => ('p','P'),  0x1A => ('[','{'),  0x1B => (']','}'),

        0x1E => ('a','A'),  0x1F => ('s','S'),  0x20 => ('d','D'),
        0x21 => ('f','F'),  0x22 => ('g','G'),  0x23 => ('h','H'),
        0x24 => ('j','J'),  0x25 => ('k','K'),  0x26 => ('l','L'),
        0x27 => (';',':'),  0x28 => ('\'','"'), 0x29 => ('`','~'),

        0x2B => ('\\','|'),
        0x2C => ('z','Z'),  0x2D => ('x','X'),  0x2E => ('c','C'),
        0x2F => ('v','V'),  0x30 => ('b','B'),  0x31 => ('n','N'),
        0x32 => ('m','M'),  0x33 => (',','<'),  0x34 => ('.','>'),
        0x35 => ('/','?'),

        0x39 => (' ',' '),  // Spazio

        _ => return None,
    };

    // Caps lock si applica solo alle lettere
    let is_letter = lo.is_ascii_alphabetic();
    let use_upper = if is_letter { shift ^ caps } else { shift };
    Some(if use_upper { hi } else { lo })
}
