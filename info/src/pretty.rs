pub use std::fmt;
pub use std::fmt::Write;
use clap::builder::Str;

pub struct PrettyPrinter {
    pub indent: usize,
    pub step: usize,
    pub buffer: String,
    pub full: bool,
    ///
    pub with_simple: bool,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self::with_simple(false)
    }
    pub fn with_simple(with_simple: bool) -> Self {
        PrettyPrinter { indent: 0, step: 2, buffer: "".into(), full: true, with_simple }
    }

    fn indent(&mut self) {
        let indent = format!("{empty:width$}", empty="", width=(self.indent * self.step));
        self.buffer += indent.as_str()
    }
    fn nl(&mut self) {
        self.buffer += &"\n";
    }

    pub fn println<T: Pretty + ?Sized>(&mut self, obj: &T) -> fmt::Result {
        Pretty::fmt(obj, self)?;
        println!("{}", self.buffer);
        self.buffer = String::new();
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

pub trait Pretty {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result;

    /** Same as `fmt`, but overrides `pp.full` for this call */
    fn fmt_full(&self, pp: &mut PrettyPrinter, full: bool) -> fmt::Result {
        let prev = pp.full;
        pp.full = full;
        let res = self.fmt(pp);
        pp.full = prev;
        res
    }
}
