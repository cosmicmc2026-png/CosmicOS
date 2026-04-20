//! Virtual File System CosmicOS — v0.0.1
//!
//! Struttura in-memory con estensioni custom .cos*.
//! Non c'è persistenza su disco: tutto è RAM-only nella prima versione.

pub mod node;

use node::FsNode;
use spin::Mutex;

// ─── Estensioni custom CosmicOS ───────────────────────────────────────────────

/// Tabella delle estensioni native CosmicOS
pub mod ext {
    pub const APP:  &str = "cosapp";  // Applicazione eseguibile
    pub const BIN:  &str = "cosbin";  // File binario/dati
    pub const CFG:  &str = "coscfg";  // Configurazione
    pub const LNK:  &str = "coslnk";  // Collegamento/shortcut
    pub const TXT:  &str = "costxt";  // Documento testo
    pub const IMG:  &str = "cosimg";  // Immagine
    pub const PKG:  &str = "cospkg";  // Pacchetto/installer
    pub const SYS:  &str = "cossys";  // File di sistema
    pub const LOG:  &str = "coslog";  // Log
    pub const LIB:  &str = "coslib";  // Libreria
}

// ─── Root del VFS ────────────────────────────────────────────────────────────

static VFS_ROOT: Mutex<Option<FsNode>> = Mutex::new(None);

pub fn init() {
    let mut root = FsNode::dir("/");

    // Desktop (con file di benvenuto)
    let mut desktop = FsNode::dir("Desktop");
    desktop.add_child(FsNode::file(
        "Benvenuto.costxt",
        b"Benvenuto in CosmicOS!\nPrestazioni spaziali.",
    ));
    root.add_child(desktop);

    // Altre cartelle utente
    root.add_child(FsNode::dir("Download"));
    root.add_child(FsNode::dir("Documenti"));
    root.add_child(FsNode::dir("Immagini"));
    root.add_child(FsNode::dir("Cestino"));

    // Cartelle di sistema
    let mut sys = FsNode::dir("Sistema");
    sys.add_child(FsNode::file("kernel.cossys",  b"CosmicOS Kernel v0.0.1"));
    sys.add_child(FsNode::file("config.coscfg",  b"theme=light\nlocale=it"));
    sys.add_child(FsNode::file("boot.coslog",     b"Avvio completato."));
    root.add_child(sys);

    *VFS_ROOT.lock() = Some(root);
}

pub fn with_root<F, R>(f: F) -> R
where
    F: FnOnce(&FsNode) -> R,
{
    let lock = VFS_ROOT.lock();
    f(lock.as_ref().expect("VFS non inizializzato"))
}

pub fn with_root_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut FsNode) -> R,
{
    let mut lock = VFS_ROOT.lock();
    f(lock.as_mut().expect("VFS non inizializzato"))
}
