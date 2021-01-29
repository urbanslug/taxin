mod cli;
mod io;
mod types;

use types::CoverageVector;

fn main() {
    let fp: String = cli::start();

    println!("Parsing coverage vector: {}", fp);
    let _cov = CoverageVector::from_file(&fp[..]);

    // let mut writer = write_tsv(Path::join(dataset_dir, "coverage_10_samples_taxin_output.tsv"));

    // TODO: move this into write tsv
    // data.iter().for_each(|datum| writer.write_record(datum).unwrap());
}
