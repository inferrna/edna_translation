use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use convert::{CHUNK_OFFSET, CHUNK_SZ, ENCODING, MAGIC_NUMBERS, num};


fn main() {

    let src_path =  std::env::args()
        .skip(1)
        .last()
        .expect("Provide source directory with txt and dat files, please");

    let mut kol: usize = 0;
    while PathBuf::from(format!("{src_path}/{}.dat", num(kol))).exists() {
        kol += 1;
    }

    let mut buf = vec![0u8; kol * CHUNK_SZ];
    let mut twobvec = vec![0;2];
    for i in 0..kol {
        let mut txt_dat = BufReader::new(File::open(format!("{src_path}/{}.dat", num(i))).unwrap());
        txt_dat.read_exact(&mut twobvec).unwrap();
        buf[i*CHUNK_SZ] = twobvec[0];
        buf[i*CHUNK_SZ+1] = twobvec[1];
        drop(txt_dat);

        let mut txt_txt = BufReader::new(File::open(format!("{src_path}/{}.txt", num(i))).unwrap());
        for j in 0..8 {
            let mut line = String::new();
            txt_txt.read_line(&mut line).unwrap();
            let line = line.trim().trim_matches('\0');
            let cp852bytes = iconv::encode(line, ENCODING).unwrap();
            if i==537 {
                dbg!(line);
                dbg!(&cp852bytes);
            }
            let len = line.len();
            let base_offset = i*CHUNK_SZ+j*CHUNK_OFFSET+2;
            buf[base_offset] = len as u8;
            for (k, byte) in cp852bytes.iter().enumerate() {
                buf[base_offset + k + 1] = byte.wrapping_add(MAGIC_NUMBERS[j]);
            }
        }
        drop(txt_txt);
    }

    let mut f = BufWriter::new(OpenOptions::new().write(true).create(true).open("RAVEN_NEW.DAT").unwrap());
    f.write_all(&buf).unwrap();
    drop(f);
}

