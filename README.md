# cyw43-controller

Bluetooth LE isochronous support for the CYW43439, in the shape `nrf-sdc` uses
for Nordic parts: a vendor-specific controller behind Rust crates.

The CYW43439's stock `.0065` firmware reports no ISO support. These crates
reach it through a resident patch installed into controller SRAM at startup,
so the controller answers ordinary HCI ISO. This is a reverse-engineering
experiment against one firmware build, not a qualified Bluetooth
implementation and not standard CIS/BIG negotiation.

## Crates

| Crate | Role |
| --- | --- |
| [`cyw43-controller-proto`](cyw43-controller-proto) | Link-layer and LE Audio byte formats. No host stack, no radio. |

Planned, mirroring `nrf-sdc`'s split:

- `cyw43-controller-blob`: the resident patch bytes and their ABI manifest,
  generated and validated in `cyw43_iso`, analogous to `nrf-sdc-sys`.
- `cyw43-controller`: the host driver that installs a blob, owns the HCI
  transport, and presents ISO, analogous to `nrf-sdc`.

Consumers are expected to be `trouble_audio` for LE Audio profiles,
`rp2040_audio` for Pico W applications, and the independent nRF54L15 test
firmware that validates the radio.

## Scope

`proto` encodes, decodes and derives. It does not establish a CIS or BIG,
schedule radio events, assign sequence numbers, or recover synchronisation
references; those belong to whatever owns the radio. Encoding a value is never
evidence of a corresponding on-air event.

Builds on `thumbv6m-none-eabi` and `thumbv8m.main-none-eabihf` as well as the
host. `no_std` + `alloc` by default, `unsafe_code = "forbid"`.
