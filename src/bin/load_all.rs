use poe_data::dat_parser::DatLoader;

fn main() {
    let mut dl = DatLoader::default();
    dl.load_all_tables();
}
