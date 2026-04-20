//! IDT e inizializzazione del PIC 8259.
//! Gestisce: breakpoint, double fault, page fault, keyboard (IRQ1).

use lazy_static::lazy_static;
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use crate::gdt::DOUBLE_FAULT_IST_INDEX;

// ─── Offset interrupt PIC ────────────────────────────────────────────────────

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer    = PIC1_OFFSET,
    Keyboard = PIC1_OFFSET + 1,
}

// ─── IDT ─────────────────────────────────────────────────────────────────────

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
    unsafe { pic_init(); }
}

// ─── PIC 8259 init ───────────────────────────────────────────────────────────

const PIC1_CMD:  u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD:  u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;

unsafe fn io_wait() {
    let mut port: Port<u8> = Port::new(0x80);
    port.write(0u8);
}

unsafe fn pic_init() {
    let mut p1c: Port<u8> = Port::new(PIC1_CMD);
    let mut p1d: Port<u8> = Port::new(PIC1_DATA);
    let mut p2c: Port<u8> = Port::new(PIC2_CMD);
    let mut p2d: Port<u8> = Port::new(PIC2_DATA);

    // Sequenza di inizializzazione
    p1c.write(ICW1_INIT); io_wait();
    p2c.write(ICW1_INIT); io_wait();

    // Offset vettori
    p1d.write(PIC1_OFFSET); io_wait();
    p2d.write(PIC2_OFFSET); io_wait();

    // Cascading: PIC1 ha PIC2 su IRQ2
    p1d.write(0x04); io_wait();
    p2d.write(0x02); io_wait();

    // Modalità 8086
    p1d.write(ICW4_8086); io_wait();
    p2d.write(ICW4_8086); io_wait();

    // Maschera: abilita solo IRQ1 (tastiera), disabilita tutto il resto
    p1d.write(0xFD); // 1111_1101 → solo IRQ1 abilitato
    p2d.write(0xFF); // tutto mascherato su PIC2
}

pub fn pic_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            let mut p2c: Port<u8> = Port::new(PIC2_CMD);
            p2c.write(0x20);
        }
        let mut p1c: Port<u8> = Port::new(PIC1_CMD);
        p1c.write(0x20);
    }
}

// ─── Handler: Breakpoint ─────────────────────────────────────────────────────

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // Ignorato silenziosamente in v0.0.1
    let _ = stack_frame;
}

// ─── Handler: Double Fault ───────────────────────────────────────────────────

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT - errore critico della CPU");
}

// ─── Handler: Page Fault ─────────────────────────────────────────────────────

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    panic!("PAGE FAULT @ {:?} - codice: {:?}", addr, error_code);
}

// ─── Handler: Keyboard IRQ1 ──────────────────────────────────────────────────

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port: Port<u8> = Port::new(0x60);
    let scancode = unsafe { port.read() };

    crate::input::keyboard::handle_scancode(scancode);

    pic_eoi(1);
}
