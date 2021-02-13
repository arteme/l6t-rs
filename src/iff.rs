/// IFF reader/writer adapted from https://github.com/qpliu/iff-rs with
/// Line6-specific quirks such as little-endian files and unaligned data
/// chunks.
///
use std::io::{Error,ErrorKind,Result};

use crate::types::{TypeID, UNALIGNED_CHUNKS, SSLB};

pub enum Chunk {
    Envelope {
        envelope_id: TypeID,
        id: TypeID,
        chunks: Vec<Chunk>,
        little_endian: bool,
        aligned: bool
    },
    Data {
        id: TypeID,
        data: Vec<u8>,
        little_endian: bool
    },
}
impl Chunk {
    pub fn new(data: &[u8], little_endian: Option<bool>) -> Result<Self> {
        Chunk::new_chunk(data, 0, data.len(), little_endian, None)
    }

    pub fn with_size_override(data: &[u8], size_override: usize, little_endian: Option<bool>) -> Result<Self> {
        Chunk::new_chunk(data, 0, data.len(), little_endian, Some(size_override))
    }

    pub fn is_little_endian(&self) -> bool {
        match self {
            Chunk::Envelope { little_endian, .. } => *little_endian,
            Chunk::Data { little_endian, .. } => *little_endian
        }
    }

