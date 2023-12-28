use std::io;
use std::io::{Cursor, Write};

use crate::iff::Chunk;
use crate::model::*;
use crate::types;
use crate::types::TypeID;

pub struct Writer<W: Write> {
    writer: W,
    pub little_endian: bool
}

impl <W: Write> Writer<W> {
    pub fn new (writer: W, little_endian: bool) -> Self {
        Self { writer, little_endian }
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), io::Error> {
        let v = [value];
        self.writer.write_all(&v)
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), io::Error> {
        let v = match self.little_endian {
            true => value.to_le_bytes(),
            false => value.to_be_bytes(),
        };
        self.writer.write_all(&v)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), io::Error> {
        let v = match self.little_endian {
            true => value.to_le_bytes(),
            false => value.to_be_bytes()
        };
        self.writer.write_all(&v)
    }

    fn write_utf_pad(&mut self, len: usize, pad: usize, value: &str) -> Result<usize, io::Error> {
        let mut n = 0;

        for c in value.chars() {
            let l = c.len_utf16() * 2; // utf-16 string in bytes
            if n + l > len - pad {
                break;
            }

            let mut b = [0; 2];
            c.encode_utf16(&mut b);
            for c in b.iter().take(l / 2) { // l is in bytes!
                self.write_u16(*c)?;
            }
            n += l;
        }
        while n < len {
            self.write_u16(0u16)?;
            n += 2;
        }

        Ok(n)
    }

    pub fn write_utf(&mut self, len: usize, value: &str) -> Result<usize, io::Error> {
        self.write_utf_pad(len, 0, value)
    }

    pub fn write_utf_z(&mut self, len: usize, value: &str) -> Result<usize, io::Error> {
        self.write_utf_pad(len, 2, value)
    }
}

fn writer_for_slice(slice: &mut [u8], little_endian: bool) -> Writer<Cursor<&mut [u8]>> {
    Writer::new( Cursor::new(slice), little_endian)
}

fn writer_for_vec(vec: &mut Vec<u8>, little_endian: bool) -> Writer<Cursor<&mut Vec<u8>>> {
    Writer::new( Cursor::new(vec), little_endian)
}

pub struct Encoder {}

impl Encoder {

    pub fn write(patch: &L6Patch) -> Result<Vec<u8>, io::Error> {
        Self::write_with_endian(patch, false)
    }

    pub fn write_with_endian(patch: &L6Patch, little_endian: bool) -> Result<Vec<u8>, io::Error> {
        let mut envelope = Chunk::create(types::FORM, types::L6PA, little_endian);

        envelope.append_chunk(write_target_device(&patch.target_device, little_endian)?);
        envelope.append_chunk(write_models(&patch.models, little_endian)?);
        envelope.append_chunk(write_meta_tags(&patch.meta, little_endian)?);

        let mut vec = Vec::new();
        envelope.write(&mut vec)?;
        Ok(vec)
    }
}

fn write_target_device(dev: &TargetDevice, little_endian: bool) -> Result<Chunk, io::Error> {
    let mut data = [0u8; 76];
    let mut w = writer_for_slice(&mut data, little_endian);

    w.write_u32(1)?; // PINF version
    w.write_u32(dev.midi_id)?;
    w.write_utf_z(32, &dev.name)?;
    w.write_u32(dev.version)?;

    Ok(Chunk::Data { id: types::PINF, data: Vec::from(data), little_endian })
}

/// Encode a string into a UTF-16 chunk
fn encode_utf(str: &String, type_id: TypeID, little_endian: bool) -> Option<Chunk> {
    if str.is_empty() { return None }

    let size = str.encode_utf16().count() * 2;
    let mut data: Vec<u8> = Vec::with_capacity(size);
    let mut w = writer_for_vec(&mut data, little_endian);
    w.write_utf(size, str).unwrap();

    Some(Chunk::Data { id: type_id, data, little_endian })
}

