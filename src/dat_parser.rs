use anyhow::{anyhow, bail, Result};
use bytes::{Buf, Bytes};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::heuristics;
use crate::types::{ColumnClaim, TableStats};

pub static DAT_LOADER: LazyLock<RwLock<DatLoader>> = LazyLock::new(|| RwLock::new(DatLoader::default()));

pub struct DatLoader {
    // path -> dat file struct
    pub dat_files: HashMap<String, DatFile>,
    fs: poe_data_tools::bundle_fs::FS,
}

impl Default for DatLoader {
    fn default() -> Self {
        Self::new("2").expect("Failed to create default DatLoader")
    }
}

impl DatLoader {
    pub fn new(version: &str) -> Result<Self> {
        let cache_dir = dirs::cache_dir().ok_or_else(|| anyhow!("no cache dir"))?.join("poe_data_tools");
        std::fs::create_dir_all(&cache_dir)?;
        let base_url = poe_data_tools::bundle_loader::cdn_base_url(&cache_dir, version)?;
        eprintln!("loading fs for version {}", version);
        let fs = poe_data_tools::bundle_fs::FS::from_cdn(&base_url, &cache_dir)?;
        Ok(DatLoader { fs, dat_files: HashMap::new() })
    }

    pub fn get_table(&mut self, name: &str) -> Option<&DatFile> {
        if let Err(e) = self.load_table(name) {
            eprintln!("Failed to load table {}: {}", name, e);
            return None;
        }
        self.dat_files.get(name)
    }
    fn load_table(&mut self, name: &str) -> Result<()> {
        if self.dat_files.contains_key(name) {
            return Ok(());
        }
        eprintln!("loading {}", name);
        let loaded = self.load_file(name)?;
        let df = parse_file(name, loaded)?;
        self.dat_files.insert(name.to_string(), df);
        Ok(())
    }
    // load_file gets a file from the cache or cdn and returns their Bytes
    fn load_file(&mut self, name: &str) -> Result<Bytes> {
        self.fs.read(name)
    }
    pub fn get_tables<'a>(&mut self, names: &'a [&'a str]) -> impl Iterator<Item = (&'a str, &DatFile)> {
        self.load_tables(names);
        names.iter().filter_map(|n| self.dat_files.get(*n).map(|df| (*n, df)))
    }
    pub fn load_tables<'a>(&'a mut self, names: &[&'a str]) {
        let missing = names.iter().copied().filter(|n| !self.dat_files.contains_key(*n)).collect::<Vec<&str>>();
        let loaded = self
            .load_files(&missing)
            .filter_map(|(n, b)| match parse_file(n, b) {
                Ok(df) => Some((n.to_string(), df)),
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", n, e);
                    None
                }
            })
            .collect::<Vec<_>>();
        self.dat_files.extend(loaded);
    }
    // load_files efficiently gets all specified files from the cache or cdn and returns their Bytes
    pub fn load_files<'a>(&'a self, names: &[&'a str]) -> impl Iterator<Item = (&'a str, Bytes)> {
        self.fs.batch_read(names).filter_map(|res| match res {
            Ok((n, b)) => Some((n, b)),
            Err((_, e)) => {
                eprintln!("Failed to read file: {}", e);
                None
            }
        })
    }
    pub fn get_file_list(&self) -> Vec<String> {
        self.fs.list().collect()
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
        row_count: table_len_rows,
        bytes_per_row: row_len_bytes,
        vdata: data,
        stats: TableStats::new(row_len_bytes),
    };

    if table_len_rows == 0 {
        return Ok(dat_file);
    }

    for row in dat_file.table.chunks_exact(row_len_bytes) {
        for (i, &byte) in row.iter().enumerate() {
            let byte_stats = &mut dat_file.stats.per_byte_stats[i];
            byte_stats.or_value |= byte;
            byte_stats.min_value = byte_stats.min_value.min(byte);
            byte_stats.max_value = byte_stats.max_value.max(byte);
            byte_stats.counts[byte as usize] += 1;
        }
    }

    for byte_stats in &mut dat_file.stats.per_byte_stats {
        byte_stats.unique_count = byte_stats.counts.iter().filter(|&&c| c > 0).count();
    }

    Ok(dat_file)
}

