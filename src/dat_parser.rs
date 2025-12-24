use anyhow::{anyhow, bail, Result};
use bytes::{Buf, Bytes};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

// datc64 column types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Unknown,
    SelfRow,
    ForeignRow,
    EnumRow,
    Bool,
    String,
    Interval,
    I16,
    U16,
    I32,
    U32,
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
            Scalar::Interval => 8,
            Scalar::I16 => 2,
            Scalar::U16 => 2,
            Scalar::I32 => 4,
            Scalar::U32 => 4,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarRef {
    SelfRef(u64),
    ForeignRef(u128),
    EnumRef(i32),
    Bool(bool),
    String(u64),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Interval(i32, i32),
}

// A ColumnClaim is a object that declares that a column may or does exist at a particular offset in the row bytes
#[derive(Debug)]
pub struct ColumnClaim {
    pub offset: usize,                   // offset in bytes, either per row or for the data section (including 0xBB magic)
    pub bytes: usize,                    // how many bytes the claim covers
    pub column_type: Cell,               // what type of field is this claim for
    pub labels: HashMap<String, String>, // arbitrary metadata for the claim
}

pub static DAT_LOADER: LazyLock<RwLock<DatLoader>> = LazyLock::new(|| RwLock::new(DatLoader::default()));

pub struct DatLoader {
    // path -> dat file struct
    pub dat_files: HashMap<String, DatFile>,
    fs: poe_data_tools::bundle_fs::FS,
}

impl Default for DatLoader {
    fn default() -> Self {
        let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
        std::fs::create_dir_all(&cache_dir).unwrap();
        let base_url = poe_data_tools::bundle_loader::cdn_base_url(&cache_dir, "2").unwrap();
        eprintln!("loading fs");
        let fs = poe_data_tools::bundle_fs::FS::from_cdn(&base_url, &cache_dir).unwrap();
        DatLoader { fs, dat_files: HashMap::new() }
    }
}

impl DatLoader {
    pub fn get_table(&mut self, name: &str) -> Option<&DatFile> {
        self.load_table(name);
        self.dat_files.get(name)
    }
    fn load_table(&mut self, name: &str) {
        if self.dat_files.contains_key(name) {
            return;
        }
        eprintln!("loading {}", name);
        match self.load_file(name) {
            Ok(loaded) => {
                let df = parse_file(name, loaded).unwrap();
                self.dat_files.insert(name.to_string(), df);
            }
            Err(err) => panic!("Couldn't load table {}: {}", name, err),
        }
    }
    // load_file gets a file from the cache or cdn and returns their Bytes
    fn load_file(&mut self, name: &str) -> Result<Bytes> {
        self.fs.read(name)
    }
    pub fn get_tables<'a>(&mut self, names: &'a [&'a str]) -> impl Iterator<Item = (&'a str, &DatFile)> {
        self.load_tables(names);
        names.iter().map(|n| (*n, self.dat_files.get(*n).unwrap()))
    }
    pub fn load_tables<'a>(&'a mut self, names: &[&'a str]) {
        let missing = names.iter().copied().filter(|n| !self.dat_files.contains_key(*n)).collect::<Vec<&str>>();
        let loaded = self
            .load_files(&missing)
            .map(|(n, b)| match parse_file(n, b) {
                Ok(df) => (n.to_string(), df),
                Err(_) => panic!("fuck"),
            })
            .collect::<Vec<_>>();
        self.dat_files.extend(loaded);
    }
    // load_files efficiently gets all specified files from the cache or cdn and returns their Bytes
    pub fn load_files<'a>(&'a self, names: &[&'a str]) -> impl Iterator<Item = (&'a str, Bytes)> {
        self.fs.batch_read(names).map(|res| match res {
            Ok((n, b)) => (n, b),
            Err((n, e)) => panic!("Failed to read file {}: {}", n, e),
        })
    }
}

