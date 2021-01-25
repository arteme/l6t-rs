use std::io::{Read, Cursor, Write};

use crate::{types, iff};
use crate::iff::Chunk;
use crate::types::{TypeID};
use crate::model::*;
use crate::bytecast;
use crate::hex::PrintHex;
use std::io;
use std::collections::HashMap;

pub struct Reader<R: Read> {
    reader: R,
    pub little_endian: bool
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R, little_endian: bool) -> Self {
        Reader { reader, little_endian }
    }

    pub fn read_u8(&mut self) -> Result<u8, io::Error> {
        let mut v = [0u8; 1];
        self.reader.read_exact(&mut v)?;
        Ok(v[0])
    }

    pub fn read_u8_into(&mut self, buffer: &mut [u8]) -> Result<(), io::Error> {
       self.reader.read_exact(buffer)?;
        Ok(())
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
        let mut vec: Vec<u16> = vec![0; len / 2];
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

    pub fn read_ascii_n(&mut self) -> Result<String, io::Error> {
        let len = self.read_u8()? as usize;
        let mut buffer = vec![0u8; len];
        self.read_u8_into(&mut buffer)?;
        // it's okay, we're dealing with ascii here
        let str = unsafe { String::from_utf8_unchecked(buffer) };
        Ok(str)
    }
}

fn reader_for_slice(slice: &[u8], little_endian: bool) -> Reader<Cursor<&[u8]>> {
    Reader::new(Cursor::new(slice), little_endian)
}

pub struct Decoder {}

impl Decoder {
    pub fn read(data: &[u8]) -> Result<L6Patch, io::Error> {
        let mut chunk = iff::Chunk::new(data, None).unwrap();

        if chunk.has_envelope_type(types::FORM, types::L6PA) {
            // L6T patch file
            let mut patch: L6Patch = Default::default();
            let little_endian = false;

            for (type_id, chunk) in chunk.all_chunks() {
                match type_id {
                    types::PATC => { patch.models = read_models(chunk)?; },
                    types::UNFO => { patch.meta = read_meta_tags(chunk)?; },
                    types::PINF => { patch.target_device = read_target_device(chunk, little_endian)?; },
                    _ => {}
                }
            }
            return Ok(patch);
        } else if chunk.has_envelope_type(types::FORM, types::SSLB) {
            // SoundDiver lib

            // sounddiver sometimes places data outsize the FORM/SSLB container
            if chunk.all_chunks().is_empty() && data.len() > 12 {
                chunk = iff::Chunk::with_size_override(data, data.len() - 8, None).unwrap()
            }

            for (type_id, chunk) in chunk.all_chunks() {
                match type_id {
                    types::LENT => { read_sslb_entry(chunk)?; },
                    //types::LHDR | types::WSEQ => {}, // ignore
                    _ => {}
                }
            }
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "ok"))
        }

        Err(io::Error::new(io::ErrorKind::InvalidInput, "cannot parse file"))
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

fn encode_date(date: &usize, type_id: TypeID) -> Option<Chunk> {
    if *date == 0 { return None }

    let str = (*date / 1000).to_string();
    return encode_utf(&str, type_id);
}

fn decode_date(str: &str) -> usize {
    match str.parse::<usize>() {
        Ok(v) => v * 1000,
        Err(_) => 0
    }
}