    fn new_chunk(data: &[u8], index: usize, last_index: usize, little_endian: Option<bool>,
                 size_override: Option<usize>) -> Result<Self> {
        if index + 8 > last_index {
            return Err(Error::new(ErrorKind::InvalidData, "invalid data"));
        }
        let mut id = Self::chunk_id(&data, index, little_endian.unwrap_or(false));
        let little_endian = if little_endian.is_some() {
            little_endian.unwrap()
        } else {
            if id.is_envelope() {
                false
            } else if id.is_le_envelope() {
                id = id.reverse(); // we didn't know this was little-endian when we read it
                true
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "data chunk needs 'little_endian' setting"));
            }
        };

        let mut size = size_override.unwrap_or_else(
            || Self::chunk_size(&data, index+4, little_endian));
        //println!("chunk '{}' len {} at {} env {}", id, size, index, id.is_envelope());
        if index + 8 + size > last_index {
            return Err(Error::new(ErrorKind::InvalidData, "invalid data"));
        }
        if id.is_envelope() {
            if size < 4 {
                return Err(Error::new(ErrorKind::InvalidData, "invalid data"));
            }
            let data_id = Self::chunk_id(&data, index+8, little_endian);
            let mut i = index + 12;

            let aligned = !UNALIGNED_CHUNKS.contains(&&data_id);
            if &data_id == &SSLB && size == 4 {
                // Sounddiver sometimes places data outsize the FORM/SSLB container
                // and doesn't update the size ;(
                size = data.len() - 8;
            }

            let mut chunks = Vec::new();
            while i < index + 8 + size {
                let chunk = Self::new_chunk(&data, i, index+8+size, Some(little_endian), None)?;
                i += chunk.size();
                if aligned && i % 2 != 0 {
                    i += 1;
                }
                chunks.push(chunk);
            }
            Ok(Chunk::Envelope{ envelope_id: id, id: data_id, chunks, little_endian, aligned })
        } else {
            Ok(Chunk::Data{ id, data: data[index+8..index+8+size].to_vec(), little_endian })
        }
    }

    fn chunk_id(data: &[u8], index: usize, little_endian: bool) -> TypeID {
        if little_endian {
            TypeID([data[index+3],data[index+2],data[index+1],data[index]])
        } else {
            TypeID([data[index],data[index+1],data[index+2],data[index+3]])
        }
    }

    fn chunk_size(data: &[u8], index: usize, little_endian: bool) -> usize {
        // SAFETY: ok because caller checks the length
        let ptr = data[index..].as_ptr() as *const [u8; 4];
        let arr = unsafe { &*ptr };
        if little_endian {
            u32::from_le_bytes(*arr) as usize
        } else {
            u32::from_be_bytes(*arr) as usize
        }
    }

    pub fn create(envelope_id: TypeID, id: TypeID, little_endian: bool) -> Self {
        let aligned = UNALIGNED_CHUNKS.contains(&&id);
        Chunk::Envelope{ envelope_id, id, chunks: Vec::new(), little_endian, aligned }
    }

    /*
    pub fn append_data(&mut self, id: TypeID, data: &[u8]) {
        if let &mut Chunk::Envelope{ ref mut chunks, little_endian, .. } = self {
            chunks.push(Chunk::Data{ id, data, little_endian });
        } else {
            panic!("Cannot add nested chunks to a data chunk");
        }
    }
     */

    pub fn append_chunk(&mut self, chunk: Chunk) {
        if let &mut Chunk::Envelope{ ref mut chunks, .. } = self {
            chunks.push(chunk);
        } else {
            panic!("Cannot add nested chunks to a data chunk");
        }
    }

    /*
    pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        match self {
            &Chunk::Envelope{ envelope_id, id, ref chunks } => {
                w.write_all(&envelope_id.0[..])?;
                let size = self.size() - 8;
                w.write_all(&[(size >> 24) as u8, (size >> 16) as u8, (size >> 8) as u8, size as u8])?;
                w.write_all(&id.0[..])?;
                for chunk in chunks {
                    chunk.write(w)?;
                }
                if size % 2 != 0 {
                    w.write_all(&[0u8])?;
                }
            },
            &Chunk::Data{ id, data } => {
                w.write_all(&id.0[..])?;
                let size = self.size() - 8;
                w.write_all(&[(size >> 24) as u8, (size >> 16) as u8, (size >> 8) as u8, size as u8])?;
                w.write_all(data)?;
                if size % 2 != 0 {
                    w.write_all(&[0u8])?;
                }
            }
        }
        Ok(())
    }

     */
    fn size(&self) -> usize {
        match self {
            &Chunk::Envelope{ ref chunks, aligned, .. } => {
                let mut size = 12;
                for chunk in chunks {
                    size += chunk.size();
                    if aligned && size % 2 != 0 {
                        size += 1;
                    }
                }
                size
            },
            &Chunk::Data{ id:_, ref data, little_endian:_ } => 8 + data.len(),
        }
    }

    pub fn has_envelope_type(&self, envelope_type_id: TypeID, type_id: TypeID) -> bool {
        match self {
            &Chunk::Envelope{ envelope_id, id, .. } =>
                envelope_type_id == envelope_id && type_id == id,
            _ => false,
        }
    }

    pub fn has_data_type(&self, type_id: TypeID) -> bool {
        match self {
            &Chunk::Data{ id, .. } => type_id == id,
            _ => false,
        }
    }


    pub fn data_chunks(&self) -> Vec<(TypeID,&Vec<u8>)> {
        let mut vec = Vec::new();
        match self {
            &Chunk::Envelope{ ref chunks, .. } => {
                for chunk in chunks {
                    match chunk {
                        &Chunk::Data{ id, ref data, little_endian:_ } => vec.push((id, data)),
                        _ => (),
                    }
                }
            },
            _ => (),
        }
        vec
    }

    pub fn all_chunks(&self) -> Vec<(TypeID,&Chunk)> {
        let mut vec = Vec::new();
        match self {
            &Chunk::Envelope{ ref chunks, .. } => {
                for chunk in chunks {
                    match chunk {
                        &Chunk::Data{ id, .. } => vec.push((id, chunk)),
                        &Chunk::Envelope{ id, .. } => vec.push((id, chunk)),
                    }
                }
            },
            _ => (),
        }
        vec
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Chunk::Envelope { envelope_id, id, chunks, little_endian, aligned } => {
                f.debug_struct("Chunk::Envelope")
                    .field("envelope", &envelope_id)
                    .field("id", &id)
                    .field("chunks.len", &chunks.len())
                    .field("little_endian", &little_endian)
                    .field("aligned", &aligned)
                    .finish()
            }
            Chunk::Data { id, data, little_endian } => {
                f.debug_struct("Chunk::Data")
                    .field("id", &id)
                    .field("data.len", &data.len())
                    .field("little_endian", &little_endian)
                    .finish()
            }
        }
    }
}
