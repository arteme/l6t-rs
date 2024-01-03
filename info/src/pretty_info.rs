use std::sync::OnceLock;
use l6t::symbolic::value::ValueGroup;
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
        if self.banks.len() == 1 && self.banks[0].patches.len() == 1 {
            // a bundle with only one patch
            Pretty::fmt(&self.banks[0].patches[0], pp)?;
        }

        let sep = hsep();
        for bank in &self.banks {
            writeln!(pp, "Bank: {}\n{}\n", bank.name, sep)?;

            for (n, patch) in bank.patches.iter().enumerate() {
                if n > 0 { writeln!(pp)?; }
                writeln!(pp, "[{}/{}]", n+1, bank.patches.len())?;
                Pretty::fmt(patch, pp)?;
            }
        }

        Ok(())
    }
}
