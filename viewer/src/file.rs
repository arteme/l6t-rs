use l6t::model::L6Patch;
use l6t::symbolic::rich::RichValueGroup;
use serde::Serialize;

#[derive(Serialize)]
pub struct Patch {
    pub patch: L6Patch,
    pub values: Vec<RichValueGroup>,
    pub errors: Vec<String>
}

pub struct Bank {
    pub name: String,
    pub patches: Vec<Patch>,
}

pub struct Bundle {
    pub banks: Vec<Bank>,
}

pub enum File {
    Patch(Patch),
    Bundle(Bundle)
}

pub enum Selection<'a> {
    None,
    Patch(&'a Patch),
    Bank(&'a Bank),
}