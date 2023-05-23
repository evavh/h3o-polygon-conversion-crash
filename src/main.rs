use std::fs::File;

use h3o::CellIndex;
use h3o::geom::ToGeo;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    println!("Reading json file with CellIndexes");
    let cell_indexes_path = format!("{CRATE_DIR}/cell_indexes_that_crash.json");
    let cell_indexes_file = File::open(&cell_indexes_path).unwrap();

    let cell_indexes: Vec<CellIndex> =
        serde_json::from_reader(cell_indexes_file).unwrap();

    println!("Trying to convert CellIndexes to Multipolygon...");
    cell_indexes.to_geom(true).unwrap();
}
