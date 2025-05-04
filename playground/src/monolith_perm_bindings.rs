#[repr(C)]
pub struct MappedMonolith {
    pub fd: cty::c_int,
    pub base: u32
}

extern "C" {
    pub fn map_monolith() -> MappedMonolith;
    pub fn unmap_monolith(map: *mut MappedMonolith);
    pub fn monolith_hash(acc: *mut MappedMonolith, input1: u32) -> u32;
    pub fn monolith_compress(acc: *mut MappedMonolith, input1: u32, input2: u32) -> u32;
}