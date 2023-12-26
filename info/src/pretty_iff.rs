use std::fmt;
use std::fmt::Write;
use l6t::iff::Chunk;
use crate::pretty::{Pretty, PrettyPrinter};

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

