use poe_data::dat_parser::DatLoader;

fn main() {
    let mut dl = DatLoader::default();
    for (name, dat_file) in dl.dat_files.iter_mut() {
        //let table: &mut DatFile = dl.get_table(table).unwrap();
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
