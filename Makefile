# CosmicOS Build System
# Cosmic v0.0.1 Developer Beta

KERNEL_ELF  := kernel/target/x86_64-cosmic/release/cosmic-kernel
ISO_DIR     := iso_root
ISO         := CosmicOS.iso
LIMINE_DIR  := limine
OVMF        := /usr/share/OVMF/OVMF_CODE.fd
QEMU        := qemu-system-x86_64

.PHONY: all build iso run run-bios clean limine-fetch

all: iso

# ─── Build kernel ────────────────────────────────────────────────────────────
build:
	cd kernel && cargo build --release \
		--target x86_64-cosmic.json \
		-Z build-std=core,alloc,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem \
		-Z json-target-spec

# ─── Fetch Limine (v7 latest stable) ─────────────────────────────────────────
limine-fetch:
	@if [ ! -d "$(LIMINE_DIR)" ]; then \
		git clone https://github.com/limine-bootloader/limine.git \
			--branch=v7.x-binary --depth=1 $(LIMINE_DIR); \
		make -C $(LIMINE_DIR); \
	fi

# ─── Build ISO ────────────────────────────────────────────────────────────────
iso: build limine-fetch
	rm -rf $(ISO_DIR)
	mkdir -p $(ISO_DIR)/boot/limine
	mkdir -p $(ISO_DIR)/EFI/BOOT
	cp $(KERNEL_ELF)                              $(ISO_DIR)/boot/cosmic-kernel.elf
	cp limine.conf                                $(ISO_DIR)/boot/limine/limine.conf
	cp $(LIMINE_DIR)/limine-bios.sys              $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/limine-bios-cd.bin           $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/limine-uefi-cd.bin           $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/BOOTX64.EFI                  $(ISO_DIR)/EFI/BOOT/
	cp $(LIMINE_DIR)/BOOTIA32.EFI                 $(ISO_DIR)/EFI/BOOT/ 2>/dev/null || true
	xorriso -as mkisofs \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot \
		-boot-load-size 4 \
		-boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part \
		--efi-boot-image \
		--protective-msdos-label \
		$(ISO_DIR) \
		-o $(ISO) 2>/dev/null
	$(LIMINE_DIR)/limine bios-install $(ISO) 2>/dev/null || true

# ─── Run UEFI (recommended) ──────────────────────────────────────────────────
run: iso
	$(QEMU) \
		-machine q35 \
		-bios $(OVMF) \
		-cdrom $(ISO) \
		-m 512M \
		-serial stdio \
		-vga std \
		-no-reboot \
		-no-shutdown

# ─── Run BIOS fallback ────────────────────────────────────────────────────────
run-bios: iso
	$(QEMU) \
		-machine q35 \
		-cdrom $(ISO) \
		-m 512M \
		-serial stdio \
		-vga std \
		-no-reboot \
		-no-shutdown

# ─── Clean ───────────────────────────────────────────────────────────────────
clean:
	cd kernel && cargo clean
	rm -rf $(ISO_DIR) $(ISO)
