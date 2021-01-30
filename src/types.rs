use csv::StringRecord;

use crate::io;

// where the data starts
static FIRST_NODE: &str = "node.1";

#[derive(Debug, PartialEq)]
pub struct CoverageVector {
    headers: Vec<String>,
    data: Vec<Vec<u64>>,
    metadata: Vec<Vec<String>>,
}

// TODO: use generics or something cleaner
impl CoverageVector {
    pub fn from_file(filepath: &str) -> CoverageVector {
        gen(io::wrap_from_path(filepath))
    }

    #[allow(dead_code)]
    fn from_str(data: &str) -> CoverageVector {
        gen(io::wrap_from_str(data))
    }

    // get the data field as a matrix of Vec<Vec<f64>>
    pub fn extract_data_matrix(&self) -> Vec<Vec<f64>> {

        // is this efficient?
        let vec_to_f64 = | v: &Vec<u64> | {
            v.iter().map(|x| *x as f64).collect::<Vec<f64>>()
        };

        let data = &self.data;

        data.iter()
            .map(vec_to_f64)
            .collect()
    }
}

fn gen((headers, records): (Vec<String>, Vec<StringRecord>)) -> CoverageVector {
    // get the index of where the data starts
    let counter: usize = headers.iter().position(|x| x == FIRST_NODE).unwrap();

    // TODO: use an iterator
    let mut metadata: Vec<Vec<String>> = Vec::new();
    let mut data: Vec<Vec<u64>> = Vec::new();
    for record in &records {
        let metadatum: Vec<String> = record
            .iter()
            .take(counter)
            .map(|value| String::from(value))
            .collect();
        metadata.push(metadatum);

        let datum: Vec<u64> = record
            .iter()
            .skip(counter)
            .map(|value| value.parse::<u64>().unwrap())
            .collect();
        data.push(datum);
    }

    CoverageVector {
        headers,
        data,
        metadata,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_TSV_STRING: &str = "\
    city\tcountry\tnode.1
Boston\tUnited States\t4628910
Concord\tUnited States\t42695
Kilifi\tKenya\t304349
";

    #[test]
    fn test_create_coverage_vector() {
        let cov = CoverageVector::from_str(TEST_TSV_STRING);

        // TODO: clean up this test
        let p = |k: Vec<&str>| {
            k.into_iter()
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        };

        let headers: Vec<String> = p(vec!["city", "country", "node.1"]);

        let metadata: Vec<Vec<String>> = vec![
            p(vec!["Boston", "United States"]),
            p(vec!["Concord", "United States"]),
            p(vec!["Kilifi", "Kenya"]),
        ];
        let data: Vec<Vec<u64>> = vec![vec![4628910], vec![42695], vec![304349]];

        assert_eq!(
            cov,
            CoverageVector {
                headers,
                data,
                metadata,
            }
        );
    }
}
