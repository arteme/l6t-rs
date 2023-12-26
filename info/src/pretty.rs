use std::fmt;

pub struct PrettyPrinter {
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

    pub(crate) fn println<T: Pretty>(obj: &T) -> fmt::Result {
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

pub trait Pretty {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result;
}
