pub const CHUNK_SZ: usize = 0x15A;
pub const CHUNK_OFFSET: usize = 0x2B;
pub const MAGIC_NUMBERS: [u8; 8] = [8, 10, 9, 11, 1, 7, 1, 7];

pub fn _num(c: usize) -> String {
    let mut tmp = String::from("0000");
    let mut cc = c;
    for ii in (0..4).rev() {
        tmp.replace_range(ii..ii+1, &((cc % 10) as u8).to_string());
        cc /= 10;
    }
    tmp
}

pub fn num(c: usize) -> String {
    format!("{:04}", c)
}