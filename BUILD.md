# CosmicOS — Istruzioni di compilazione e avvio

## Prerequisiti

### Linux / WSL2 (raccomandato su Windows)

```bash
# 1. Installa Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup component add rust-src llvm-tools --toolchain nightly

# 2. Installa strumenti di sistema
sudo apt update
sudo apt install -y qemu-system-x86 xorriso \
                   ovmf git make

# 3. Verifica OVMF (per boot UEFI)
ls /usr/share/OVMF/OVMF_CODE.fd
# Se non esiste: sudo apt install -y ovmf
```

### macOS

```bash
brew install qemu xorriso
# Rust nightly come sopra
```

---

## Compilazione

```bash
cd CosmicOS

# Scarica Limine (solo la prima volta — automatico con make)
make limine-fetch

# Compila il kernel + crea ISO
make iso
```

---

## Avvio in QEMU

### UEFI (raccomandato)

```bash
make run
```

### BIOS (fallback)

```bash
make run-bios
```

---

## Alternativa: solo kernel (senza ISO)

```bash
cd kernel
cargo build --release \
    --target ../x86_64-cosmic.json \
    -Z build-std=core,alloc,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem
```

Il binario ELF sarà in:
`kernel/target/x86_64-cosmic/release/cosmic-kernel`

---

## Pulizia

```bash
make clean
```

---

## Note importanti

- **Rust nightly obbligatorio**: usa `rust-toolchain.toml` nella root — Cargo lo seleziona in automatico.
- **`-Z build-std`**: necessario per compilare `core` e `alloc` senza la libreria standard.
- **OVMF**: firmware UEFI per QEMU. Se non disponibile, usa `make run-bios`.
- **QEMU VGA**: il framebuffer è esposto su VGA std. Se i colori non corrispondono, prova `-vga virtio` nel Makefile.
- **Limine v7**: il Makefile clona la branch `v7.x-binary` di Limine.

---

## Credenziali login default

| Campo    | Valore  |
|----------|---------|
| Utente   | `admin` |
| Password | `cosmic`|

---

## Scorciatoie desktop

| Tasto | Azione                       |
|-------|------------------------------|
| `I`   | Apri/chiudi Info di sistema  |
| `F`   | Apri/chiudi Gestione file    |
| `S`   | Apri/chiudi Impostazioni     |

---

## Struttura progetto

```
CosmicOS/
├── rust-toolchain.toml     # Rust nightly
├── Makefile                # Build + run
├── limine.conf             # Config bootloader
├── BUILD.md                # Questo file
└── kernel/
    ├── Cargo.toml          # Dipendenze Rust
    ├── .cargo/config.toml  # Target custom + build-std
    ├── x86_64-cosmic.json  # Target JSON no_std
    ├── linker.ld           # Linker script
    └── src/
        ├── main.rs         # Entry point kernel
        ├── panic.rs        # Panic handler
        ├── gdt.rs          # Global Descriptor Table
        ├── interrupts.rs   # IDT + PIC 8259
        ├── memory/         # Heap allocator
        ├── gfx/            # Graphics subsystem
        │   ├── color.rs    # Palette CosmicOS
        │   ├── font.rs     # Font bitmap 8×8
        │   └── renderer.rs # Renderer framebuffer
        ├── input/          # PS/2 keyboard driver
        ├── vfs/            # Virtual File System
        └── ui/             # Pipeline grafica
            ├── boot_screen.rs
            ├── login.rs
            ├── desktop.rs
            └── window.rs
```
