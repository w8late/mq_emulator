use std::{fs::File, io, io::prelude::*};

pub struct ROMLoader {
    pub(super) bytes: [u8; 4096],
    pub(super) len: usize,
}

impl ROMLoader {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut bytes = [0; 4096];
        let mut file= File::open(filename)?;
        let len = file.read(&mut bytes)?;

        Ok(Self {
            bytes,
            len,
        })
    }
}