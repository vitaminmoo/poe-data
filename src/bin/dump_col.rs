use bytes::Buf;
use poe_data::dat_parser::DatLoader;

fn main() {
    let mut loader = DatLoader::default();
    let name = "data/balance/AlternatePassiveAdditions.datc64";
    let dat_file = loader.get_table(name).expect("Failed to load table");

    println!("Table: AlternatePassiveAdditions");
    println!("Row count: {}", dat_file.row_count);

    let rows_iter = dat_file.column_rows_iter(0, 8);
    let mut seen = std::collections::HashMap::new();

    for (i, mut row) in rows_iter.enumerate() {
        let val = row.get_u64_le();
        if val == 0xFEFEFEFEFEFEFEFE {
            continue;
        }

        if val < 8 || (val as usize) >= dat_file.vdata.len() {
            println!("Row {}: Invalid offset {}", i, val);
            continue;
        }

        match dat_file.string_from_offset_if_valid(val as usize) {
            Ok(s) => {
                if s.len() < 2 {
                    println!("Row {}: Short string '{}' (len {})", i, s, s.len());
                }
                if !s.is_ascii() {
                    println!("Row {}: Non-ASCII string '{}'", i, s);
                }
                if let Some(&prev_offset) = seen.get(&s) {
                    if prev_offset != val {
                        println!("Row {}: Duplicate string '{}' at different offset {} (prev {})", i, s, val, prev_offset);
                    }
                } else {
                    seen.insert(s, val);
                }
            }
            Err(e) => println!("Row {}: Invalid string at {}: {}", i, val, e),
        }
    }
}
