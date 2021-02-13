use std::fs::File;
use std::io::Read;
use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use l6t::model;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        std::process::exit(-1);
    }

    let mut v: Vec<u8> = Vec::new();
    File::open(args[1].as_str()).unwrap().read_to_end(&mut v).unwrap();

    if &v[2..2+4] == &[0x2d, 0x6c, 0x7a, 0x35] { // "-lz5"
        println!("Compressed file, bailing out");
        std::process::exit(-1);
    }

    let chunk = Chunk::new(v.as_slice(), None).unwrap();
    PrettyPrinter::println(&chunk).unwrap();

    let patch = Decoder::read(v.as_slice());
    match patch {
        Ok(p) => PrettyPrinter::println(&p).unwrap(),
        Err(e) => println!("ERROR: {}", e)
    }
}

struct PrettyPrinter {
    pub indent: usize,
    pub step: usize,
    pub buffer: String
}

impl PrettyPrinter {
    fn new() -> Self {
        PrettyPrinter { indent: 0, step: 2, buffer: "".into() }
    }
    fn indent(&mut self) {
        let indent = format!("{empty:width$}", empty="", width=(self.indent * self.step));
        self.buffer += indent.as_str()
    }
    fn nl(&mut self) {
        self.buffer += &"\n";
    }

    fn println<T: Pretty>(obj: &T) -> fmt::Result {
        let mut pp = PrettyPrinter::new();
        Pretty::fmt(obj, &mut pp)?;
        println!("{}", pp.buffer);
        Ok(())
    }
}

impl fmt::Write for PrettyPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.buffer.ends_with('\n') { self.indent() }
        for (i, slice) in s.split_terminator('\n').enumerate() {
            if i != 0 {
                self.nl();
                self.indent();
            }
            self.buffer += slice
        }
        if s.ends_with('\n') { self.nl() }
        Ok(())
    }
}

trait Pretty {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result;
}

impl Pretty for Chunk {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        write!(pp, "{:?}", self)?;
        match self {
            Chunk::Envelope { chunks, .. } => {
                writeln!(pp, " {{")?;
                pp.indent += 1;
                for c in chunks {
                    Pretty::fmt(c, pp)?;
                }
                pp.indent -= 1;
                writeln!(pp, "}}")?;
            }
            _ => writeln!(pp)?
        }
        Ok(())
    }
}

impl Pretty for model::L6Patch {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "File type: L6T")?;
        writeln!(pp)?;
        Pretty::fmt(&self.meta, pp)?;
        Pretty::fmt(&self.target_device, pp)?;
        Pretty::fmt(&self.models, pp)?;
        Ok(())
    }
}

impl Pretty for model::MetaTags {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {

        let str_field = |name: &str, field: &str, pp: &mut PrettyPrinter| {
            if !field.is_empty() {
                writeln!(pp, "{:16}: {}", name, field)
            } else {
                Ok(())
            }
        };
        let date_field = |name: &str, field: &usize, pp: &mut PrettyPrinter| {
            if *field != 0 {
                writeln!(pp, "{:16}: {}", name, field)
            } else {
                Ok(())
            }
        };

        writeln!(pp, "Info:")?;
        pp.indent += 1;
        str_field("author", &self.author, pp)?;
        str_field("guitarist", &self.guitarist, pp)?;
        str_field("band", &self.band, pp)?;
        str_field("song", &self.song, pp)?;
        str_field("style", &self.style, pp)?;
        str_field("pickup style", &self.pickup_style, pp)?;
        str_field("pickup position", &self.pickup_position, pp)?;
        date_field("date", &self.date, pp)?;
        str_field("amp name", &self.amp_name, pp)?;
        str_field("creator app", &self.creator_app, pp)?;
        str_field("creator app ver", &self.creator_app_version, pp)?;
        str_field("comments", &self.comments, pp)?;
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::TargetDevice {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "Device:")?;
        pp.indent += 1;
        writeln!(pp, "{:16}: {:#08x}", "id", self.midi_id)?;
        writeln!(pp, "{:16}: {}", "name", self.name)?;
        writeln!(pp, "{:16}: {}", "version", self.version)?;
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for HashMap<u32, model::Model> {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "Model:")?;
        pp.indent += 1;
        for v in self.values() {
            Pretty::fmt(v, pp)?;
        }
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::Model {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "[{:#04x}] model={:#08x} ordinal={} {}",
                 self.slot_id, self.model_id, self.ordinal,
                 if self.enabled { "enabled" } else { "disabled" } )?;
        pp.indent += 1;
        for param in self.params.iter() {
            Pretty::fmt(param, pp)?
        }
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::ModelParam {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "id={:#08x} {}", self.param_id, self.value)
    }
}

