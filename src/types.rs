use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// datc64 column types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scalar {
    Unknown,
    SelfRow,
    ForeignRow,
    EnumRow,
    Bool,
    String,
    File,
    Directory,
    Color,
    Interval,
    DateTime,
    I16,
    U16,
    Hash16,
    I32,
    U32,
    Hash32,
    I64,
    U64,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TypeSet(u32);

impl TypeSet {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn full() -> Self {
        Self(!0) // All bits set
    }

    pub fn from_size(bytes: usize) -> Self {
        let mut set = Self::empty();
        match bytes {
            1 => {
                set.insert(Scalar::Bool);
            }
            2 => {
                set.insert(Scalar::I16);
                set.insert(Scalar::U16);
                set.insert(Scalar::Hash16);
            }
            4 => {
                set.insert(Scalar::I32);
                set.insert(Scalar::U32);
                set.insert(Scalar::Hash32);
                set.insert(Scalar::F32);
                set.insert(Scalar::EnumRow);
            }
            8 => {
                set.insert(Scalar::I64);
                set.insert(Scalar::U64);
                set.insert(Scalar::F64);
                set.insert(Scalar::SelfRow);
                set.insert(Scalar::String);
                set.insert(Scalar::File);
                set.insert(Scalar::Directory);
                set.insert(Scalar::Color);
                set.insert(Scalar::Interval);
                set.insert(Scalar::DateTime);
            }
            16 => {
                set.insert(Scalar::ForeignRow);
                // Arrays are handled separately in Cell, but here we track Scalar types.
                // We might need to handle "Array" as a distinct thing if we want to validte it,
                // but Cell::Array matches. For now, 16 bytes is mostly ForeignRow.
                // The old logic handled arrays separately.
            }
            _ => {}
        }
        set
    }

    fn bit_mask(s: Scalar) -> u32 {
        match s {
            Scalar::Unknown => 0,
            Scalar::SelfRow => 1 << 0,
            Scalar::ForeignRow => 1 << 1,
            Scalar::EnumRow => 1 << 2,
            Scalar::Bool => 1 << 3,
            Scalar::String => 1 << 4,
            Scalar::File => 1 << 5,
            Scalar::Directory => 1 << 6,
            Scalar::Color => 1 << 7,
            Scalar::Interval => 1 << 8,
            Scalar::I16 => 1 << 9,
            Scalar::U16 => 1 << 10,
            Scalar::Hash16 => 1 << 11,
            Scalar::I32 => 1 << 12,
            Scalar::U32 => 1 << 13,
            Scalar::Hash32 => 1 << 14,
            Scalar::I64 => 1 << 15,
            Scalar::U64 => 1 << 16,
            Scalar::F32 => 1 << 17,
            Scalar::F64 => 1 << 18,
            Scalar::DateTime => 1 << 19,
        }
    }

    pub fn insert(&mut self, s: Scalar) {
        self.0 |= Self::bit_mask(s);
    }

    pub fn remove(&mut self, s: Scalar) {
        self.0 &= !Self::bit_mask(s);
    }

    pub fn contains(&self, s: Scalar) -> bool {
        (self.0 & Self::bit_mask(s)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = Scalar> + '_ {
        // This is a simple iterator implementation
        // We can optimize this if needed, but for now checking all enum variants is fine given the small number
        [
            Scalar::SelfRow,
            Scalar::ForeignRow,
            Scalar::EnumRow,
            Scalar::Bool,
            Scalar::String,
            Scalar::File,
            Scalar::Directory,
            Scalar::Color,
            Scalar::Interval,
            Scalar::DateTime,
            Scalar::I16,
            Scalar::U16,
            Scalar::Hash16,
            Scalar::I32,
            Scalar::U32,
            Scalar::Hash32,
            Scalar::I64,
            Scalar::U64,
            Scalar::F32,
            Scalar::F64,
        ]
        .into_iter()
        .filter(move |&s| self.contains(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell {
    Scalar(Scalar),
    Array(Scalar),
}

impl Scalar {
    pub fn bytes(&self) -> usize {
        match self {
            Scalar::Unknown => 0,
            // index to a row in the current table, 0xfe filled if null
            Scalar::SelfRow => 8,
            // index to some other table, 0xfe filled if null
            Scalar::ForeignRow => 16,
            // index to a non-table enum (not in a datc64, can be zero or 1 indexed), 0xfe filled if null
            Scalar::EnumRow => 4,
            Scalar::Bool => 1,
            Scalar::String => 8,
            Scalar::File => 8,
            Scalar::Directory => 8,
            Scalar::Color => 8,
            Scalar::Interval => 8,
            Scalar::DateTime => 8,
            Scalar::I16 => 2,
            Scalar::U16 => 2,
            Scalar::Hash16 => 2,
            Scalar::I32 => 4,
            Scalar::U32 => 4,
            Scalar::Hash32 => 4,
            Scalar::I64 => 8,
            Scalar::U64 => 8,
            Scalar::F32 => 4,
            Scalar::F64 => 8,
        }
    }
}

impl Cell {
    pub fn bytes(&self) -> usize {
        match self {
            Cell::Scalar(s) => s.bytes(),
            Cell::Array(_) => 16,
        }
    }
}

// A ColumnClaim is a object that declares that a column may or does exist at a particular offset in the row bytes
#[derive(Debug, Clone)]
pub struct ColumnClaim {
    pub offset: usize,                   // offset in bytes, either per row or for the data section (including 0xBB magic)
    pub bytes: usize,                    // how many bytes the claim covers
    pub column_type: Cell,               // what type of field is this claim for
    pub labels: HashMap<String, String>, // arbitrary metadata for the claim
}

#[derive(Debug, Clone)]
pub struct ByteStats {
    pub or_value: u8,
    pub min_value: u8,
    pub max_value: u8,
    pub counts: [usize; 256], // Frequency of each byte value
    pub unique_count: usize,
}

impl Default for ByteStats {
    fn default() -> Self {
        Self::new()
    }
}

impl ByteStats {
    pub fn new() -> Self {
        ByteStats {
            or_value: 0,
            min_value: 0xFF,
            max_value: 0,
            counts: [0; 256],
            unique_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableStats {
    // One entry per byte offset in a row
    pub per_byte_stats: Vec<ByteStats>,
}

impl TableStats {
    pub fn new(row_len_bytes: usize) -> Self {
        TableStats {
            per_byte_stats: vec![ByteStats::new(); row_len_bytes],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub check: String,
    pub row: usize,
    pub value_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnValidation {
    pub offset: usize,
    pub length: usize,
    pub allowed_types: Vec<Scalar>,
    pub is_array: bool,
    pub failures: Vec<(Scalar, ValidationError)>,
}
