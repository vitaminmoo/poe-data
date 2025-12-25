use std::collections::HashMap;

// datc64 column types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
