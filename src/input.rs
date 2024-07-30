// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::File,
    io::{self, Cursor, Read, Stdin},
};

#[derive(Debug)]
pub enum Input {
    String(Cursor<String>),
    File(File),
    Stdin(Stdin),
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Self::String(ref mut string) => string.read(buf),
            Self::File(ref mut file) => file.read(buf),
            Self::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}
