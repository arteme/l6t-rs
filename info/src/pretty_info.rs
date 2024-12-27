use std::sync::OnceLock;
use l6t::symbolic::group::ValueGroup;
use l6t::symbolic::rich::RichValueGroup;
use crate::{DecodedBundle, DecodedPatch};
use crate::pretty::*;

/// group separator
pub fn sep() -> &'static str {
    static STR: OnceLock<String> = OnceLock::new();
    STR.get_or_init(
        || std::iter::repeat('-').take(65).collect::<String>()
    )
}

/// header separator
pub fn hsep() -> &'static str {
    static STR: OnceLock<String> = OnceLock::new();
    STR.get_or_init(
        || std::iter::repeat('=').take(65).collect::<String>()
    )
}

impl Pretty for Vec<RichValueGroup> {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        let sep = sep();
        for group in self {
            writeln!(pp, "{}\n{}", group.name, sep)?;

            for (name, value) in &group.values {
                writeln!(pp, "{:30} : {}", name, value)?;
                if pp.with_simple {
                    let simple = value.get_simple();
                    writeln!(pp, "{:30} : {:5} : {}", "", simple.get_type(), simple)?;
                }
/*
                if value.is_simple() {
                    writeln!(pp, "{:30} : {:5} : {}", name, value.get_simple_type(), value)?;
                } else {
                    writeln!(pp, "{:30} : {}", name, value)?;
                    let simple = value.get_simple();
                    writeln!(pp, "{:30} : {:5} : {}", "", simple.get_type(), simple)?;
                }
 */
            }
            writeln!(pp)?;
        }

        Ok(())
    }
}

impl Pretty for Vec<ValueGroup> {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        let sep = sep();
        for group in self {
            writeln!(pp, "{}\n{}", group.name, sep)?;

            for (name, value) in &group.values {
                writeln!(pp, "{:30} : {:5} : {}", name, value.get_type(), value)?;
            }
            writeln!(pp)?;
        }

        Ok(())
    }
}

impl Pretty for DecodedPatch {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        Pretty::fmt_full(&self.patch, pp, false)?;
        writeln!(pp)?;

        Pretty::fmt(&self.values, pp)?;

        if !self.errors.is_empty() {
            writeln!(pp, "ERRORS\n{}", sep())?;
            for error in self.errors.iter() {
                writeln!(pp, "{}", error)?;
            }
        }

        Ok(())
    }
}
impl Pretty for DecodedBundle {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        if self.banks.is_empty() { return Ok(()) }
        if !self.is_bundle {
            // only one patch
            Pretty::fmt(&self.banks[0].patches[0], pp)?;
            return Ok(())
        }

        let sep = hsep();
        for (n, bank) in self.banks.iter().enumerate() {
            writeln!(pp, "[{}/{}] Bank: {}\n{}\n", n+1, self.banks.len(), bank.name, sep)?;

            for (n, patch) in bank.patches.iter().enumerate() {
                if n > 0 { writeln!(pp)?; }
                // This will prefix the L6Patch pretty output to form a seamless line:
                // [xx/xx] Patch type: ...
                write!(pp, "[{}/{}] ", n+1, bank.patches.len())?;
                Pretty::fmt(patch, pp)?;
            }
        }

        Ok(())
    }
}
