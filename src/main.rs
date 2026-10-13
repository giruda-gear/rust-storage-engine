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

    fn write_entry(&mut self, index: usize, key: u32, value: u32) {
        let offset = Self::entry_offset(index);

        self.write_u32(offset, key);
        self.write_u32(offset + 4, value);
    }

    fn read_entry(&self, index: usize) -> (u32, u32) {
        let offset = Self::entry_offset(index);

        let key = self.read_u32(offset);
        let value = self.read_u32(offset + 4);

        (key, value)
    }

    // binary search (array is already sorted.)
    fn get(&self, key: u32) -> Option<u32> {
        let mut left = 0;
        let mut right = self.key_count() as usize;

        while left < right {
            let mid = left + (right - left) / 2;
            let (stored_key, stored_value) = self.read_entry(mid);

            if stored_key == key {
                return Some(stored_value);
            }

            if stored_key < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        None
    }

    fn insert(&mut self, key: u32, value: u32) {
        let count = self.key_count() as usize;
        let position = self.find_insert_position(key);

        if position < count {
            let (stored_key, _) = self.read_entry(position);

            if stored_key == key {
                self.write_entry(position, key, value);
                return;
            }
        }

        self.shift_right(position);
        self.write_entry(position, key, value);
        self.set_key_count((count + 1) as u32);
    }

    fn find_insert_position(&self, key: u32) -> usize {
        let count = self.key_count() as usize;

        for index in 0..count {
            let (stored_key, _) = self.read_entry(index);

            if stored_key >= key {
                return index;
            }
        }

        count
    }

    fn shift_right(&mut self, start: usize) {
        let count = self.key_count() as usize;

        for index in (start..count).rev() {
            let (key, value) = self.read_entry(index);

            self.write_entry(index + 1, key, value);
        }
    }
}

fn main() {
    let mut page = Page::new(1);

    page.insert(20, 100);
    page.insert(10, 200);
    page.insert(30, 300);

    page.insert(20, 299);

    // page.shift_right(1);

    for index in 0..page.key_count() as usize {
        println!("{:?}", page.read_entry(index));
    }

    println!("{:?}", page.get(20));
}
