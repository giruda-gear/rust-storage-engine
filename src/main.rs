const PAGE_SIZE: usize = 4096; // 4kb
// u32 = 4 bytes / 8 bit = 1 byte
const PAGE_ID_OFFSET: usize = 0; // offset 0..4
const KEY_COUNT_OFFSET: usize = 4; // offset 4..8  number of keys
const ENTRIES_OFFSET: usize = 8; // page id & key count 0..8
const ENTRIES_SIZE: usize = 8; // key(u32) + value(u32)

struct Page {
    // id: u32, -> PAGE_ID_OFFSET
    data: [u8; PAGE_SIZE],
}

impl Page {
    fn new(id: u32) -> Self {
        let mut page = Self {
            // id,
            data: [0; PAGE_SIZE],
        };

        page.write_u32(PAGE_ID_OFFSET, id);

        page
    }

    // [u8; 4] for acutual bytes. length known by the type.
    // [&u8] borrowed view into some bytes. length known at runtime.
    fn write_u32(&mut self, offset: usize, value: u32) {
        // 42 = 0x0000002A
        let bytes = value.to_le_bytes(); // 2A 00 00 00 (little endian)
        self.data[offset..offset + 4].copy_from_slice(&bytes);
        println!("data[{}] = {:?}", offset, &self.data[offset..offset + 4]);
    }

    fn read_u32(&self, offset: usize) -> u32 {
        // &[u8](length unknown) -> [u8; 4]
        let bytes: [u8; 4] = self.data[offset..offset + 4].try_into().unwrap();

        u32::from_le_bytes(bytes)
    }

    fn id(&self) -> u32 {
        self.read_u32(PAGE_ID_OFFSET)
    }

    fn key_count(&self) -> u32 {
        self.read_u32(KEY_COUNT_OFFSET)
    }

    fn set_key_count(&mut self, count: u32) {
        self.write_u32(KEY_COUNT_OFFSET, count)
    }

    fn entry_offset(index: usize) -> usize {
        ENTRIES_OFFSET + index * ENTRIES_SIZE
    }
}

fn main() {
    let mut page = Page::new(42);

    page.write_u32(ENTRIES_OFFSET, 10);
    page.write_u32(ENTRIES_OFFSET + 4, 100);

    page.set_key_count(1);

    let key = page.read_u32(ENTRIES_OFFSET);
    let value = page.read_u32(ENTRIES_OFFSET + 4);

    println!("page id: {}", page.id());
    println!("key count: {}", page.key_count());
    println!("key = {}", key);
    println!("value = {}", value);

    println!("entry 0 offset = {}", Page::entry_offset(0));
    println!("entry 1 offset = {}", Page::entry_offset(1));
}
