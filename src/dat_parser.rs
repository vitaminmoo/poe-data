use anyhow::{anyhow, bail, Result};
use bytes::{Buf, Bytes};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

// datc64 table types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Unknown,
    SelfRow,
    ForeignRow,
    EnumRow,
    Bool,
    String,
    I16,
    U16,
    I32,
    U32,
    F32,
    I64,
    U64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Scalar(Scalar),
    Array(Scalar),
}

impl Column {
    pub fn bytes(&self) -> usize {
        match self {
            // index to the current table, 0xfe filled if null
            Column::Scalar(Scalar::SelfRow) => 8,
            // index to some other table, 0xfe filled if null
            Column::Scalar(Scalar::ForeignRow) => 16,
            // index to a non-table enum (not a dat, can be zero or 1 indexed), 0xfe filled if null
            Column::Scalar(Scalar::EnumRow) => 4,
            // uint8_le 0 or 1
            Column::Scalar(Scalar::Bool) => 1,
            // index into a utf-16 string in the data table with double-null termination
            Column::Scalar(Scalar::String) => 8,
            Column::Scalar(Scalar::I16) => 2,
            Column::Scalar(Scalar::U16) => 2,
            Column::Scalar(Scalar::I32) => 4,
            Column::Scalar(Scalar::U32) => 4,
            Column::Scalar(Scalar::F32) => 4,
            // 8 bytes of count, 8 bytes of offset in the data field. Offset is always increasing and interleaved evenly in column, row order
            // note that if count is 0 then offset is still valid but points to zero bytes, which means it can point to the last byte of the data section, and multiple adjacent empty array cells could point to the same offset if no other columns point to data
            Column::Array(_) => 16,
            // who knows
            Column::Scalar(_) => 0,
        }
    }
}

// A ColumnClaim is a object that declares that a column may or does exist at a particular offset in the row bytes
#[derive(Debug)]
pub struct ColumnClaim {
    pub offset: usize, // offset in bytes, either per row or for the data section (including 0xBB magic)
    pub bytes: usize,  // how many bytes the claim covers
    pub column_type: Column, // what type of field is this claim for
    pub labels: HashMap<String, String>, // arbitrary metadata for the claim
}

pub static DAT_LOADER: LazyLock<RwLock<DatLoader>> =
    LazyLock::new(|| RwLock::new(DatLoader::default()));

pub struct DatLoader {
    pub fs: poe_tools::bundle_fs::FS,
    // path -> dat file struct
    pub dat_files: HashMap<String, DatFile>,
    pub cache_dir: PathBuf,
}

impl Default for DatLoader {
    fn default() -> Self {
        let cache_dir = dirs::cache_dir().unwrap().join("poe_data_tools");
        let base_url = poe_tools::bundle_loader::cdn_base_url(&cache_dir, "2").unwrap();
        eprintln!("loading fs");
        let fs = poe_tools::bundle_fs::from_cdn(&base_url, &cache_dir);
        DatLoader {
            fs,
            dat_files: HashMap::new(),
            cache_dir,
        }
    }
}

impl DatLoader {
    pub fn get_table(&mut self, name: &str) -> Option<&mut DatFile> {
        if !self.dat_files.contains_key(name) {
            self.load_table(name);
        }
        self.dat_files.get_mut(name)
    }

