use std::io;
use std::io::{Read, Cursor};

use crate::types;
use crate::iff::Chunk;
use crate::model::*;
use crate::bytecast;
use crate::error::Error;
use crate::hex::PrintHex;

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

pub enum DecoderResult {
    Patch(L6Patch),
    Bundle(L6Bundle),
}

impl Decoder {
    pub fn read(data: &[u8]) -> Result<DecoderResult, Error> {
        let chunk = Chunk::from_data(data, None)?;

        let readers = vec![
            read_l6patch,
            read_l6bundle,
            read_sounddiver_lib
        ];
        for reader in readers {
            match reader(&chunk, data) {
                Err(Error::FormatNotSupported()) => continue,
                res @ _ => return res
            }
        }

        Err(Error::FormatNotSupported())
    }
}

fn read_l6patch(chunk: &Chunk, _data: &[u8]) -> Result<DecoderResult, Error> {
    if !chunk.has_envelope_type(types::FORM, types::L6PA) &&
        !chunk.has_envelope_type(types::FORM, types::L6AS) &&
        !chunk.has_envelope_type(types::FORM, types::L6FS) {
        return Err(Error::FormatNotSupported());
    }

    // L6T patch file
    let mut patch: L6Patch = Default::default();
    patch.patch_type = match chunk.id() {
        types::L6PA => PatchType::Patch,
        types::L6AS => PatchType::AmpSetup,
        types::L6FS => PatchType::FxSetup,
        _ => unreachable!()
    };
    let little_endian = false;

    for (type_id, chunk) in chunk.all_chunks() {
        match type_id {
            types::PATC => { patch.models = read_models(chunk)?; },
            types::UNFO => { patch.meta = read_meta_tags(chunk)?; },
            types::PINF => { patch.target_device = read_target_device(chunk, little_endian)?; },
            _ => {}
        }
    }
    Ok(DecoderResult::Patch(patch))
}

fn read_l6bundle(chunk: &Chunk, _data: &[u8]) -> Result<DecoderResult, Error> {
    if !chunk.has_envelope_type(types::FORM, types::L6BA) &&
        !chunk.has_envelope_type(types::FORM, types::L6CO) {
        return Err(Error::FormatNotSupported());
    }

    // L6B, L6C file
    let bundle_type = match chunk.id() {
        types::L6BA => BundleType::Bundle,
        types::L6CO => BundleType::Collection,
        _ => unreachable!()
    };

    let mut banks = vec![];
    for (type_id, chunk) in chunk.all_chunks() {
        match type_id {
            types::HEAD => {
                // not actually using head chunk for anything
                read_head(chunk, false)?;
            },
            types::BANK if bundle_type == BundleType::Bundle => {
                let bank = read_bank(chunk, false)?;
                banks.push(bank);
            }
            types::FLDR if bundle_type == BundleType::Collection => {
                let bank = read_bank(chunk, false)?;
                banks.push(bank);
            }
            //types::UNFO => { patch.meta = read_meta_tags(chunk)?; },
            //types::PINF => { patch.target_device = read_target_device(chunk, little_endian)?; },
            _ => {}
        }
    }

    let batch = L6Bundle { bundle_type, banks };
    Ok(DecoderResult::Bundle(batch))
}

fn read_sounddiver_lib(chunk: &Chunk, data: &[u8]) -> Result<DecoderResult, Error> {
    if !chunk.has_envelope_type(types::FORM, types::SSLB) {
        return Err(Error::FormatNotSupported());
    }

    // SoundDiver lib

    // sounddiver sometimes places data outsize the FORM/SSLB container
    let correct_chunk;
    let chunk = if chunk.all_chunks().is_empty() && data.len() > 12 {
        correct_chunk = Chunk::from_data_with_size(data, data.len() - 8, None).unwrap();
        &correct_chunk
    } else {
        chunk
    };

    for (type_id, chunk) in chunk.all_chunks() {
        match type_id {
            types::LENT => { read_sslb_entry(chunk)?; },
            //types::LHDR | types::WSEQ => {}, // ignore
            _ => {}
        }
    }

    todo!()
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
        1 => Ok(Value::Float(f32::from_bits(data[1]))),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported value type {:#x}", data[0]))
        )
    }
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

fn read_head(chunk: &Chunk, little_endian: bool) -> Result<BatchHead, io::Error> {
    let data = match chunk {
        Chunk::Data { data, .. } => data,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data chunk expected"))
    };

    let mut r = reader_for_slice(data, little_endian);
    let version = r.read_u32()?;

    Ok(BatchHead { version })
}

fn read_bank(chunk: &Chunk, little_endian: bool) -> Result<Bank, Error> {
    let mut bank = Bank::default();
    for (type_id, chunk) in chunk.all_chunks() {
        match type_id {
            types::BINF => {
                let bank_info = read_bank_info(chunk, little_endian)?;
                bank.name = bank_info.name;
            }
            types::L6PA | types::L6AS | types::L6FS => {
                let res = read_l6patch(chunk, &[0u8])?;
                let patch = match res {
                    DecoderResult::Patch(patch) => { patch }
                    _ => {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "incorrect decoder output").into());
                    }
                };
                bank.patches.push(patch);

            }
            _ => {
                println!("{:?}", type_id);
                return Err(io::Error::new(io::ErrorKind::InvalidData, "chunk not supported").into());
            }
        }
    }

    Ok(bank)
}

fn read_bank_info(chunk: &Chunk, little_endian: bool) -> Result<BankInfo, io::Error> {
    let data = match chunk {
        Chunk::Data { data, .. } => data,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data chunk expected"))
    };

    if data.len() != 68 { return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect chunk length")); }

    let mut r = reader_for_slice(data, little_endian);
    r.read_u32()?;
    let name = r.read_utf(32)?;

    Ok(BankInfo { name })
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

fn read_models(chunk: &Chunk) -> Result<Vec<Model>, io::Error> {
    let chunks = match chunk {
        Chunk::Envelope { ref chunks, .. } => chunks,
        Chunk::Data { .. } => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Container chunk expected"))
    };

    chunks.iter().map(|chunk| {
        if chunk.has_envelope_type(types::LIST, types::MODL) {
            read_model(chunk)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Incorrect envelope type"))
        }
    }).collect()
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
    r.read_u8()?; // this should be 0x03

    // This must be the POD model name if using UNI module
    // Skip everything except the last 55 bytes of actual data
    let n = r.read_u8()? - 55;
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

