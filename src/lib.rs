use csv::ReaderBuilder;
use pyo3::prelude::*;
use serde;
use std::time::Instant;

#[derive(serde::Deserialize, Debug)]
struct Organization {
    #[serde(rename(deserialize = "Index"))]
    index: i64,
    #[serde(rename(deserialize = "Organization Id"))]
    organization_id: String,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Website"))]
    website: String,
    #[serde(rename(deserialize = "Country"))]
    country: String,
    #[serde(rename(deserialize = "Description"))]
    description: String,
    #[serde(rename(deserialize = "Founded"))]
    founded: String,
    #[serde(rename(deserialize = "Industry"))]
    industry: String,
    #[serde(rename(deserialize = "Number of employees"))]
    number_of_employess: i64,
}

#[pyfunction]
fn read_csv(file_name: String) {
    // Start
    let start_time = Instant::now();

    let file_name = file_name.as_str();
    let mut builder = ReaderBuilder::new();
    // Customize ReaderBuilder, builder, options
    builder.double_quote(false).comment(Some(b'-'));
    let result = builder.from_path(file_name);

    // Handle failure on read operation
    if result.is_err() {
        println!("Failed to read CSV. File path may not exist or you don't have permissions.");
        std::process::exit(9);
    }

    // Perform file read
    let mut reader = result.unwrap();

    println!(
        r#"
    Index   |   Organization Id  |       Name       |           Website         |   Country |       Description      |   Founded |   Industry    |   Number of employees
    "#
    );

    for record in reader.deserialize() {
        // Avoid move operation | losing access to the record Result
        // by story the unwrapped result in a variable
        let data_value: Organization = record.unwrap();
        println!(
            r#"
        {} {} {} {} {} {} {} {} {}
        "#,
            data_value.index,
            data_value.organization_id,
            data_value.name,
            data_value.website,
            data_value.country,
            data_value.description,
            data_value.founded,
            data_value.industry,
            data_value.number_of_employess
        );
    }

    // End
    let end_time = Instant::now();
    let time_taken = end_time.duration_since(start_time);
    println!("Execution time: {:?}", time_taken);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusty_csv(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;
    Ok(())
}
