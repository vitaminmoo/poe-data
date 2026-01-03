use bytes::Buf;
use poe_data::dat_parser::DatLoader;

fn main() {
    let mut loader = DatLoader::default();
    let name = "data/hellscapeareapacks.datc64";
    let dat_file = loader.get_table(name).expect("Failed to load table");

    println!("Table: HellscapeAreaPacks");
    println!("Row count: {}", dat_file.row_count);

    let rows_iter = dat_file.column_rows_iter(16, 16);

    for (i, mut row) in rows_iter.enumerate() {
        let count = row.get_u64_le();
        let offset = row.get_u64_le();

        println!("Row {}: Count={} Offset={}", i, count, offset);
    }
}