fn decode_value(data: &[u32;2]) -> Result<Value, io::Error> {
    match data[0] {
        0 => Ok(Value::Int(data[1])),
        1 => {
            let f: f32 = unsafe { std::mem::transmute_copy(&data[1]) };
            Ok(Value::Float(f))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported value type {:#x}", data[0]))
        )
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

fn read_meta_tags(chunk: &Chunk) -> Result<MetaTags, io::Error> {
    let mut tags: MetaTags = Default::default();
    for (type_id, data) in chunk.data_chunks() {
        let mut r = reader_for_slice(data, chunk.is_little_endian());
        match type_id {
            types::IAUT => tags.author = r.read_utf(data.len())?,
            types::IGTR => tags.guitarist = r.read_utf(data.len())?,
            types::IBND => tags.band = r.read_utf(data.len())?,
            types::ISNG => tags.song = r.read_utf(data.len())?,
            types::ISTL => tags.style = r.read_utf(data.len())?,
            types::IPUS => tags.pickup_style = r.read_utf(data.len())?,
            types::IPUP => tags.pickup_position = r.read_utf(data.len())?,
            types::IDAT => tags.date = decode_date(&r.read_utf(data.len())?),
            types::IAMP => tags.amp_name = r.read_utf(data.len())?,
            types::IAPP => tags.creator_app = r.read_utf(data.len())?,
            types::IAPV => tags.creator_app_version = r.read_utf(data.len())?,
            types::ICMT => tags.comments = r.read_utf(data.len())?,
            _ => {}
        }
    }

    Ok(tags)
}

fn read_target_device(chunk: &Chunk, little_endian: bool) -> Result<TargetDevice, io::Error> {
    let data = match chunk {
        Chunk::Data { data, .. } => data,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data chunk expected"))
    };

    if data.len() != 76 { return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect chunk length")); }

    let mut r = reader_for_slice(data, little_endian);
    if r.read_u32()? != 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect PINF version"))
    }
    let midi_id = r.read_u32()?;
    let name = r.read_utf_z(32)?;
    let version = r.read_u32()?;
    Ok(TargetDevice { midi_id, name, version })
}

fn read_models(chunk: &Chunk) -> Result<HashMap<u32, Model>, io::Error> {
    let chunks = match chunk {
        Chunk::Envelope { ref chunks, .. } => chunks,
        Chunk::Data { .. } => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Container chunk expected"))
    };

    let mut models: HashMap<u32, Model> = HashMap::new();

    for chunk in chunks {
        if chunk.has_envelope_type(types::LIST, types::MODL) {
            let model = read_model(chunk)?;
            models.insert(model.slot_id, model);
        }
    }

    Ok(models)
}


fn read_model(chunk: &Chunk) -> Result<Model, io::Error> {
    let mut model: Model = Default::default();
    let mut params: Vec<ModelParam> = vec![];
    let little_endian= chunk.is_little_endian();

    for (type_id, chunk) in chunk.data_chunks() {
        match type_id {
            types::MINF => model = read_model_info(chunk, little_endian)?,
            types::PARM => params.push(read_model_param(chunk, little_endian)?),
            _ => {}
        }
    }
    model.params.extend(params);

    Ok(model)
}

fn read_model_info(data: &[u8], little_endian: bool) -> Result<Model, io::Error> {
    let mut r = reader_for_slice(data, little_endian);
    let mut model: Model = Default::default();

    model.model_id = r.read_u32()?;
    model.slot_id = r.read_u32()?;
    model.ordinal = r.read_u8()?;
    r.read_u8()?;
    r.read_u8()?;
    model.enabled = r.read_u8()? > 0;

    Ok(model)
}
fn read_model_param(data: &[u8], little_endian: bool) -> Result<ModelParam, io::Error> {
    let mut r = reader_for_slice(data, little_endian);
    let mut param: ModelParam = Default::default();

    param.param_id = r.read_u32()? & 0x00ffffff;
    let data = [r.read_u32()?, r.read_u32()?];
    param.value = decode_value(&data)?;

    Ok(param)
}

fn read_sslb_entry(chunk: &Chunk) -> Result<(), io::Error> {
    let little_endian = chunk.is_little_endian();
    let data = match chunk {
        Chunk::Data { data, .. } => data,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data chunk expected"))
    };
    if data.len() < 13 {
        // Empty entry?
        return Ok(());
    }
    let mut r = reader_for_slice(data, little_endian);
    let mut header = [0u8; 13];
    r.read_u8_into(&mut header)?;

    // 2a = POD, ff = universal
    let can_process = header[1] == 0x2a || (header[0..2] == [0x80, 0xff]);
    let name = r.read_ascii_n()?;
    if !can_process {
        // Not a POD program, stop here
        return Ok(());
    }
    let z = r.read_u8()?; // this should be 0x03
    let n = 55 - r.read_u8()?;
    if n > 0 {
        let mut bytes = vec![0u8; n as usize];
        r.read_u8_into(&mut bytes)?;
    }
    let mut bytes = vec![0u8; 55];
    r.read_u8_into(&mut bytes)?;

    loop {
        let id = r.read_u8()?;
        match id {
            0x02 => {
                let str = r.read_ascii_n()?;
                println!("comment: {}", str);
            },
            0x06 => {
                let str = r.read_ascii_n()?;
                println!("position: {}", str);
            },
            0x00 => {
                let mut skip = vec![0u8; 2];
                r.read_u8_into(&mut skip)?;
                println!("end:");
                skip.print_hex();
                break;
            }
            _ => {
                println!("unknown: {:#02x}", id);
                break;
            }
        }
    }

    Ok(())
}

