# cyw43-controller

Installs the resident ISO patch into a CYW43439's controller SRAM and presents
ordinary HCI ISO on a part whose stock `.0065` firmware reports no ISO support.

Analogous to `nrf-sdc`: the controller-specific layer, with no host stack and
no bus driver of its own. The caller supplies exclusive startup access to the
controller through the [`Runtime`] trait; whichever driver owns the SPI/SDIO
bus implements it.

[`BOOT_FIRMWARE`] re-exports the exact `.0065` HCD container with the required
112-byte zero reservation. Load it during CYW43 Bluetooth initialization; no
application-local firmware path or per-image copy is required.

This is a reverse-engineering experiment against one firmware build, not a
qualified Bluetooth implementation and not standard CIS/BIG negotiation. Every
error or cancelled future requires a chip restart and firmware reload.
