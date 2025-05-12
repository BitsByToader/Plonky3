#[repr(C)]
#[derive(Clone, Copy)]
pub struct MappedMonolith {
    pub fd: cty::c_int,
    pub base: *mut u32
}

unsafe extern "C" {
    pub fn map_monolith() -> MappedMonolith;
    pub fn unmap_monolith(map: *mut MappedMonolith);
    pub fn monolith_hash(acc: MappedMonolith, input1: u32) -> u32;
    pub fn monolith_compress(acc: MappedMonolith, input1: u32, input2: u32) -> u32;
}