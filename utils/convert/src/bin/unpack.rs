use std::fs::File;
use std::io::{BufWriter, Read, Write};
use convert::{CHUNK_OFFSET, CHUNK_SZ, ENCODING, MAGIC_NUMBERS, num};


const BUF_SIZE: usize = 260000;

fn main() {
    let mut buf = Vec::with_capacity(BUF_SIZE);
    let mut f = File::open("RAVEN.DAT").unwrap();
    f.read_to_end(&mut buf).unwrap();
    drop(f);

    let kol = buf.len() / CHUNK_SZ;
    for i in 0..kol {
        let mut txt_dat = BufWriter::new(File::create(format!("{}.dat", num(i))).unwrap());
        txt_dat.write_all(&buf[i*CHUNK_SZ..i*CHUNK_SZ+2]).unwrap();
        drop(txt_dat);

        let mut txt_txt = BufWriter::new(File::create(format!("{}.txt", num(i))).unwrap());
        for j in 0..8 {
            let base_offset = i*CHUNK_SZ+j*CHUNK_OFFSET+2;
            let str_len = buf[base_offset] as usize;
            let mut raw_str = vec![0; str_len];
            for k in 1..=str_len {
                let idx = base_offset + k;
                buf[idx] -= MAGIC_NUMBERS[j];
                raw_str.push(buf[idx]);
            }
            let unicode_str = iconv::decode(&raw_str, ENCODING).unwrap();
            writeln!(txt_txt, "{}", unicode_str).unwrap();
        }
        drop(txt_txt);
    }
}
