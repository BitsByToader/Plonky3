use memmap2::MmapOptions;
use memmap2::MmapMut;
use std::fs::OpenOptions;

pub trait OffsetAddrRW {
    fn write_u32(&mut self, index: usize, value: u32) -> Option<()>;
    fn read_u32(&self, index: usize) -> Option<u32>;
}

impl OffsetAddrRW for MmapMut {
    fn write_u32(&mut self, index:usize, value: u32) -> Option<()> {
        // Convert the u32 value to its byte representation in native endianness
        let bytes_to_write = value.to_ne_bytes(); // Returns [u8; 4]

        // Calculate the required end index (exclusive)
        let end_index = index.checked_add(4)?;

        // Safely get the mutable 4-byte sub-slice
        // .get_mut() handles bounds checking
        let target_slice = self.get_mut(index..end_index)?;

        // Copy the bytes into the target slice
        target_slice.copy_from_slice(&bytes_to_write);

        Some(())
    }

    fn read_u32(&self, index: usize) -> Option<u32> {
        // Calculate the required end index (exclusive)
        let end_index = index.checked_add(4)?;

        // Safely get the 4-byte sub-slice
        let byte_slice = self.get(index..end_index)?;

        // Convert the slice to a fixed-size array [u8; 4]
        // This unwrap is safe because .get() ensured the slice has 4 bytes.
        let byte_array: [u8; 4] = byte_slice.try_into().unwrap();

        // Convert the array to u32 using native endianness
        let value = u32::from_ne_bytes(byte_array);

        Some(value)
    }
}

pub struct HWMonolith {
    pub mmap: MmapMut
}

impl HWMonolith {
    pub fn new() -> HWMonolith {
        let mmap: MmapMut = unsafe {
            MmapOptions::new()
                .len(16)
                .offset(0x43C0_0000)
                .map_mut(&OpenOptions::new().read(true).write(true).open("/dev/mem").unwrap()).unwrap()
        };

        HWMonolith { mmap }
    }

    pub fn hash(&mut self, input: u32) -> u32 {
        loop {
            self.mmap.write_u32(0, (input<<1) | 1);
            let out = self.mmap.read_u32(8).unwrap();

            if (out&1) == 1 {
                return out >> 1;
            }
        }
    }
}