pub const CHUNK_SZ: usize = 0x15A;
pub const CHUNK_OFFSET: usize = 0x2B;
pub const MAGIC_NUMBERS: [u8; 8] = [8, 10, 9, 11, 1, 7, 1, 7];

pub const ENCODING: &str = "cp852";

pub fn num(c: usize) -> String {
    format!("{:04}", c)
}