fn parse_file(source: &str, file: Bytes) -> Result<DatFile> {
    // length + magic
    if file.len() < 4 + 8 {
        bail!("file too short");
    }

    let magic_index = file.windows(8).position(|window| window == [0xBB; 8]).ok_or(anyhow!("magic bytes not found"))?;

    let mut data = Bytes::from_owner(file);
    let mut table = data.split_to(magic_index);

    let table_len_rows = table.get_u32_le() as usize;
    let mut row_len_bytes = 0;
    if table_len_rows > 0 {
        row_len_bytes = table.len() / table_len_rows;
    }

    let mut dat_file = DatFile {
        source: source.to_string(),
        table,
        bytes_per_row: row_len_bytes,
        vdata: data,
        table_row_or: vec![0; row_len_bytes],
        table_row_min: vec![0xFF; row_len_bytes],
        table_row_max: vec![0; row_len_bytes],
    };

    if table_len_rows == 0 {
        return Ok(dat_file);
    }

    for row in dat_file.table.chunks_exact(row_len_bytes) {
        for (i, &byte) in row.iter().enumerate() {
            dat_file.table_row_or[i] |= byte;
            dat_file.table_row_min[i] = dat_file.table_row_min[i].min(byte);
            dat_file.table_row_max[i] = dat_file.table_row_max[i].max(byte);
        }
    }

    Ok(dat_file)
}

#[derive(Debug, Clone)]
pub struct DatFile {
    pub source: String,         // path to the file that we got this data from
    pub table: Bytes,           // the entire fixed-length table section without the rows header
    pub bytes_per_row: usize,   // how many bytes per row
    pub vdata: Bytes,           // the entire variable-length data section, including 8 bytes of magic
    pub table_row_or: Vec<u8>,  // 1 byte per row byte, all rows bitwise or'd together
    pub table_row_min: Vec<u8>, // 1 byte per row byte, containing the min value of all rows
    pub table_row_max: Vec<u8>, // 1 byte per row byte, containing the max value of all rows
}

