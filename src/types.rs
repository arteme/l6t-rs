use std::fmt::Debug;

/// The Type ID of a chunk.
#[derive(Clone,Copy,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct TypeID(pub [u8; 4]);

macro_rules! type_id {
    ($name:ident, $val:expr) => (
        pub const $name: TypeID = TypeID([$val[0], $val[1], $val[2], $val[3]]);
    );
    ($name:ident) => (
        type_id!($name, stringify!($name).as_bytes());
    )
}

type_id!(FORM);
type_id!(LIST);
type_id!(FORM_LE, b"MROF");
type_id!(LIST_LE, b"TSIL");

impl TypeID {
    pub fn is_le_envelope(self) -> bool {
        match self {
            FORM_LE | LIST_LE => true,
            _ => false,
        }
    }

    pub fn is_envelope(self) -> bool {
        match self {
            FORM | LIST => true,
            _ => false,
        }
    }
    pub fn reverse(&self) -> TypeID {
        TypeID([self.0[3], self.0[2], self.0[1], self.0[0]])
    }
}

impl std::fmt::Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(),std::fmt::Error> {
        write!(f, "{}{}{}{}", self.0[0] as char, self.0[1] as char, self.0[2] as char, self.0[3] as char)
    }
}

impl std::fmt::Debug for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TypeID(b\"{}\")", self)
    }
}

impl<'a> From<&'a [u8; 4]> for TypeID {
    fn from(id: &'a [u8; 4]) -> TypeID {
        TypeID([id[0], id[1], id[2], id[3]])
    }
}

type_id!(L6BA, b"L6BA");
type_id!(L6CO, b"L6CO");
type_id!(L6PA, b"L6PA");
type_id!(L6TE, b"L6TE");
type_id!(L6AS, b"L6AS");
type_id!(L6FS, b"L6FS");
type_id!(HEAD, b"HEAD");
type_id!(BANK, b"BANK");
type_id!(FLDR, b"FLDR");
type_id!(BINF, b"BINF");
type_id!(PINF, b"PINF");
type_id!(MINF, b"MINF");
type_id!(MODL, b"MODL");
type_id!(PARM, b"LARM");
type_id!(PATC, b"PATC");
type_id!(UNFO, b"UNFO");
type_id!(INFO, b"INFO");
type_id!(META, b"META");
type_id!(TUNE, b"TUNE");
type_id!(TN12, b"TN12");
type_id!(TNAC, b"TNAC");
type_id!(TNGS, b"TNGS");
type_id!(TNG2, b"TNG2");
type_id!(PKPS, b"PKPS");
type_id!(PKP_, b"PKP_");
type_id!(L6GP, b"L6GP");
type_id!(TONE, b"TONE");
type_id!(AMP_, b"AMP_");
type_id!(CAB_, b"CAB_");
type_id!(PRFX, b"PRFX");
type_id!(EFX_, b"EFX_");
type_id!(SSLB, b"SSLB");
type_id!(LHDR, b"LHDR");
type_id!(WSEQ, b"WSEQ");
type_id!(LENT, b"LENT");

pub const UNALIGNED_CHUNKS: &[&TypeID] = &[ &SSLB ];
pub const PATCH_IDS: &[&TypeID] = &[ &L6PA, &L6AS, &L6FS, &L6TE, &L6GP ];
pub const BUNDLE_IDS: &[&TypeID] = &[ &L6BA, &L6CO, &SSLB ];

type_id!(IAUT);
type_id!(IBND);
type_id!(ICMT);
type_id!(IGTR);
type_id!(ISNG);
type_id!(ISTL);
type_id!(IPUS);
type_id!(IPUP);
type_id!(IDAT);
type_id!(IAMP);
type_id!(IAPP);
type_id!(IAPV);

pub enum MetaTagsTypes {
    author = 0,
    /*
guitarist: String,
band: String,
song: String,
style: String,
pickup_style: String,
pickup_position: String,
date: u16,
amp_name: String,
creator_app: String,
creator_app_version: String

     */
}
