#[cfg(test)]
use imagesize::blob_size;
use std::{
    fs::{metadata, read_dir, File},
    io::{Error, Read},
    path::PathBuf,
};

#[test]
fn fuzzer_crashes_fixed() {
    let mut entries = read_dir("tests/fuzz_crashes")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()
        .unwrap();

    entries.sort();

    for entry in entries {
        let data = get_file_as_byte_vec(entry);
        let _ = blob_size(&data);
    }
}

fn get_file_as_byte_vec(filename: PathBuf) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).expect("buffer overflow");

    buffer
}
