const PAGE_SIZE: usize = 4096;

struct Page {
    id: u32,
    data: [u8; PAGE_SIZE],
}

impl Page {
    fn new(id: u32) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
        }
    }

    // [u8; 4] for acutual bytes. length known by the type.
    // [&u8] borrowed view into some bytes. length known at runtime.
    fn write_u32(&mut self, offset: usize, value: u32) {
        // 42 = 0x0000002A
        let bytes = value.to_le_bytes(); // 2A 00 00 00 (little endian)
        println!("bytes = {:?}", bytes);
        self.data[offset..offset + 4].copy_from_slice(&bytes);
        println!("page data = {:?}", &self.data[0..8]);
    }

    fn read_u32(&self, offset: usize) -> u32 {
        // &[u8](length X) -> [u8; 4]
        let bytes: [u8; 4] = self.data[offset..offset + 4].try_into().unwrap();

        u32::from_le_bytes(bytes)
    }
}

fn main() {
    let mut page = Page::new(0);

    page.write_u32(1, 42);

    let value = page.read_u32(1);

    println!("page id {}", page.id);
    println!("value: {}", value);
}