    fn load_table(&mut self, name: &str) {
        if self.dat_files.contains_key(name) {
            return;
        }

        eprintln!("loading {}", name);
        match self.load_file(name) {
            Ok(loaded) => {
                self.dat_files.insert(name.to_string(), loaded);
            }
            Err(err) => panic!("Couldn't load table {}: {}", name, err),
        }
    }
    pub fn load_all_tables(&mut self) {
        for path in self.fs.list() {
            if PathBuf::from(&path).extension().unwrap_or_default() == "datc64" {
                self.load_table(&path);
            }
        }
    }
    pub fn load_file(&mut self, name: &str) -> Result<DatFile> {
        let file: Bytes;
        let cache_file = self.cache_dir.join("tables").join(name);

        if let Ok(cached_file) = fs::read(&cache_file) {
            file = cached_file.into();
        } else {
            file = self.fs.read(name).unwrap();
            fs::create_dir_all(cache_file.parent().unwrap()).expect("failed to create cache dir");
            fs::write(&cache_file, &file).expect("failed to write cache file");
        }

        // length + magic
        if file.len() < 4 + 8 {
            bail!("file too short");
        }

        let magic_index = file
            .windows(8)
            .position(|window| window == [0xBB; 8])
            .ok_or(anyhow!("magic bytes not found"))?;

        let mut data = Bytes::from_owner(file);
        let mut table = data.split_to(magic_index);

        let table_len_rows = table.get_u32_le() as usize;
        let mut row_len_bytes = 0;
        if table_len_rows > 0 {
            row_len_bytes = table.len() / table_len_rows;
        }

        let mut dat_file = DatFile {
            source: name.to_string(),
            table,
            _table_len_rows: table_len_rows,
            row_len_bytes,
            data,
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
}

#[derive(Debug, Clone)]
pub struct DatFile {
    pub source: String,         // path to the file that we got this data from
    pub table: Bytes,           // The entire fixed-length table section without the rows header
    pub _table_len_rows: usize, // how many rows in the table
    pub row_len_bytes: usize,   // how many bytes per row
    pub data: Bytes, // The entire variable-length data section, including 8 bytes of magic
    pub table_row_or: Vec<u8>, // 1 byte per row byte, all rows bitwise or'd together
    pub table_row_min: Vec<u8>, // 1 byte per row byte, containing the min value of all rows
    pub table_row_max: Vec<u8>, // 1 byte per row byte, containing the max value of all rows
}

impl DatFile {
    // Get all rows of the table
    pub fn rows(&self) -> Vec<Bytes> {
        self.table
            .chunks_exact(self.row_len_bytes)
            .map(|x| self.table.slice_ref(x))
            .collect()
    }
    pub fn rows_iter(&self) -> impl Iterator<Item = Bytes> + '_ {
        self.table
            .chunks_exact(self.row_len_bytes)
            .map(|x| self.table.slice_ref(x))
    }
    // Get all rows of a column by offset and length
    pub fn column_rows(&self, offset: usize, bytes: usize) -> Vec<Bytes> {
        self.rows_iter()
            .map(|x| x.slice(offset..offset + bytes))
            .collect()
    }
    pub fn column_rows_iter(
        &self,
        offset: usize,
        bytes: usize,
    ) -> impl Iterator<Item = Bytes> + '_ {
        self.rows_iter()
            .map(move |x| x.slice(offset..offset + bytes))
    }
    pub fn cell(&self, row: usize, index: usize, bytes: usize) -> Bytes {
        self.table
            .slice(row * self.row_len_bytes + index..row * self.row_len_bytes + index + bytes)
    }
    pub fn cell_foreignrow(&mut self, row: usize, index: usize) -> u64 {
        self.cell(row, index, Column::Scalar(Scalar::ForeignRow).bytes())
            .get_u64_le()
    }
    pub fn cell_selfrow(&mut self, row: usize, index: usize) -> u32 {
        self.cell(row, index, Column::Scalar(Scalar::SelfRow).bytes())
            .get_u32_le()
    }
    pub fn cell_enumrow(&mut self, row: usize, index: usize) -> u16 {
        self.cell(row, index, Column::Scalar(Scalar::EnumRow).bytes())
            .get_u16_le()
    }
    pub fn cell_i16(&mut self, row: usize, index: usize) -> i16 {
        self.cell(row, index, Column::Scalar(Scalar::I16).bytes())
            .get_i16_le()
    }
    pub fn cell_u16(&mut self, row: usize, index: usize) -> u16 {
        self.cell(row, index, Column::Scalar(Scalar::U16).bytes())
            .get_u16_le()
    }
    pub fn cell_i32(&mut self, row: usize, index: usize) -> i32 {
        self.cell(row, index, Column::Scalar(Scalar::I32).bytes())
            .get_i32_le()
    }
    pub fn cell_u32(&mut self, row: usize, index: usize) -> u32 {
        self.cell(row, index, Column::Scalar(Scalar::U32).bytes())
            .get_u32_le()
    }
    pub fn cell_f32(&mut self, row: usize, index: usize) -> f32 {
        self.cell(row, index, Column::Scalar(Scalar::F32).bytes())
            .get_f32_le()
    }
    pub fn cell_bool(&mut self, row: usize, index: usize) -> bool {
        self.cell(row, index, Column::Scalar(Scalar::Bool).bytes())
            .get_u8()
            > 0
    }
    pub fn cell_array_foreignrow(&mut self, row: usize, index: usize) -> Result<Vec<u64>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::ForeignRow).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u64_le()).collect())
    }
    pub fn cell_array_selfrow(&mut self, row: usize, index: usize) -> Result<Vec<u32>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::SelfRow).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u32_le()).collect())
    }
    pub fn cell_array_enumrow(&mut self, row: usize, index: usize) -> Result<Vec<u16>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::EnumRow).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u16_le()).collect())
    }
    pub fn cell_array_i16(&mut self, row: usize, index: usize) -> Result<Vec<i16>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::I16).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_i16_le()).collect())
    }
    pub fn cell_array_u16(&mut self, row: usize, index: usize) -> Result<Vec<u16>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::U16).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u16_le()).collect())
    }
    pub fn cell_array_i32(&mut self, row: usize, index: usize) -> Result<Vec<i32>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::I32).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_i32_le()).collect())
    }
    pub fn cell_array_u32(&mut self, row: usize, index: usize) -> Result<Vec<u32>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::U32).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u32_le()).collect())
    }
    pub fn cell_array_f32(&mut self, row: usize, index: usize) -> Result<Vec<f32>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::F32).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_f32_le()).collect())
    }
    pub fn cell_array_bool(&mut self, row: usize, index: usize) -> Result<Vec<bool>> {
        self.array_from_cell(row, index, Column::Scalar(Scalar::Bool).bytes())
            .map(|mut x| x.iter_mut().map(|y| y.get_u8() > 0).collect())
    }
    pub fn cell_array_string(&mut self, row: usize, index: usize) -> Result<Vec<String>> {
        let mut cell = self.cell(row, index, 16);
        let count = cell.get_u64_le() as usize;
        let offset = cell.get_u64_le() as usize;
        self.strings_from_offset(offset, count)
    }

    pub fn array_from_cell(
        &mut self,
        row: usize,
        index: usize,
        bytes: usize,
    ) -> Result<Vec<Bytes>> {
        let mut cell = self.cell(row, index, 16);
        let count = cell.get_u64_le() as usize;
        let offset = cell.get_u64_le() as usize;
        let array = self.array_from_offset(offset, count, bytes)?;
        Ok(array)
    }
    pub fn array_from_offset(
        &mut self,
        offset: usize,
        member_count: usize,
        member_bytes: usize,
    ) -> Result<Vec<Bytes>> {
        self.valid_data_ref(offset)?;
        //self.increment_data_ref(offset, member_count * member_bytes);
        let start = offset;
        let end = start + member_count * member_bytes;
        let data = self
            .data
            .get(start..end)
            .ok_or_else(|| anyhow!("invalid data range: {}:{}-{}", self.source, start, end))?;
        let result = data
            .chunks_exact(member_bytes)
            .map(|x| self.data.slice_ref(x))
            .collect();
        Ok(result)
    }

    // Get a string pointed to by a cell
    pub fn cell_string(&mut self, row: usize, index: usize) -> Result<String> {
        let mut cell = self.cell(row, index, 8);
        self.string_from_offset(cell.get_u64_le() as usize)
    }
    // Get a string pointed to by an offset in the data
    pub fn string_from_offset(&mut self, offset: usize) -> Result<String> {
        self.valid_data_ref(offset)?;
        let s = self.string_from_offset_if_valid(offset)?;
        //self.increment_data_ref(offset, s.len() * 2 + 4); // 2 bytes per char, 2 null terminators
        Ok(s)
    }
    // Get count strings pointed to by an offset in the data
    pub fn strings_from_offset(&self, offset: usize, count: usize) -> Result<Vec<String>> {
        let mut strings = Vec::new();
        let mut current_offset = offset;
        for _ in 0..count {
            let s = self.string_from_offset_if_valid(current_offset)?;
            current_offset += s.len() * 2 + 4; // +2 for null terminators
            strings.push(s);
        }
        Ok(strings)
    }

    // Get a string by offset
    fn string_from_offset_if_valid(&self, offset: usize) -> Result<String> {
        let mut start = self.data.slice(offset..);
        let mut utf16string = Vec::new();
        let mut complete = false;
        while start.has_remaining() {
            if start.remaining() < 2 {
                bail!(
                    "eof before double null-termination: {}:{}",
                    self.source,
                    offset,
                );
            }
            let utf16_val = start.get_u16_le();
            if utf16_val == 0 {
                if start.has_remaining() {
                    let next = start.get_u16_le();
                    if next != 0 {
                        bail!(
                            "string lacks second null-termination: {}:{}",
                            self.source,
                            offset,
                        );
                    }
                }
                complete = true;
                break;
            }
            utf16string.push(utf16_val);
        }

        if !complete {
            bail!(
                "string not null-terminated before eof: {}:{}",
                self.source,
                offset,
            );
        }

        Ok(String::from_utf16(&utf16string)?)
    }
    // check if an offset is valid for the data
    pub fn valid_data_ref(&self, offset: usize) -> Result<()> {
        if offset >= self.data.len() {
            bail!("offset out of bounds: {}:{}", self.source, offset);
        }
        if offset < 8 {
            bail!(
                "string offset is pointing to magic bytes: {}:{}",
                self.source,
                offset
            );
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
                            if self.table_row_min[col_index..col_index + 8]
                                == self.table_row_max[col_index..col_index + 8]
                            {
                                // if all rows have the same value, it's probably not a string, unless it's ""
                                if s.is_empty() {
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
                        column_type: Column::Scalar(Scalar::String),
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
                        column_type: Column::Array(Scalar::Unknown),
                        labels: HashMap::new(),
                    });
                }

                let col_max = cells
                    .iter_mut()
                    .map(|cell| cell.get_u128_le() as usize)
                    .max()
                    .unwrap();
                if col_max > 0 && col_max <= 30000 {
                    claims.push(ColumnClaim {
                        offset: col_index,
                        bytes: 16,
                        column_type: Column::Scalar(Scalar::ForeignRow),
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
    fn test_oneoff() {
        let mut dl = DatLoader::default();
        let dat_file: &mut DatFile = dl.get_table("data/mods.datc64").unwrap();
        println!("Id, ModType, Level");
        for row in 0..5 {
            print!(
                "{:?}, ",
                dat_file.cell_string(row, 0).unwrap_or("".to_string())
            );
            print!("{:?}, ", dat_file.cell_foreignrow(row, 10));
            print!("{:?}", dat_file.cell_u32(row, 26));
            println!();
        }
    }
    #[test]
    fn test_get_claims_mods() {
        let mut dl = DatLoader::default();
        let dat_file: &mut DatFile = dl.get_table("data/mods.datc64").unwrap();
        for bytes in [1, 2, 4, 8, 16] {
            if dat_file.row_len_bytes < bytes + 1 {
                continue;
            }
            let last_col = dat_file.row_len_bytes - bytes - 1;
            for index in 0..last_col {
                let claims = dat_file.get_column_claims(index, bytes);
                for claim in claims {
                    println!(
                        "{}:{}: {:?}",
                        claim.offset,
                        claim.offset + claim.bytes - 1,
                        claim.column_type
                    );
                }
            }
        }
    }
    #[test]
    fn test_load_all() {
        let mut dl = DatLoader::default();
        dl.load_all_tables();
        for (name, dat_file) in dl.dat_files.iter_mut() {
            for bytes in [1, 2, 4, 8, 16] {
                if dat_file.row_len_bytes < bytes + 1 {
                    continue;
                }
                let last_col = dat_file.row_len_bytes - bytes - 1;
                for index in 0..last_col {
                    let claims = dat_file.get_column_claims(index, bytes);
                    for claim in claims {
                        println!(
                            "{} {}:{}: {:?}",
                            name,
                            claim.offset,
                            claim.offset + claim.bytes - 1,
                            claim.column_type
                        );
                    }
                }
            }
        }
    }
}
