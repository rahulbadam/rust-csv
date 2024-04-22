use csv;
use std::error::Error;

fn read_csv_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for res in reader.records() {
        let rec = res?;
        println!("{:?}", rec);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_csv_file("./addresses.csv") {
        eprint!("{}", e)
    }
}