#[derive(Debug, Clone)]
pub struct DatFile {
    pub source: String,       // path to the file that we got this data from
    pub table: Bytes,         // the entire fixed-length table section without the rows header
    pub row_count: usize,     // number of rows in the table
    pub bytes_per_row: usize, // how many bytes per row
    pub vdata: Bytes,         // the entire variable-length data section, including 8 bytes of magic
    pub stats: TableStats,
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
        let mut string_offset_bytes = self.vdata.slice(offset..offset + (8 * count));
        for _ in 0..count {
            let string_offset = string_offset_bytes.get_u64_le() as usize;
            let s = self.string_from_offset_if_valid(string_offset)?;
            //current_offset += s.len() * 2 + 4; // +2 for null terminators
            strings.push(s);
        }
        Ok(strings)
    }

    // Get a string by offset
    pub fn string_from_offset_if_valid(&self, offset: usize) -> Result<String> {
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

    pub fn get_column_claims(&self, col_index: usize, cell_length: usize, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
        heuristics::get_column_claims(self, col_index, cell_length, known_files)
    }

    pub fn get_all_column_claims(&self, known_files: Option<&[String]>) -> Vec<ColumnClaim> {
        heuristics::get_all_column_claims(self, known_files)
    }

    pub fn validate_types(&self, known_files: Option<&[String]>) -> Vec<crate::types::ColumnValidation> {
        heuristics::validate_file_types(self, known_files)
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
    use crate::types::{Cell, Scalar};

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

        let mut file_list = dl.get_file_list();
        for f in file_list.iter_mut() {
            *f = f.to_lowercase();
        }
        file_list.sort();

        //let shit = vec!["data/balance/worldareas.datc64"];
        let shit = dat_paths.iter().map(|x| x.as_str()).collect::<Vec<_>>();

        println!("converting dat_paths");

        for (_, dat_file) in dl.get_tables(&shit) {
            let claims = dat_file.get_all_column_claims(Some(&file_list));
            for claim in claims {
                print!(
                    "{}:{}..{}, {:?}",
                    dat_file.source,
                    claim.offset,
                    claim.offset + claim.bytes - 1,
                    claim.column_type
                );
                if matches!(
                    claim.column_type,
                    Cell::Scalar(Scalar::String) | Cell::Scalar(Scalar::File) | Cell::Scalar(Scalar::Directory) | Cell::Scalar(Scalar::Color)
                ) {
                    let mut empty = true;
                    for s in dat_file
                        .column_rows_iter(claim.offset, claim.bytes)
                        .map(|cell| {
                            dat_file
                                .string_from_offset(cell.clone().get_u64_le() as usize)
                                .unwrap_or_else(|_| "<null/invalid>".to_string())
                        })
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

    #[test]
    fn test_specific_tables() {
        let mut dl = DatLoader::default();
        let mut file_list = dl.get_file_list();
        for f in file_list.iter_mut() {
            *f = f.to_lowercase();
        }
        file_list.sort();

        let files = vec!["data/balance/rarity.datc64", "data/balance/hideouts.datc64", "data/balance/miscanimated.datc64"];
        dl.load_tables(&files);

        // Rarity Test
        if let Some(rarity) = dl.dat_files.get("data/balance/rarity.datc64") {
            // offset 32 is a color string
            let claims = rarity.get_column_claims(32, 8, Some(&file_list));
            assert!(
                claims.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::Color))),
                "Rarity offset 32 should be Color"
            );
        }

        // Hideouts Test
        if let Some(hideouts) = dl.dat_files.get("data/balance/hideouts.datc64") {
            // offset 16 is Hash16
            let claims_16 = hideouts.get_column_claims(16, 2, Some(&file_list));
            assert!(
                claims_16.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::Hash16))),
                "Hideouts offset 16 should be Hash16"
            );

            // offset 18 is File
            let claims_18 = hideouts.get_column_claims(18, 8, Some(&file_list));
            assert!(
                claims_18.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::File))),
                "Hideouts offset 18 should be File"
            );

            // offset 52 is Array of 4-byte ints (16 bytes total for array ref)
            let claims_52 = hideouts.get_column_claims(52, 16, Some(&file_list));
            assert!(
                claims_52.iter().any(|c| matches!(c.column_type, Cell::Array(_))),
                "Hideouts offset 52 should be Array"
            );

            // offset 69 is foreign reference to MtxTypes table
            let claims_69 = hideouts.get_column_claims(69, 16, Some(&file_list));
            assert!(
                claims_69.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::ForeignRow))),
                "Hideouts offset 69 should be ForeignRow"
            );
        }

        // MiscAnimated Test
        if let Some(miscanimated) = dl.dat_files.get("data/balance/miscanimated.datc64") {
            // offset 40 is Hash32
            let claims_40 = miscanimated.get_column_claims(40, 4, Some(&file_list));
            assert!(
                claims_40.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::Hash32))),
                "MiscAnimated offset 40 should be Hash32"
            );
        }
    }

    #[test]
    fn test_hideouts_hash16_candidates() {
        let mut dl = DatLoader::default();
        let files = vec!["data/balance/hideouts.datc64"];
        dl.load_tables(&files);

        if let Some(hideouts) = dl.dat_files.get("data/balance/hideouts.datc64") {
            println!("Analyzing Hideouts.datc64 for Hash16 candidates...");
            let mut detected = Vec::new();
            // Iterate all possible 2-byte alignments
            for i in 0..hideouts.bytes_per_row - 1 {
                let claims = hideouts.get_column_claims(i, 2, None);
                if claims.iter().any(|c| matches!(c.column_type, Cell::Scalar(Scalar::Hash16))) {
                    println!("Hash16 candidate found at offset {}", i);
                    detected.push(i);
                }
            }
            println!("Detected Hash16 offsets: {:?}", detected);
        }
    }

    #[test]
    fn test_conflict_resolution() {
        let mut dl = DatLoader::default();
        let files = vec!["data/balance/hideouts.datc64"];
        dl.load_tables(&files);
        let mut file_list = dl.get_file_list();
        for f in file_list.iter_mut() {
            *f = f.to_lowercase();
        }
        file_list.sort();

        if let Some(hideouts) = dl.dat_files.get("data/balance/hideouts.datc64") {
            let claims = hideouts.get_all_column_claims(Some(&file_list));

            // Offset 16 should be Hash16
            assert!(
                claims.iter().any(|c| c.offset == 16 && matches!(c.column_type, Cell::Scalar(Scalar::Hash16))),
                "Offset 16 should resolve to Hash16"
            );

            // Offset 18 should be File
            assert!(
                claims.iter().any(|c| c.offset == 18 && matches!(c.column_type, Cell::Scalar(Scalar::File))),
                "Offset 18 should resolve to File"
            );

            // Offset 52 should be Array
            assert!(
                claims.iter().any(|c| c.offset == 52 && matches!(c.column_type, Cell::Array(_))),
                "Offset 52 should resolve to Array"
            );

            // Offset 69 should be ForeignRow
            assert!(
                claims
                    .iter()
                    .any(|c| c.offset == 69 && matches!(c.column_type, Cell::Scalar(Scalar::ForeignRow))),
                "Offset 69 should resolve to ForeignRow"
            );

            // Ensure NO overlapping claims exist for File at 18
            let file_claim = claims.iter().find(|c| c.offset == 18).unwrap();
            let overlap = claims
                .iter()
                .any(|c| c.offset >= file_claim.offset && c.offset < file_claim.offset + file_claim.bytes && c.offset != 18);
            assert!(!overlap, "Should have no overlapping claims for File at 18");
        }
    }

    #[test]
    fn test_diagnose_miscobjects() {
        let mut dl = DatLoader::default();
        let mut file_list = dl.get_file_list();
        for f in file_list.iter_mut() {
            *f = f.to_lowercase();
        }

        let target_name = "data/balance/miscobjects.datc64";
        let exists = file_list.iter().any(|f| f == target_name);
        if !exists {
            println!("miscobjects.datc64 not found in file list, skipping test");
            return;
        }

        dl.load_tables(&[target_name]);

        if let Some(dat) = dl.dat_files.get(target_name) {
            println!("Analyzing miscobjects.datc64...");

            // Check raw Hash32 claims
            let c32 = dat.get_column_claims(32, 4, None);
            println!("Raw claims at 32 (4 bytes): {:?}", c32);
            let c40 = dat.get_column_claims(40, 4, None);
            println!("Raw claims at 40 (4 bytes): {:?}", c40);

            // Check raw Hash16 claims
            let c16_32 = dat.get_column_claims(32, 2, None);
            println!("Raw claims at 32 (2 bytes): {:?}", c16_32);
            let c16_34 = dat.get_column_claims(34, 2, None);
            println!("Raw claims at 34 (2 bytes): {:?}", c16_34);

            let c16_40 = dat.get_column_claims(40, 2, None);
            println!("Raw claims at 40 (2 bytes): {:?}", c16_40);
            let c16_42 = dat.get_column_claims(42, 2, None);
            println!("Raw claims at 42 (2 bytes): {:?}", c16_42);

            // Check resolved claims
            let resolved = dat.get_all_column_claims(Some(&file_list));
            let interesting: Vec<_> = resolved
                .iter()
                .filter(|c| (c.offset >= 32 && c.offset < 36) || (c.offset >= 40 && c.offset < 44))
                .collect();
            println!("Resolved claims in range 32-36, 40-44: {:?}", interesting);
        }
    }
}
