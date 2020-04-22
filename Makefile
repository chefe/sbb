# Install to /usr unless otherwise specified, such as `make PREFIX=/app`
PREFIX=/usr

# What to run to install various files
INSTALL=install
INSTALL_PROGRAM=$(INSTALL)
INSTALL_DATA=$(INSTALL) -m 644

# Directories into which to install the various files
BIN_DIR=$(DESTDIR)$(PREFIX)/bin
SHARE_DIR=$(DESTDIR)$(PREFIX)/share

# Architecture for the flatpak build, default to x86_64 if not set
FLATPAK_ARCH ?= x86_64

# Define targets without a explicite file
.PHONY: clean clean-cargo clean-flatpak install uninstall flatpak aarch64-flatpak

debug: target/debug/sbb

release: target/release/sbb

target/debug/sbb: src Cargo.lock Cargo.toml
	cargo build

target/release/sbb: src Cargo.lock Cargo.toml
	cargo build --release

install: target/release/sbb data
	mkdir -p $(BIN_DIR)
	$(INSTALL_PROGRAM) target/release/sbb $(BIN_DIR)/io.chefe.sbb
	mkdir -p $(SHARE_DIR)/applications
	$(INSTALL_DATA) data/io.chefe.sbb.desktop $(SHARE_DIR)/applications/io.chefe.sbb.desktop
	mkdir -p $(SHARE_DIR)/icons/hicolor/scalable/apps/
	$(INSTALL_DATA) data/io.chefe.sbb.svg $(SHARE_DIR)/icons/hicolor/scalable/apps/io.chefe.sbb.svg
	touch $(SHARE_DIR)/icons/hicolor # Force icon cache refresh

uninstall:
	rm -f $(SHARE_DIR)/applications/io.chefe.sbb.desktop
	rm -f $(SHARE_DIR)/icons/hicolor/scalable/apps/io.chefe.sbb.svg
	rm -f $(BIN_DIR)/io.chefe.sbb

install-flatpak: io.chefe.sbb.$(FLATPAK_ARCH).flatpak
	flatpak install --user io.chefe.sbb.$(FLATPAK_ARCH).flatpak

flatpak: io.chefe.sbb.$(FLATPAK_ARCH).flatpak

io.chefe.sbb.$(FLATPAK_ARCH).flatpak: src data Cargo.lock Cargo.toml
	flatpak-builder --user --install-deps-from=flathub --arch=$(FLATPAK_ARCH) --force-clean --repo=.flatpak-repo .flatpak-build-$(FLATPAK_ARCH) data/io.chefe.sbb.json
	flatpak build-bundle --arch=$(FLATPAK_ARCH) .flatpak-repo io.chefe.sbb.$(FLATPAK_ARCH).flatpak io.chefe.sbb

aarch64-flatpak:
	FLATPAK_ARCH=aarch64 make flatpak

clean: clean-cargo clean-flatpak

clean-cargo:
	cargo clean

clean-flatpak:
	rm -rf .flatpak-build-* .flatpak-builder .flatpak-repo io.chefe.sbb.*.flatpak
