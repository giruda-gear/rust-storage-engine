### Page API Layers
```text
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
```
---
### DAY 1 Integer ↔ Bytes
```text
                  WRITE
42u32
  │
  │ to_le_bytes()
  ▼
[42, 0, 0, 0]              ← [u8; 4]
  │
  │ copy_from_slice()
  ▼
┌──────────────────────────────┐
│ 42 │ 0 │ 0 │ 0 │ 0 │ 0 ... │  Page
└──────────────────────────────┘

                  READ
┌──────────────────────────────┐
│ 42 │ 0 │ 0 │ 0 │ 0 │ 0 ... │  Page
└──────────────────────────────┘
  │
  │ [0..4]
  ▼
[42, 0, 0, 0]              ← &[u8]
  │
  │ try_into()
  ▼
[42, 0, 0, 0]              ← [u8; 4]
  │
  │ u32::from_le_bytes()
  ▼
42u32
```
**Rust Perspective**

`&[u8]` and `[u8; 4]` are different types, so `try_into()` is needed for the conversion.

**Storage Perspective**

There is no `u32` on disk. There are only bytes interpreted according to the byte layout we define.

---

### DAY 2 PAGE HEADER
```text
offset
 0             4            8            12           16
 │             │            │             │            │
 ▼             ▼            ▼             ▼            ▼
┌─────────────┬─────────────┬─────────────┬─────────────┐
│ 2A 00 00 00 │ 01 00 00 00 │ 0A 00 00 00 │ 64 00 00 00 │
└─────────────┴─────────────┴─────────────┴─────────────┘
      42             1            10            100
    page ID      key count        key          value
```
---
### DAY 3 Fixed-size Entries
```text
0           4           8                  16                 24
│           │           │                   │                  │
▼           ▼           ▼                   ▼                  ▼
┌───────────┬───────────┬─────────┬─────────┬─────────┬─────────┐
│ page id   │ key count │ key 10  │ val 100 │ key 20  │ val 200 │
└───────────┴───────────┴─────────┴─────────┴─────────┴─────────┘
                        └──── entry 0 ──────┘└──── entry 1 ─────┘
```