fn encode_date(date: &usize, type_id: TypeID, little_endian: bool) -> Option<Chunk> {
    if *date == 0 { return None }

    let str = (*date / 1000).to_string();
    encode_utf(&str, type_id, little_endian)
}

fn encode_value(value: &Value) -> Result<[u32;2], io::Error> {
    match value {
        Value::Int(v) => {
            Ok([0, *v])
        }
        Value::Float(v) => {
            Ok([1, v.to_bits()])
        }
    }
}

fn write_meta_tags(tags: &MetaTags, little_endian: bool) -> Result<Chunk, io::Error> {
    let mut envelope = Chunk::create(types::LIST, types::UNFO, little_endian);
    let chunks = [
        encode_utf(&tags.author, types::IAUT, little_endian),
        encode_utf(&tags.guitarist, types::IGTR, little_endian),
        encode_utf(&tags.band, types::IBND, little_endian),
        encode_utf(&tags.song, types::ISNG, little_endian),
        encode_utf(&tags.style, types::ISTL, little_endian),
        encode_utf(&tags.pickup_style, types::IPUS, little_endian),
        encode_utf(&tags.pickup_position, types::IPUP, little_endian),
        encode_date(&tags.date, types::IDAT, little_endian),
        encode_utf(&tags.amp_name, types::IAMP, little_endian),
        encode_utf(&tags.creator_app, types::IAPP, little_endian),
        encode_utf(&tags.creator_app_version, types::IAPV, little_endian),
        encode_utf(&tags.comments, types::ICMT, little_endian),
    ];

    for mut chunk in chunks {
        let Some(chunk) = chunk else { continue };
        envelope.append_chunk(chunk);
    }

    Ok(envelope)
}

fn write_models(models: &[Model], little_endian: bool) -> Result<Chunk, io::Error> {
    let mut envelope = Chunk::create(types::LIST, types::PATC, little_endian);
    for model in models {
        envelope.append_chunk(write_model(model, little_endian)?);
    }

    Ok(envelope)
}

fn write_model(model: &Model, little_endian: bool) -> Result<Chunk, io::Error> {
    let mut envelope = Chunk::create(types::LIST, types::MODL, little_endian);

    envelope.append_chunk(write_model_info(&model, little_endian)?);
    for param in &model.params {
        envelope.append_chunk(write_model_param(&param, little_endian)?)
    }

    Ok(envelope)
}

fn write_model_info(model: &Model, little_endian: bool) -> Result<Chunk, io::Error> {
    let mut data = [0u8; 12];
    let mut w = writer_for_slice(&mut data, little_endian);

    w.write_u32(model.model_id)?;
    w.write_u32(model.slot_id)?;
    w.write_u8(model.ordinal)?;;
    w.write_u8(0)?;
    w.write_u8(0)?;
    w.write_u8(if model.enabled { 1 } else { 0 })?;;

    Ok(Chunk::Data { id: types::MINF, data: Vec::from(data), little_endian })
}

fn write_model_param(param: &ModelParam, little_endian: bool) -> Result<Chunk, io::Error> {
    let mut data = [0u8; 12];
    let mut w = writer_for_slice(&mut data, little_endian);

    w.write_u32(0x3f000000 | (param.param_id & 0x00ffffff))?;

    let value = encode_value(&param.value)?;
    w.write_u32(value[0])?;
    w.write_u32(value[1])?;

    Ok(Chunk::Data { id: types::PARM, data: Vec::from(data), little_endian })
}

#[cfg(test)]
mod test {
    use crate::encoder::{writer_for_vec};

    #[test]
    fn test_write_utf() {
        let str = "Hello";
        let expected = &[0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x6f];

        let mut vec = Vec::with_capacity(10);
        let mut w = writer_for_vec(&mut vec, false);

        w.write_utf(10, str).unwrap();

        // NUL-terminated

        let expected = &[0x00, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x00];

        let mut vec = Vec::with_capacity(10);
        let mut w = writer_for_vec(&mut vec, false);

        w.write_utf_z(10, str).unwrap();

        assert_eq!(&vec, expected);
    }
}