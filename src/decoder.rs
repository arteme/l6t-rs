use std::io::{Read, Cursor, Write};

use crate::{types, iff};
use crate::iff::Chunk;
use crate::types::{TypeID};
use crate::model::*;
use crate::bytecast;
use std::borrow::Borrow;
use std::num::ParseIntError;
use std::io;

pub struct Reader<R: Read> {
    reader: R,
    pub little_endian: bool
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R, little_endian: bool) -> Self {
        Reader { reader, little_endian }
    }

    pub fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut v = [0u8; 4];
        self.reader.read_exact(&mut v)?;
        Ok(match self.little_endian {
            true => u32::from_le_bytes(v),
            false => u32::from_be_bytes(v)
        })
    }

    pub fn read_utf(&mut self, len: usize) -> Result<String, io::Error> {
        let mut vec: Vec<u16> = vec![0; len];
        let mut arr = bytecast::u16_as_ne_mut_bytes(vec.as_mut_slice());
        self.reader.read_exact(&mut arr)?;

        match self.little_endian {
            true => {
                for v in vec.as_mut_slice() {
                    *v = u16::from_le(*v);
                }
            }
            false => {
                for v in vec.as_mut_slice() {
                    *v = u16::from_be(*v);
                }
            }
        }
        Ok(String::from_utf16_lossy(&vec))
    }

    pub fn read_utf_z(&mut self, len: usize) -> Result<String, io::Error> {
        let str = self.read_utf(len)?;
        Ok(str.split('\0').next().unwrap().to_string())
    }
}




//#[derive(Debug)]
pub struct Decoder {
    little_endian: bool
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            little_endian: false
        }
    }
    pub fn read(&mut self, data: &[u8]) -> Result<L6Patch, io::Error> {
        let chunk = iff::Chunk::new(data, None).unwrap();

        if chunk.has_envelope_type(types::FORM, types::L6PA) {
            // L6T patch file
            let mut patch: L6Patch = Default::default();

            for (type_id, chunk) in chunk.all_chunks() {
                match type_id {
                    types::UNFO => { patch.meta = decode_meta_tags(chunk).unwrap(); },
                    types::PINF => { patch.target_device = read_target_device(chunk, self.little_endian)?; },
                    _ => {}
                }
            }
            return Ok(patch);
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "cannot parse file"))
    }

    fn read_u32(&self, data: &[u8]) -> u32 {
        // SAFETY: ok because caller checks the length
        let arr = array_ref![data, 0, 4];
        if self.little_endian {
            u32::from_le_bytes(*arr) as u32
        } else {
            u32::from_be_bytes(*arr) as u32
        }
    }
}

/// Encode a string into a UTF-16 chunk
fn encode_utf(str: &String, type_id: TypeID) -> Option<Chunk> {
    if str.is_empty() { return None }

    let utf16 = str.encode_utf16();
    let mut vec =  Vec::with_capacity(utf16.size_hint().0);
    str.encode_utf16().map(|x| vec.write(&x.to_be_bytes()));

    Some(Chunk::Data {
        id: type_id,
        data: vec,
        little_endian: false
    })
}

fn decode_utf(data: &[u8]) -> String {
    let utf16: Vec<u16> = data.chunks(2).take(data.len() & (usize::MAX-1))
        .map(|c| u16::from_be_bytes(*array_ref![c, 0, 2])).collect();
    String::from_utf16_lossy(&utf16)

}

fn encode_date(date: &usize, type_id: TypeID) -> Option<Chunk> {
    if *date == 0 { return None }

    let str = (*date / 1000).to_string();
    return encode_utf(&str, type_id);
}

fn decode_date(data: &[u8]) -> usize {
    let str = decode_utf(data);
    match str.parse::<usize>() {
        Ok(v) => v * 1000,
        Err(_) => 0
    }
}

/// Encode MetaTags into an unicode info chunk
fn encode_meta_tags(tags: &MetaTags) -> Chunk {
    let mut c = Chunk::create(types::LIST, types::UNFO, false);
    let mut chunks = [
        encode_utf(&tags.author, types::IAUT),
        encode_utf(&tags.guitarist, types::IGTR),
        encode_utf(&tags.band, types::IBND),
        encode_utf(&tags.song, types::ISNG),
        encode_utf(&tags.style, types::ISTL),
        encode_utf(&tags.pickup_style, types::IPUS),
        encode_utf(&tags.pickup_position, types::IPUP),
        encode_date(&tags.date, types::IDAT),
        encode_utf(&tags.amp_name, types::IAMP),
        encode_utf(&tags.creator_app, types::IAPP),
        encode_utf(&tags.creator_app_version, types::IAPV),
        encode_utf(&tags.comments, types::ICMT),
    ];
    for chunk in chunks.iter_mut() {
        chunk.take().map(|chunk| c.append_chunk(chunk));
    }
    c
}

fn decode_meta_tags(chunk: &Chunk) -> Option<MetaTags> {
    if !chunk.has_envelope_type(types::LIST, types::UNFO) { return None }

    let mut tags: MetaTags = Default::default();
    for (type_id, data) in chunk.data_chunks() {
        match type_id {
            types::IAUT => tags.author = decode_utf(data),
            types::IGTR => tags.guitarist = decode_utf(data),
            types::IBND => tags.band = decode_utf(data),
            types::ISNG => tags.song = decode_utf(data),
            types::ISTL => tags.style = decode_utf(data),
            types::IPUS => tags.pickup_style = decode_utf(data),
            types::IPUP => tags.pickup_position = decode_utf(data),
            types::IDAT => tags.date = decode_date(data),
            types::IAMP => tags.amp_name = decode_utf(data),
            types::IAPP => tags.creator_app = decode_utf(data),
            types::IAPV => tags.creator_app_version = decode_utf(data),
            types::ICMT => tags.comments = decode_utf(data),
            _ => {}
        }
    }

    return Some(tags);
}

fn read_target_device(chunk: &Chunk, little_endian: bool) -> Result<TargetDevice, io::Error> {
    let data = match chunk {
        Chunk::Data { data, .. } => data,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data chunk expected"))
    };

    println!("{}", data.len());
    if data.len() != 76 { return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect chunk length")); }

    let mut r = Reader::new(Cursor::new(data), little_endian);
    if r.read_u32()? != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect PINF version"))
    }
    let midi_id = r.read_u32()?;
    let name = r.read_utf_z(32)?;
    let version = r.read_u32()?;
    Ok(TargetDevice { midi_id, name, version })
}

