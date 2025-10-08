use std::path::Path;

use test_case::test_case;

use crate::input::Tx;

#[test_case("deposits")]
#[test_case("withdrawals")]
#[test_case("disputes")]
#[test_case("resolves")]
#[test_case("chargebacks")]
fn parse_csv(case_name: &str) {
    let input_file = Path::new(file!())
        .parent()
        .expect("file!().parent")
        .join("cases")
        .join(format!("{}.csv", case_name));
    let mut output = vec![];
    let csv_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(input_file)
        .expect("CsvReader::from_path");
    for decode_result in csv_reader.into_deserialize::<Tx>() {
        output.push(decode_result.map_err(|e| e.to_string()));
    }
    insta::with_settings!({
        snapshot_path => "cases",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_debug_snapshot!(case_name, output);
    });
}
