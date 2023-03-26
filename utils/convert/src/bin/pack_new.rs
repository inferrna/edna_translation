use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use convert::{CHUNK_OFFSET, CHUNK_SZ, MAGIC_NUMBERS, num};

const BUF_SIZE: usize = 260000;

fn main() -> std::io::Result<()> {
    // Open the output file for writing
    let mut output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("RAVEN_REASSEMBLED.DAT")?;

    // Iterate over each chunk file and reassemble its contents
    let mut i = 0;
    loop {
        let dat_fnm = format!("{}.dat", num(i));
        let txt_fnm = format!("{}.txt", num(i));
        let dat_file_path = Path::new(&dat_fnm);
        let txt_file_path = Path::new(&txt_fnm);

        if !dat_file_path.exists() || !txt_file_path.exists() {
            // We've reached the end of the chunk files
            break;
        }

        // Read in the contents of the dat file
        let mut dat_file = BufReader::new(File::open(dat_file_path)?);
        let mut dat_buf = [0; 2];
        dat_file.read_exact(&mut dat_buf)?;

        // Read in the contents of the txt file
        let mut txt_file = BufReader::new(File::open(txt_file_path)?);
        let mut txt_buf = vec![0; CHUNK_SZ - 2];
        txt_file.read_to_end(&mut txt_buf)?;

        // Reassemble the chunk and write it to the output file
        let mut chunk_buf = [0; CHUNK_SZ];
        chunk_buf[..2].copy_from_slice(&dat_buf);
        for j in 0..8 {
            let base_offset = j * CHUNK_OFFSET + 2;
            let str_len = txt_buf[base_offset - 2];
            for k in 0..str_len {
                let idx = base_offset + k as usize;
                txt_buf[idx] += MAGIC_NUMBERS[j];
            }
            chunk_buf[base_offset..base_offset + str_len as usize].copy_from_slice(
                &txt_buf[base_offset - 2..base_offset - 2 + str_len as usize],
            );
        }
        output_file.write_all(&chunk_buf)?;

        i += 1;
    }

    Ok(())
}