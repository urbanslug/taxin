mod cli;
mod distance;
mod io;
mod types;

use types::CoverageVector;

fn main() {
    let fp: String = cli::start();

    println!("Parsing coverage vector: {}", fp);
    let cov = CoverageVector::from_file(&fp[..]);
    let m: Vec<Vec<f64>> = cov.extract_data_matrix();

    println!("Calculating pairwise distances");
    let _distance_matrix: Vec<Vec<f64>> = distance::eucledian(&m);

    // println!("{:?}", distance_matrix);

    // let mut writer = write_tsv(Path::join(dataset_dir, "coverage_10_samples_taxin_output.tsv"));

    // TODO: move this into write tsv
    // data.iter().for_each(|datum| writer.write_record(datum).unwrap());
}
