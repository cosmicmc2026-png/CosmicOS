//! Nodi del Virtual File System CosmicOS.

use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Directory,
    File,
}

pub struct FsNode {
    pub name:     String,
    pub kind:     NodeKind,
    pub data:     Vec<u8>,         // contenuto (solo per File)
    pub children: Vec<FsNode>,     // figli (solo per Directory)
}

impl FsNode {
    /// Crea una nuova directory.
    pub fn dir(name: &str) -> Self {
        Self {
            name:     String::from(name),
            kind:     NodeKind::Directory,
            data:     Vec::new(),
            children: Vec::new(),
        }
    }

    /// Crea un nuovo file con contenuto iniziale.
    pub fn file(name: &str, data: &[u8]) -> Self {
        Self {
            name:     String::from(name),
            kind:     NodeKind::File,
            data:     Vec::from(data),
            children: Vec::new(),
        }
    }

    pub fn is_dir(&self)  -> bool { self.kind == NodeKind::Directory }
    pub fn is_file(&self) -> bool { self.kind == NodeKind::File }

    /// Aggiunge un figlio (solo per directory).
    pub fn add_child(&mut self, child: FsNode) {
        if self.is_dir() {
            self.children.push(child);
        }
    }

    /// Cerca un figlio per nome (immutabile).
    pub fn child(&self, name: &str) -> Option<&FsNode> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Cerca un figlio per nome (mutabile).
    pub fn child_mut(&mut self, name: &str) -> Option<&mut FsNode> {
        self.children.iter_mut().find(|c| c.name == name)
    }

    /// Estensione del file (parte dopo l'ultimo '.').
    pub fn extension(&self) -> Option<&str> {
        let dot = self.name.rfind('.')?;
        Some(&self.name[dot + 1..])
    }

    /// Dimensione: byte per file, numero di figli per directory.
    pub fn size(&self) -> usize {
        if self.is_file() { self.data.len() } else { self.children.len() }
    }

    /// Scrive dati in un file (sovrascrive).
    pub fn write(&mut self, data: &[u8]) {
        if self.is_file() {
            self.data = Vec::from(data);
        }
    }

    /// Appende dati a un file.
    pub fn append(&mut self, data: &[u8]) {
        if self.is_file() {
            self.data.extend_from_slice(data);
        }
    }
}
