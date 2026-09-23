High-level page API
────────────────────
id()
key_count()
set_key_count()

        ↓ uses

Low-level byte API
────────────────────
read_u32()
write_u32()

        ↓ operates on

Raw storage
────────────────────
data: [u8; 4096]