impl DatFile {
    // Get all rows of the table
    pub fn rows(&self) -> Vec<Bytes> {
        self.rows_iter().collect()
    }
    pub fn rows_iter(&self) -> impl Iterator<Item = Bytes> + '_ {
        self.table.chunks_exact(self.bytes_per_row).map(|x| self.table.slice_ref(x))
    }
    // Get all rows of a column by offset and length
    pub fn column_rows(&self, offset: usize, bytes: usize) -> Vec<Bytes> {
        self.column_rows_iter(offset, bytes).collect()
    }
    pub fn column_rows_iter(&self, offset: usize, bytes: usize) -> impl Iterator<Item = Bytes> + '_ {
        self.rows_iter().map(move |x| x.slice(offset..offset + bytes))
    }

    pub fn array_from_offset(&self, offset: usize, member_count: usize, member_bytes: usize) -> Result<Vec<Bytes>> {
        if member_count == 0 {
            return Ok(Vec::new());
        }
        self.valid_data_ref(offset)?;
        //self.increment_data_ref(offset, member_count * member_bytes);
        let start = offset;
        let end = start + member_count * member_bytes;
        let data = self
            .vdata
            .get(start..end)
            .ok_or_else(|| anyhow!("invalid data range: {}:{}-{}", self.source, start, end))?;
        let result = data.chunks_exact(member_bytes).map(|x| self.vdata.slice_ref(x)).collect();
        Ok(result)
    }

    // Get a string pointed to by an offset in the data
    pub fn string_from_offset(&self, offset: usize) -> Result<String> {
        self.valid_data_ref(offset)?;
        let s = self.string_from_offset_if_valid(offset)?;
        //self.increment_data_ref(offset, s.len() * 2 + 4); // 2 bytes per char, 2 null terminators
        Ok(s)
    }
    // Get count strings pointed to by an offset in the data
    pub fn strings_from_offset(&self, offset: usize, count: usize) -> Result<Vec<String>> {
        let mut strings = Vec::new();
        //let mut current_offset = offset;
        let mut string_offset_bytes = self.vdata.slice(offset..offset + (4 * count));
        for _ in 0..count {
            let string_offset = string_offset_bytes.get_u32_le() as usize;
            let s = self.string_from_offset_if_valid(string_offset)?;
            //current_offset += s.len() * 2 + 4; // +2 for null terminators
            strings.push(s);
        }
        Ok(strings)
    }

    // Get a string by offset
    fn string_from_offset_if_valid(&self, offset: usize) -> Result<String> {
        let mut start = self.vdata.slice(offset..);
        let mut utf16string = Vec::new();
        let mut complete = false;
        while start.has_remaining() {
            if start.remaining() < 2 {
                bail!("eof before double null-termination: {}:{}", self.source, offset,);
            }
            let utf16_val = start.get_u16_le();
            if utf16_val == 0 {
                if start.remaining() < 2 {
                    bail!("not enough bytes or some shit I don't know: {}:{}", self.source, offset)
                }
                let next = start.get_u16_le();
                if next != 0 {
                    bail!("string lacks second null-termination: {}:{}", self.source, offset,);
                }
                complete = true;
                break;
            }
            utf16string.push(utf16_val);
        }

        if !complete {
            bail!("string not null-terminated before eof: {}:{}", self.source, offset,);
        }

        Ok(String::from_utf16(&utf16string)?)
    }

    // check if an offset is valid for the data
    pub fn valid_data_ref(&self, offset: usize) -> Result<()> {
        if offset > self.vdata.len() {
            bail!("offset out of bounds: {}:{} (data len {})", self.source, offset, self.vdata.len());
        }
        if offset < 8 {
            bail!("string offset is pointing to magic bytes: {}:{}", self.source, offset);
        }
        Ok(())
    }

    pub fn get_column_claims(&self, col_index: usize, cell_length: usize) -> Vec<ColumnClaim> {
        let mut cells = self.column_rows(col_index, cell_length);
        if cells.is_empty() {
            println!("no cells for column {}", col_index);
            return Vec::new();
        }
        match cell_length {
            1 => {
                /* disabled while testing because possible bools are everywhere and I don't care
                let mut claims = Vec::new();
                if !cells.iter_mut().any(|cell| cell.get_u8() > 1) {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 1,
                        column_type: Column::Scalar(Scalar::Bool),
                        labels: HashMap::new(),
                    });
                }
                claims
                */
                Vec::new()
            }
            2 => Vec::new(),
            4 => Vec::new(),
            8 => {
                let mut claims = Vec::new();
                let mut seen_strings = HashMap::new();
                if !cells.iter_mut().any(|cell| {
                    let offset = cell.get_u64_le() as usize;
                    if self.valid_data_ref(offset).is_err() {
                        return true;
                    }
                    match self.string_from_offset_if_valid(offset) {
                        Ok(s) => {
                            if s.len() < 2 && !(s.is_empty() || s == " ") {
                                return true;
                            }
                            if !s.is_ascii() {
                                return true;
                            }
                            if let Some(prev_offset) = seen_strings.get(&s) {
                                if offset != *prev_offset {
                                    return true;
                                }
                            }
                            if self.table_row_min[col_index..col_index + 8] == self.table_row_max[col_index..col_index + 8] {
                                // if all rows have the same value, it's probably not a string, unless it's ""
                                if !s.is_empty() {
                                    return true;
                                }
                            }
                            if self.table_row_min[col_index..col_index + 1] == [0xfe; 1] && self.table_row_max[col_index..col_index + 1] == [0xfe; 1] {
                                // if the first byte is all fe it's probably overlapping an empty thing
                                return true;
                            }
                            if self.table_row_min[col_index..col_index + 1] == [0x00; 1] && self.table_row_max[col_index..col_index + 1] == [0x00; 1] {
                                // if the first byte is all 00 it's probably not a string unless it's ""
                                if !s.is_empty() {
                                    return true;
                                }
                            }
                            seen_strings.insert(s.to_owned(), offset);
                        }
                        Err(_) => return true,
                    }
                    false
                }) {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 8,
                        column_type: Cell::Scalar(Scalar::String),
                        labels: HashMap::new(),
                    });
                }
                claims
            }
            16 => {
                let mut claims = Vec::new();
                let mut prev_offset = 0;
                let mut is_array = true;
                for cell in cells.clone().iter_mut() {
                    let count = cell.get_u64_le() as usize;
                    let offset = cell.get_u64_le() as usize;
                    if self.valid_data_ref(offset).is_err() {
                        is_array = false;
                        break;
                    }
                    if offset <= prev_offset || count >= 30 {
                        is_array = false;
                        break;
                    }
                    prev_offset = offset;
                }
                if is_array {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 16,
                        column_type: Cell::Array(Scalar::Unknown),
                        labels: HashMap::new(),
                    });
                }

                let col_max = cells.iter_mut().map(|cell| cell.get_u128_le() as usize).max().unwrap();
                if col_max > 0 && col_max <= 30000 {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 16,
                        column_type: Cell::Scalar(Scalar::ForeignRow),
                        labels: HashMap::new(),
                    });
                }
                claims
            }
            _ => Vec::new(),
        }
    }
}

