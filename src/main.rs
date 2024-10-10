use csv::Reader;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please enter a filename");
        return;
    }

    let lines = match open_csv(&args[1]) {
        Err(err) => panic!("{}", err),
        Ok(lines) => lines,
    };

    if let Err(err) = make_json(lines) {
        panic!("{}", err);
    }
}

fn make_json(records: Vec<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&records)?;

    println!("{}", json);

    Ok(())
}

fn open_csv(filename: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut lines: Vec<HashMap<String, String>> = Vec::new();
    let mut rdr = Reader::from_reader(file);

    for line in rdr.deserialize() {
        let item: HashMap<String, String> = line?;

        lines.push(item)
    }

    Ok(lines)
}
