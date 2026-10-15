use cyw43_controller_blob::{BOOT_FIRMWARE, BOOTSTRAP_ADDRESS};

#[test]
fn boot_firmware_contains_exact_bootstrap_reservation() {
    let count_offset = usize::from(BOOT_FIRMWARE[0]) + 1;
    let record_count = usize::from(BOOT_FIRMWARE[count_offset]);
    let mut offset = count_offset + 1;
    let mut base = 0_u32;
    let mut reservations = 0;
    let mut end_records = 0;

    assert!(BOOT_FIRMWARE.starts_with(b"JCYW4343A2_001.003.016.0065"));

    for index in 0..record_count {
        let length = usize::from(BOOT_FIRMWARE[offset]);
        let address = u32::from(u16::from_be_bytes([
            BOOT_FIRMWARE[offset + 1],
            BOOT_FIRMWARE[offset + 2],
        ]));
        let record_type = BOOT_FIRMWARE[offset + 3];
        let payload = &BOOT_FIRMWARE[offset + 4..offset + 4 + length];

        match record_type {
            0 if base + address == BOOTSTRAP_ADDRESS => {
                assert_eq!(payload.len(), 112);
                assert!(payload.iter().all(|byte| *byte == 0));
                reservations += 1;
            }
            1 => {
                assert_eq!(index, record_count - 1);
                assert!(payload.is_empty());
                end_records += 1;
            }
            2 => base = u32::from(u16::from_be_bytes(payload.try_into().unwrap())) << 4,
            4 => base = u32::from(u16::from_be_bytes(payload.try_into().unwrap())) << 16,
            _ => {}
        }
        offset += 4 + length;
    }

    assert_eq!(offset, BOOT_FIRMWARE.len());
    assert_eq!(reservations, 1);
    assert_eq!(end_records, 1);
}