pub fn hexdump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}  ", i * 16);
        for &byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in chunk.len()..16 {
            print!("   ");
        }
        print!(" |");
        for &byte in chunk {
            let c = byte as char;
            if c.is_ascii_graphic() || c == ' ' {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        for _ in chunk.len()..16 {
            print!(" ");
        }
        print!("| ");

        // UTF-16 interpretation (assuming little-endian)
        let utf16_chunks = chunk.chunks_exact(2);
        for utf16_bytes in utf16_chunks {
            let utf16_val = u16::from_le_bytes([utf16_bytes[0], utf16_bytes[1]]);
            match char::from_u32(utf16_val as u32) {
                Some(c) => print!("{}", c),
                None => print!("."), // Or handle invalid UTF-16 as needed
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_claims_mods() {
        let mut dl = DatLoader::default();

        println!("getting dat_paths");
        let dat_paths = dl
            .fs
            .list()
            .filter(|x| x.ends_with(".datc64"))
            .filter(|x| {
                if let Some(rest) = x.strip_prefix("data/balance/") {
                    !rest.contains('/')
                } else if let Some(rest) = x.strip_prefix("data/") {
                    !rest.contains('/')
                } else {
                    false
                }
            })
            .collect::<Vec<String>>();

        //let shit = vec!["data/balance/worldareas.datc64"];
        let shit = dat_paths.iter().map(|x| x.as_str()).collect::<Vec<_>>();

        println!("converting dat_paths");

        for (_, dat_file) in dl.get_tables(&shit) {
            //for bytes in [1, 2, 4, 8, 16] {
            for bytes in [8] {
                if dat_file.bytes_per_row < bytes + 1 {
                    continue;
                }
                let last_col = dat_file.bytes_per_row - bytes - 1;
                for index in 0..last_col {
                    let claims = dat_file.get_column_claims(index, bytes);
                    for claim in claims {
                        print!(
                            "{}:{}..{}, {:?}",
                            dat_file.source,
                            claim.offset,
                            claim.offset + claim.bytes - 1,
                            claim.column_type
                        );
                        if claim.column_type == Cell::Scalar(Scalar::String) {
                            let mut empty = true;
                            for s in dat_file
                                .column_rows_iter(claim.offset, claim.bytes)
                                .map(|cell| dat_file.string_from_offset(cell.clone().get_i32_le() as usize).unwrap())
                                .take(5)
                                .filter(|s| !s.is_empty())
                            {
                                empty = false;
                                print!("{}, ", s,);
                            }
                            if empty {
                                print!(" <all empty strings> ");
                            }
                        }
                        println!()
                    }
                }
            }
        }
    }
}
