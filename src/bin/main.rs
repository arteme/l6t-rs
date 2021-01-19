use std::fs::File;
use std::io::Read;
use std::fmt;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;

fn dump_chunk(c: &Chunk, indent: usize) {
    print!("{empty:width$}{c:?}", empty="", width=indent, c=c);
    match c {
        Chunk::Envelope { chunks, .. } => {
            println!(" {{");
            for c in chunks {
                dump_chunk(c, indent + 2);
            }
            println!("{empty:width$}}}", empty="", width=indent);
        }
        _ => println!()
    };
}

fn main() {
    let mut f = File::open("mal_g1.lib").unwrap();
    let mut f = File::open("Master clean.l6t").unwrap();
    let mut v: Vec<u8> = Vec::new();
    let len = f.read_to_end(&mut v).unwrap();

    let chunk = Chunk::new(v.as_slice(), None).unwrap();
    dump_chunk(&chunk, 0);

    let patch = Decoder::read(v.as_slice());
    println!("{:?}", patch);
}
