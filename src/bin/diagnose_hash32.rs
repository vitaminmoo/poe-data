use bytes::Buf;
use poe_data::dat_parser::DatLoader;
use std::collections::HashMap;

fn main() {
    let mut dl = DatLoader::default();
    // Search for miscanimated file
    let files: Vec<String> = dl
        .get_file_list()
        .into_iter()
        .filter(|f| f.to_lowercase().contains("miscanimated.datc64"))
        .collect();

    if files.is_empty() {
        eprintln!("Could not find MiscAnimated table");
        return;
    }

    let file_name = &files[0];
    println!("Analyzing {}", file_name);

    let dat_file = dl.get_table(file_name).expect("Failed to load table");

    let rows = dat_file.rows();
    let num_rows = rows.len();
    let row_len = dat_file.bytes_per_row;

    println!("Rows: {}, Bytes per row: {}", num_rows, row_len);

    // Check every 4-byte window
    println!("Offset | MinVal | MaxVal     | Entropy | Ratio  | Buckets | LSB   MSB   | Odd (L M)     | Note");
    println!("-------|--------|------------|---------|--------|---------|-------------|---------------|-----");
    for offset in 0..row_len.saturating_sub(3) {
        let mut values = Vec::new();
        for row in &rows {
            let val = row.slice(offset..offset + 4).get_u32_le();
            values.push(val);
        }

        let min_val = *values.iter().min().unwrap_or(&0);
        let max_val = *values.iter().max().unwrap_or(&0);

        let mut counts = HashMap::new();
        for &val in &values {
            *counts.entry(val).or_insert(0) += 1;
        }
        let len = values.len() as f64;
        let entropy: f64 = counts.values().fold(0.0, |acc, &count| {
            let p = count as f64 / len;
            acc - p * p.log2()
        });

        // For u32, the space is huge, so max entropy is constrained by row count usually.
        let max_possible = values.len() as f64;
        let max_possible_log2 = max_possible.log2();
        let entropy_ratio = if max_possible_log2 > 0.0 { entropy / max_possible_log2 } else { 0.0 };

        // Buckets of top nibble (16 buckets)
        let mut buckets = [0; 16];
        let mut lsb_set = std::collections::HashSet::new();
        let mut msb_set = std::collections::HashSet::new();

        let mut lsb_odd = 0;
        let mut msb_odd = 0;

        for &val in &values {
            buckets[(val >> 28) as usize] += 1;
            let lsb = (val & 0xFF) as u8;
            let msb = (val >> 24) as u8;
            lsb_set.insert(lsb);
            msb_set.insert(msb);

            if !lsb.is_multiple_of(2) {
                lsb_odd += 1;
            }
            if !msb.is_multiple_of(2) {
                msb_odd += 1;
            }
        }
        let filled_buckets = buckets.iter().filter(|&&x| x > 0).count();
        let lsb_odd_pct = lsb_odd as f64 / values.len() as f64;
        let msb_odd_pct = msb_odd as f64 / values.len() as f64;

        let is_target = offset == 40;
        let marker = if is_target { "***" } else { "   " };

        println!(
            "{:3}    | {:6} | {:10} | {:.4}  | {:.4} | {:2}/16   | L:{:3} M:{:3} | Odd:{:.2} {:.2} | {}",
            offset,
            min_val,
            max_val,
            entropy,
            entropy_ratio,
            filled_buckets,
            lsb_set.len(),
            msb_set.len(),
            lsb_odd_pct,
            msb_odd_pct,
            marker
        );
    }
}
