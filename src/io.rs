use csv::{ReaderBuilder, StringRecord, Writer};
use std::fs::File;

pub fn wrap_from_path(filepath: &str) -> (Vec<String>, Vec<StringRecord>) {
    let rdr = ReaderBuilder::new().delimiter(b'\t').from_path(filepath);

    let mut rdr = match rdr {
        Ok(r) => r,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
        .unwrap();

    // Get headers into a string
    let headers = rdr
        .headers()
        .unwrap()
        .iter()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    (headers, records)
}

pub fn wrap_from_str(data: &str) -> (Vec<String>, Vec<StringRecord>) {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());

    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
        .unwrap();

    // Get headers into a string
    let headers = rdr
        .headers()
        .unwrap()
        .iter()
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    (headers, records)
}

#[allow(dead_code)]
pub fn write_tsv(filepath: &str) -> Writer<File> {
    let wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_path(filepath);

    let wtr = match wtr {
        Ok(r) => r,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    wtr
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
    fn test_parse_tsv() {
        let (_, records) = wrap_from_str(TEST_TSV_STRING);

        assert_eq!(
            records,
            vec![
                vec!["Boston", "United States", "4628910"],
                vec!["Concord", "United States", "42695"],
                vec!["Kilifi", "Kenya", "304349"],
            ]
        );
    }
}
